import Surreal from 'surrealdb.js';
import store from "./index";

export default class DatabaseStore {
    private root: typeof store
    private db: Surreal | undefined

    constructor(root: typeof store) {
        this.root = root
    }

    async setup() {
        if (this.root.configStore.config.remoteDatabaseLocation != null) {
            await this.connectToDB(this.root.configStore.config.remoteDatabaseLocation)
        } else if (this.root.configStore.isTauri()) {
            await this.root.configStore.tauri?.invoke("setup_database")
        }
    }

    async query(statement: string, vars?: Record<string, unknown>): Promise<any> {
        if (this.db !== undefined) {
            // TODO: This will result in error if database connection is reset (f.e. database is restarted on remote)
            return this.db.query(statement, vars)
        } else if (this.root.configStore.isTauri()) {
            return this.root.configStore.tauri?.invoke("query_database", {
                "query": statement,
                "vars": vars
            })!
        }

        throw new DatabaseError("No database is running")
    }

    async check(): Promise<boolean> {
        if (this.db !== undefined) {
            if (true) {
                return true
            } else {
                return false
            }
        } else if (this.root.configStore.isTauri()) {
            return this.root.configStore.tauri?.invoke("check_database")!
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

export class DatabaseError {
    message: string

    constructor(message: string) {
        this.message = message
    }
}