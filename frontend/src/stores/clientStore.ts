import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import {ano, no} from "../util/consts";
import Client, {ClientResponse} from "../model/client";
import RelationStore from "./client/relationStore";

export default class ClientStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    relationStore: RelationStore

    indexedClients: Map<string, Client>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexedClients = observable.map()
        this.relationStore = new RelationStore(root)

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,
            clients: computed.struct
        })
    }

    get clients(): Client[] {
        return Array.from(this.indexedClients.values())
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await this.root.addressStore.refresh()
        await refresh()
        await this.root.clientStore.relationStore.refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM client ORDER BY id")
        this.registerSyncData()

        await this.relationStore.setup()
    }

    close() {
        this.relationStore.close()
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
            // Update clients
            const clientIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = ClientResponse.from(item)
                    if (!response) return

                    let client = this.indexedClients.get(response.data.id)
                    if (client !== undefined) {
                        response.applyOn(client)
                    } else {
                        client = response.intoObject()
                        if (!client) return

                        this.indexedClients.set(response.data.id, client)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedClients.keys()).filter((id) => !clientIDs.has(id)).forEach((id) => this.indexedClients.delete(id))
        })
    }
}