import {createContext, ReactNode, useContext} from "react";
import ConfigStore from "./configStore";

class Store {
    configStore: ConfigStore

    constructor() {
        this.configStore = new ConfigStore(this)
    }

    async setup() {
        await this.configStore.setup()
    }
}

// Create and setup static store
const store = new Store()
export const setupStore = store.setup()

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