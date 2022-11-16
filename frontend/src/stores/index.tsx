import {createContext, ReactNode, useContext} from "react";
import ConfigStore from "./configStore";
import NotificationStore from "./notificationStore";
import DialogStore from "./dialogStore";

class Store {
    configStore: ConfigStore
    notificationStore: NotificationStore
    dialogStore: DialogStore

    constructor() {
        this.configStore = new ConfigStore(this)
        this.notificationStore = new NotificationStore(this)
        this.dialogStore = new DialogStore(this)
    }

    async init() {
        await this.configStore.init()
    }
}

// Create and setup static store
const store = new Store()
export const initStore = store.init()

// Provide react integration for accessing
const StoreContext = createContext(store)

// The provider component surrounding app
export function StoreProvider({children}: { children: ReactNode }) {
    return <StoreContext.Provider value={store}>{children}</StoreContext.Provider>
}

// Store usage
export const useStore = () => {
    return useContext<typeof store>(StoreContext)
}

// Provide raw access
export default store