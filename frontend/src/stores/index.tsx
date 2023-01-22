import {createContext, ReactNode, useContext} from "react";
import NotificationStore from "./notificationStore";
import DialogStore from "./dialogStore";
import AddressStore from "./addressStore";
import ClientStore from "./clientStore";
import SettingsStore from "./settingsStore";

class Store {
    settingsStore: SettingsStore

    notificationStore: NotificationStore
    dialogStore: DialogStore

    addressStore: AddressStore
    clientStore: ClientStore

    constructor() {
        this.settingsStore = new SettingsStore(this)

        this.notificationStore = new NotificationStore(this)
        this.dialogStore = new DialogStore(this)

        this.addressStore = new AddressStore(this)
        this.clientStore = new ClientStore(this)
    }

    async setup() {
        await this.settingsStore.setup()

        await this.addressStore.setup()
        await this.clientStore.setup()
    }
}

// Create and setup static store
const store = new Store()
export const setupStore = store.setup().then(() => {
    console.log("Store setup finished")
})

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