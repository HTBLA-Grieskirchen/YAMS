import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import Address, {AddressResponse} from "../model/address";
import {ano, no} from "../util/consts";

export default class AddressStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    indexedAddresses: Map<string, Address>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexedAddresses = observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,

            addresses: computed.struct,
        })
    }

    get addresses(): Address[] {
        return Array.from(this.indexedAddresses.values())
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM address ORDER BY id")

        this.registerSyncData()
    }

    close() {
        const [_r, _u, clean] = this.dataLive

        clean()
    }

    private registerSyncData() {
        autorun(() => {
            this.syncData()
        })
    }

    private syncData() {
        const [result, _u, _c] = this.dataLive
        if (!(result.length > 0 && result[0].result)) return

        runInAction(() => {
            // Update addresses
            const addressIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = AddressResponse.from(item)
                    if (!response) return

                    let address = this.indexedAddresses.get(response.data.id)
                    if (address !== undefined) {
                        response.applyOn(address)
                    } else {
                        address = response.intoObject()
                        if (!address) return

                        this.indexedAddresses.set(response.data.id, address)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedAddresses.keys()).filter((id) => !addressIDs.has(id)).forEach((id) => this.indexedAddresses.delete(id))
        })
    }
}