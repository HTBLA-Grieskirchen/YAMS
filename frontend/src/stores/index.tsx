import {createContext, ReactNode, useContext} from "react";
import NotificationStore from "./notificationStore";
import DialogStore from "./dialogStore";
import AddressStore from "./addressStore";
import ClientStore from "./clientStore";
import PurchaseStore from "./purchaseStore";

class Store {
    notificationStore: NotificationStore
    dialogStore: DialogStore

    addressStore: AddressStore
    clientStore: ClientStore
    purchaseStore: PurchaseStore

    constructor() {
        this.notificationStore = new NotificationStore(this)
        this.dialogStore = new DialogStore(this)

        this.addressStore = new AddressStore(this)
        this.clientStore = new ClientStore(this)
        this.purchaseStore = new PurchaseStore(this)
    }

    async setup() {
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