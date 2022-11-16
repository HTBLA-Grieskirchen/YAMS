import store, {initStore} from "../stores";
import {tauri} from "@tauri-apps/api";
import Surreal, {Result} from "surrealdb.js";
import {observable, runInAction} from "mobx";
import {useEffect, useState} from "react";
import Live from "surrealdb.js/types/classes/live";
import {ano, no} from "../util/consts";

class DatabaseConnection {
    private db: Surreal | undefined
    private tauri: typeof tauri | undefined
    private setupPromise: Promise<void> | undefined


    constructor() {
        this.setupPromise = undefined
    }

    async setup() {
        await initStore
        if (store.configStore.config.remoteDatabaseLocation !== null) {
            await this.connectToDB(store.configStore.config.remoteDatabaseLocation)
        } else if (store.configStore.isTauri()) {
            this.tauri = store.configStore.tauri ?? undefined
            await store.configStore.tauri?.invoke("setup_database")
        }
    }

    async setupCompleted() {
        // TODO: Move store config to own independent file
        if (this.setupPromise) {
            await this.setupPromise
        } else {
            this.setupPromise = this.setup()
        }
    }

    async query(statement: string, vars?: Record<string, unknown>): Promise<Result<any>[]> {
        await this.setupCompleted()
        if (this.db) {
            // TODO: This will result in error if database connection is reset (f.e. database is restarted on remote)
            return this.db.query(statement, vars)
        } else if (this.tauri) {
            return this.tauri.invoke("query_database", {
                "query": statement,
                "vars": vars
            })!
        }

        throw new DatabaseError("No database is running")
    }

    async live(statement: string, vars?: Record<string, unknown>): Promise<Live | undefined> {
        await this.setupCompleted()
        if (this.db && false) {
            // TODO: This will result in error if database connection is reset (f.e. database is restarted on remote)
            return this.db?.sync(statement, vars)
        }

        return undefined
    }

    async check(): Promise<boolean> {
        if (this.db) {
            return true
        } else if (this.tauri) {
            return this.tauri.invoke("check_database")!
        }
        return false
    }

    async close() {
        if (this.db !== undefined) {
            const db = this.db
            this.db = undefined
            await db.close()
        }
    }

    isEmbedded(): boolean {
        return !this.isRemote()
    }

    isRemote(): boolean {
        return !this.tauri
    }

    private async connectToDB(url: string) {
        const db = new Surreal(url)
        await db.signin({
            user: "root",
            pass: "root"
        })
        await db.use("yams", "yams")
        this.db = db
    }
}

const db = new DatabaseConnection()

export class DatabaseError {
    message: string

    constructor(message: string) {
        this.message = message
    }
}

export type LiveRefresher = () => Promise<void>
// TODO: Remove once sync is implemented
export type LiveCleaner = () => void

export async function query(
    statement: string,
    vars?: Record<string, unknown>
): Promise<Result<any>[]> {
    return db.query(statement, vars)
}

/**
 * This provides the result from the given statement in reactive style. The result can be refreshed using the second
 * return value. Once the result is not used anymore, the connection can be closed with the third return value.
 *
 * **Attention**: Be careful when accessing the result, as invalid statements (like `result[0]` in an empty result array)
 * can cause unexpected behaviour (conflicting with MobX)
 *
 * @param statement – Specifies the SurrealQL statements.
 * @param vars – Assigns variables which can be used in the query.
 */
export async function live(
    statement: string,
    vars?: Record<string, unknown>
): Promise<[Result<any>[], LiveRefresher, LiveCleaner]> {
    const live = await db.live(statement, vars)
    if (live) {
        // TODO: Provide live support once sync is implemented
        return [[] as Result<any>[], ano, no]
    }

    const getObservableResult = async () => {
        const staticResult = await query(statement, vars)
        return staticResult.map(item => observable(item))
    }

    const result: Result<any>[] = observable(await getObservableResult())

    const updateResult = async () => {
        const observableResult = await getObservableResult()

        runInAction(() => {
            // TODO: Maybe support diff changes via id as key (benefit is questionable?)
            result.length = 0
            result.push(...observableResult)
        })
    }

    // TODO: Remove once sync is implemented
    let interval: NodeJS.Timer | undefined = undefined
    if (db.isRemote()) {
        interval = setInterval(updateResult, 1250)
    }

    return [result, updateResult, () => clearInterval(interval)]
}

interface QueryResult<T> {
    response: Result<T>[],
    loading?: never
}

interface LoadingResult {
    response?: never,
    loading: boolean
}

export type RequestedResult<T> = QueryResult<T> | LoadingResult


/**
 * @param statement – Specifies the SurrealQL statements.
 * @param vars – Assigns variables which can be used in the query.
 */
export function useQuery(
    statement: string,
    vars?: Record<string, unknown>
): RequestedResult<any> {
    const [result, setResult] = useState<RequestedResult<any>>({loading: true})

    useEffect(() => {
        query(statement, vars).then(response => setResult({response}))
    }, [])

    return result
}

/**
 * This provides the result from the given statement in reactive style. The result can be refreshed using the second
 * return value.
 *
 * **Attention**: Be careful when accessing the result, as invalid statements (like `result[0]` in an empty result array)
 * can cause unexpected behaviour (conflicting with MobX)
 *
 * @param statement – Specifies the SurrealQL statements.
 * @param vars – Assigns variables which can be used in the query.
 */
export function useLive(
    statement: string,
    vars?: Record<string, unknown>
): [RequestedResult<any>, LiveRefresher] {
    const [result, setResult] = useState<RequestedResult<any>>({loading: true})
    const [update, setUpdate] = useState<LiveRefresher>(() => ano)

    const init = async () => {
        const [response, refresh, clear] = await live(statement, vars)
        setResult({response})
        setUpdate(() => refresh)
        return clear
    }

    useEffect(() => {
        const initPromise = init()

        return () => {
            initPromise.then((clear) => clear())
        }
    }, [])

    return [result, update]
}
