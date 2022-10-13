import DatabaseStore from "./databaseStore";

class RootStore {
    dbStore: any = new DatabaseStore()

    constructor() {
    }

    init(url: string) {
        this.dbStore.connectToDB(url)
    }
}

export default RootStore