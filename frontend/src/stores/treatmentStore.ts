import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import Address, {AddressResponse} from "../model/address";
import City, {CityResponse} from "../model/city";
import Country, {CountryResponse} from "../model/country";
import {ano, no} from "../util/consts";
import Treatment_type, {TreatmentTypeResponse} from "../model/treatment_type";
import Treatment, {TreatmentResponse} from "../model/treatment";

export default class TreatmentStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    indexedTypes: Map<string, Treatment_type>
    indexedTreatments: Map<string, Treatment>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexedTypes=observable.map()
        this.indexedTreatments=observable.map()
        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,

            types: computed.struct,
            treatments: computed.struct

        })
    }

    get types(): Treatment_type[] {
        return Array.from(this.indexedTypes.values())
    }
    get treatments(): Treatment[] {
        return Array.from(this.indexedTreatments.values())
    }


    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM treatment, treatment_type ORDER BY id")

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
            // Update treatment_types
            const typeIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = TreatmentTypeResponse.from(item)
                    if (!response) return

                    let type = this.indexedTypes.get(response.data.id)
                    if (type !== undefined) {
                        response.applyOn(type)
                    } else {
                        type = response.intoObject()
                        if (!type) return

                        this.indexedTypes.set(response.data.id,type)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedTypes.keys()).filter((id) => !typeIDs.has(id)).forEach((id) => this.indexedTypes.delete(id))


            // Update treatments
            const treatmentIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = TreatmentResponse.from(item)
                    if (!response) return

                    let treatment = this.indexedTreatments.get(response.data.id)
                    if (treatment !== undefined) {
                        response.applyOn(treatment)
                    } else {
                        treatment = response.intoObject()
                        if (!treatment) return

                        this.indexedTreatments.set(response.data.id, treatment)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedTreatments.keys()).filter((id) => !treatmentIDs.has(id)).forEach((id) => this.indexedTreatments.delete(id))
        })
    }
}