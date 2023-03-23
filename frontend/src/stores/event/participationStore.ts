import store from "../index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../../libs/database";
import {ano, no} from "../../util/consts";
import EventParticipation, {EventParticipationResponse} from "../../model/participation";

export default class ParticipationStore {
    indexed: Map<string, EventParticipation>
    indexedByFrom: Map<string, EventParticipation[]>
    indexedByDest: Map<string, EventParticipation[]>
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
            participations: computed.struct
        })
    }

    get participations(): EventParticipation[] {
        return Array.from(this.indexed.values())
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await this.root.clientStore.refresh()
        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM participated_in ORDER BY id")
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
            const participationIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = EventParticipationResponse.from(item)
                    if (!response) return

                    let participation = this.indexed.get(response.data.id)
                    if (participation !== undefined) {
                        response.applyOn(participation)
                    } else {
                        participation = response.intoObject()
                        if (!participation) return

                        this.indexed.set(response.data.id, participation)
                    }

                    let indexFromArray = this.indexedByFrom.get(participation.from.record.join()) ?? (() => {
                        const arr = observable.array()
                        this.indexedByFrom.set(participation!!.from.record.join(), arr)
                        return arr
                    })()
                    indexFromArray.push(participation)
                    let indexDestArray = this.indexedByDest.get(participation.dest.record.join()) ?? (() => {
                        const arr = observable.array()
                        this.indexedByDest.set(participation!!.dest.record.join(), arr)
                        return arr
                    })()
                    indexDestArray.push(participation)

                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexed.keys()).filter((id) => !participationIDs.has(id)).forEach((id) => this.indexed.delete(id))
        })
    }
}
