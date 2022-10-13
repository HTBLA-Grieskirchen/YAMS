import Surreal from 'surrealdb.js';

class DatabaseStore {
    db: Surreal | undefined = undefined

    constructor() {
    }

    async connectToDB(url: string) {
        this.db = new Surreal(url)
        await this.db.use('yams', 'yams')
    }

    async query(statement: string, vars: any): Promise<any> {
        let result = await this.db?.query(statement, vars)
        return result
    }

    async wait() {
        await this.db?.wait()
    }

    async close() {
        this.db?.close()
    }
}

export default DatabaseStore