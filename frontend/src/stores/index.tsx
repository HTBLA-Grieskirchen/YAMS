import {createContext, ReactNode, useContext} from "react";
import NotificationStore from "./notificationStore";
import DialogStore from "./dialogStore";
import AddressStore from "./addressStore";
import ClientStore from "./clientStore";
import TreatmentStore from "./treatmentStore";
import TreatmentAppointmentStore from "./treatmentAppointmentStore";
import ClientFileStore from "./clientFileStore";

class Store {
    notificationStore: NotificationStore
    dialogStore: DialogStore
    treatmentStore:TreatmentStore
    addressStore: AddressStore
    clientStore: ClientStore
    treatmentAppointmentStore: TreatmentAppointmentStore
    clientFileStore:ClientFileStore

    constructor() {
        this.notificationStore = new NotificationStore(this)
        this.dialogStore = new DialogStore(this)
        this.treatmentStore=new TreatmentStore(this)
        this.addressStore = new AddressStore(this)
        this.clientStore = new ClientStore(this)
        this.treatmentAppointmentStore=new TreatmentAppointmentStore(this)
        this.clientFileStore=new ClientFileStore(this)
    }

    async setup() {
        await this.addressStore.setup()
        await this.clientStore.setup()
        await this.treatmentStore.setup()
        await this.treatmentAppointmentStore.setup()
        await this.clientFileStore.setup()
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