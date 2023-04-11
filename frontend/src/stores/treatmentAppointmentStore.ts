import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import Address, {AddressResponse} from "../model/address";
import City, {CityResponse} from "../model/city";
import Country, {CountryResponse} from "../model/country";
import {ano, no} from "../util/consts";
import Treatment_type, {TreatmentTypeResponse} from "../model/treatment_type";
import Treatment, {TreatmentResponse} from "../model/treatment";
import Treatment_Appointment, {TreatmentAppointmentResponse} from "../model/treatment_appointment";

export default class TreatmentAppointmentStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    indexedTreatmentAppointments: Map<string, Treatment_Appointment>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]


        this.indexedTreatmentAppointments=observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,


            treatmentAppointments: computed.struct

        })
    }


    get treatmentAppointments(): Treatment_Appointment[] {
        return Array.from(this.indexedTreatmentAppointments.values())
    }


    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM treatment_appointment ORDER BY id")

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

            // Update treatments
            const treatmentAppointmentIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = TreatmentAppointmentResponse.from(item)
                    if (!response) return

                    let treatment_appointment = this.indexedTreatmentAppointments.get(response.data.id)
                    if (treatment_appointment !== undefined) {
                        response.applyOn(treatment_appointment)
                    } else {
                        treatment_appointment = response.intoObject()
                        if (!treatment_appointment) return

                        this.indexedTreatmentAppointments.set(response.data.id, treatment_appointment)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedTreatmentAppointments.keys()).filter((id) => !treatmentAppointmentIDs.has(id)).forEach((id) => this.indexedTreatmentAppointments.delete(id))
        })
    }
}