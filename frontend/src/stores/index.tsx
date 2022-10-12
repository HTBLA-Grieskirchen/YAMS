import {createContext, useContext} from "react";
import RootStore from "./rootStore";

const store = new RootStore()

export const StoreContext = createContext(store)

export const useStore = () => {
    return useContext<typeof store>(StoreContext)
}

export default store