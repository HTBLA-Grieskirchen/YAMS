import store from "../index";
import { action, autorun, computed, makeAutoObservable, observable, runInAction } from "mobx";
import { live } from "../../libs/database";
import { ano, no } from "../../util/consts";
import ClientRelation, { ClientRelationResponse } from "../../model/relation";

export default class RelationStore {
    indexed: Map<string, ClientRelation>
    indexedByFrom: Map<string, ClientRelation[]>
    indexedByDest: Map<string, ClientRelation[]>
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexed = observable.map()
        this.indexedByFrom = observable.map()
        this.indexedByDest = observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,
            relations: computed.struct
        })
    }

    get relations(): ClientRelation[] {
        return Array.from(this.indexed.values())
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await this.root.addressStore.refresh()
        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM client_relation ORDER BY id")
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
            this.indexedByDest.clear()
            this.indexedByFrom.clear()

            // Update relations
            const relationIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = ClientRelationResponse.from(item)
                    if (!response) return

                    let relation = this.indexed.get(response.data.id)
                    if (relation !== undefined) {
                        response.applyOn(relation)
                    } else {
                        relation = response.intoObject()
                        if (!relation) return

                        this.indexed.set(response.data.id, relation)
                    }

                    let indexFromArray = this.indexedByFrom.get(relation.from.record.join()) ?? (() => {
                        const arr = observable.array()
                        this.indexedByFrom.set(relation!!.from.record.join(), arr)
                        return arr
                    })()
                    indexFromArray.push(relation)
                    let indexDestArray = this.indexedByDest.get(relation.dest.record.join()) ?? (() => {
                        const arr = observable.array()
                        this.indexedByDest.set(relation!!.dest.record.join(), arr)
                        return arr
                    })()
                    indexDestArray.push(relation)

                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexed.keys()).filter((id) => !relationIDs.has(id)).forEach((id) => this.indexed.delete(id))
        })
    }
}
