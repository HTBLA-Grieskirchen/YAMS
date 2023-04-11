import {autorun, runInAction, set, toJS} from 'mobx'

export default function (_this: any, storeName = 'store') {
    let firstRun = true

    // will run on change
    autorun(() => {
        if (typeof window === "undefined") return

        // on load check if there's an existing store on
        // localStorage and extend the store
        if (firstRun) {
            firstRun = false
            const existingStore = window.localStorage.getItem(storeName)

            if (existingStore) {
                runInAction(() => set(_this, JSON.parse(existingStore)))
            }
        }

        // from then on serialize and save to localStorage
        const serializedThis = toJS(_this)
        window.localStorage.setItem(
            storeName,
            JSON.stringify(serializedThis)
        )
    })
}