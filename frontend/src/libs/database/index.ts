import Surreal, { type QueryResult } from "surrealdb";
import {observable, runInAction} from "mobx";
import {useEffect, useState} from "react";
import {ano, no} from "../../util/consts";
import getConfig, {tauri, TauriType} from "../../config";

class DatabaseConnection {
    private db: Surreal | undefined
    private tauri: TauriType | undefined
    private setupPromise: Promise<void> | undefined


    constructor() {
        this.setupPromise = undefined
    }

    async setup() {
        const config = await getConfig()
        if (config.remoteDatabaseLocation !== null) {
            await this.connectToDB(config.remoteDatabaseLocation)
        } else if (tauri) {
            this.tauri = tauri
            await tauri?.invoke("setup_database")
        }
    }

    async setupCompleted() {
        if (this.setupPromise) {
            await this.setupPromise
        } else {
            this.setupPromise = this.setup().then(() => {
                console.log("Database connection established")
            })
        }
    }

    async query(statement: string, vars?: Record<string, unknown>): Promise<QueryResult<any>[]> {
        await this.setupCompleted()
        if (this.db) {
            // TODO: This will result in error if database connection is reset (f.e. database is restarted on remote)
            return this.db.queryRaw(statement, vars)
        } else if (this.tauri) {
            return this.tauri.invoke("query_database", {
                "query": statement,
                "vars": vars
            })!
        }

        throw new DatabaseError("No database is running")
    }

    async live(statement: string, vars?: Record<string, unknown>): Promise<any | undefined> {
        await this.setupCompleted()
        if (this.db && false) {
            // TODO: Provide live support once sync is implemented
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
        const db = new Surreal()
        await db.connect(url)
        await db.signin({
            username: "root",
            password: "root"
        })
        await db.use({
            namespace: "yams",
            database: "yams"
        })
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

export interface CompatibilityResult<T> {
    result?: T;
    error?: Error;
}

export type Result<T> = CompatibilityResult<T>;

export async function query(
    statement: string,
    vars?: Record<string, unknown>
): Promise<CompatibilityResult<any>[]> {
    const response = await db.query(statement, vars)
    return response.map(r => {
        if (r.status === "OK") {
            return { result: r.result, error: undefined }
        } else {
            return { result: undefined, error: new Error(r.result as string) }
        }
    })
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
): Promise<[CompatibilityResult<any>[], LiveRefresher, LiveCleaner]> {
    const liveResult = await db.live(statement, vars)
    if (liveResult) {
        // TODO: Provide live support once sync is implemented
        return [[] as CompatibilityResult<any>[], ano, no]
    }

    const getObservableResult = async () => {
        const staticResult = await query(statement, vars)
        return staticResult.map(item => observable(item))
    }

    const result: CompatibilityResult<any>[] = observable(await getObservableResult())

    const updateResult = async () => {
        const observableResult = await getObservableResult()

        runInAction(() => {
            result.length = 0
            result.push(...observableResult)
        })
    }

    // TODO: Remove once sync is implemented
    let interval: any = undefined
    if (db.isRemote()) {
        interval = setInterval(updateResult, 1250)
    }

    return [result, updateResult, () => clearInterval(interval)]
}

interface QueryResultState<T> {
    response: CompatibilityResult<T>[],
    loading?: never
}

interface LoadingResult {
    response?: never,
    loading: boolean
}

export type RequestedResult<T> = QueryResultState<T> | LoadingResult


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
            initPromise.then((clear) => clear && clear())
        }
    }, [])

    return [result, update]
}
