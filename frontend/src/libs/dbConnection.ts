import store, {setupStore} from "../stores";
import {tauri} from "@tauri-apps/api";
import Surreal, {Result} from "surrealdb.js";
import {observable, runInAction} from "mobx";
import {useEffect} from "react";
import {useLocalObservable} from "mobx-react";

class DatabaseConnection {
    private db: Surreal | undefined
    private tauri: typeof tauri | undefined
    private readonly setupPromise: Promise<void>


    constructor() {
        this.setupPromise = this.setup()
    }

    async setup() {
        await setupStore
        if (store.configStore.config.remoteDatabaseLocation !== null) {
            await this.connectToDB(store.configStore.config.remoteDatabaseLocation)
        } else if (store.configStore.isTauri()) {
            this.tauri = store.configStore.tauri ?? undefined
            await store.configStore.tauri?.invoke("setup_database")
        }
    }

    async setupCompleted() {
        await this.setupPromise
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

export async function query(
    statement: string,
    vars?: Record<string, unknown>
): Promise<Result<any>[]> {
    return db.query(statement, vars)
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
 * @param refreshInterval - How often the result should be updated.
 */
export function useQuery(
    statement: string,
    vars?: Record<string, unknown>,
    refreshInterval?: number
): [Result<any>[], () => void] {
    const result = useLocalObservable<Result<any>[]>(() => [])

    const updateResult = async () => {
        const queryResult = await query(statement, vars)

        runInAction(() => {
            result.length = 0
            for (const entry of queryResult) {
                result.push(observable(entry))
            }
        })
    }

    useEffect(() => {
        updateResult().then()
        if (refreshInterval) {
            let interval = setInterval(updateResult, refreshInterval)
            return () => clearInterval(interval)
        }
    }, [])

    return [result, updateResult]
}