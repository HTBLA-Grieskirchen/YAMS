import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import TreatmentAppointment, {TreatmentAppointmentResponse} from "../model/treatmentAppointment";
import Treatment, {TreatmentResponse} from "../model/treatment";
import Treatment_type, {Treatment_typeResponse} from "../model/treatment_type";
import {ano, no} from "../util/consts";

export default class TreatmentAppointmentStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    indexedTreatmentAppointments: Map<string, TreatmentAppointment>
    indexedTreatments: Map<string, Treatment>
    indexedTreatment_types: Map<string, Treatment_type>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexedTreatmentAppointments = observable.map()
        this.indexedTreatments = observable.map()
        this.indexedTreatment_types = observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,

            treatmentAppointments: computed.struct,
            treatment_types: computed.struct,
            treatments: computed.struct,
        })
    }

    get treatmentAppointments(): TreatmentAppointment[] {
        return Array.from(this.indexedTreatmentAppointments.values())
    }

    get treatment_types(): Treatment_type[] {
        return Array.from(this.indexedTreatment_types.values())
    }

    get treatments(): Treatment[] {
        return Array.from(this.indexedTreatments.values())
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM treatmentAppointment, treatment, treatment_type ORDER BY id")

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
            const treatment_typeIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = Treatment_typeResponse.from(item)
                    if (!response) return

                    let treatment_type = this.indexedTreatment_types.get(response.data.id)
                    if (treatment_type !== undefined) {
                        response.applyOn(treatment_type)
                    } else {
                        treatment_type = response.intoObject()
                        if (!treatment_type) return

                        this.indexedTreatment_types.set(response.data.id, treatment_type)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedTreatment_types.keys()).filter((id) => !treatment_typeIDs.has(id)).forEach((id) => this.indexedTreatment_types.delete(id))

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

            // Update treatmentAppointments
            const treatmentAppointmentIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = TreatmentAppointmentResponse.from(item)
                    if (!response) return

                    let treatmentAppointment = this.indexedTreatmentAppointments.get(response.data.id)
                    if (treatmentAppointment !== undefined) {
                        response.applyOn(treatmentAppointment)
                    } else {
                        treatmentAppointment = response.intoObject()
                        if (!treatmentAppointment) return

                        this.indexedTreatmentAppointments.set(response.data.id, treatmentAppointment)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedTreatmentAppointments.keys()).filter((id) => !treatmentAppointmentIDs.has(id)).forEach((id) => this.indexedTreatmentAppointments.delete(id))
        })
    }
}