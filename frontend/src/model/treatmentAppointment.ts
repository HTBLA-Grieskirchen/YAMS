import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import Treatment from "./treatment";
import {makeAutoObservable, runInAction} from "mobx";
import store from "../stores";

export default class TreatmentAppointment implements SurrealObject {
    static readonly TABLE: string = "treatment_appointment"
    readonly table: string = TreatmentAppointment.TABLE
    readonly record: Record
    treatment: Treatment
    cost: number
    date: Date
    extra: string

    constructor(id: string, treatment: Treatment, cost: number, date: Date, extra: string) {
        this.record = makeRecordForTable(id, this.table)
        this.treatment = treatment
        this.cost = cost
        this.date = date
        this.extra = extra

        makeAutoObservable(this)
    }

    get label() {
        return this.cost + " " + this.date + " " + this.extra + ", " + this.treatment.targets_animal + " " + this.treatment.symptomatic
    }
}

export class TreatmentAppointmentResponse implements SurrealResponse<TreatmentAppointment> {
    readonly data: {
        id: string,
        treatment: string,
        cost: number,
        date: Date,
        extra?: string
    }

    private constructor(data: TreatmentAppointmentResponse["data"]) {
        this.data = data
    }

    static from(item: any): TreatmentAppointmentResponse | undefined {
        if (
            item.id === undefined ||
            item.treatment === undefined || item.cost === undefined || item.date == undefined
        ) return

        return new TreatmentAppointmentResponse(item)
    }

    applyOn(object: TreatmentAppointment): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // TreatmentAppointment properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.treatment != object.treatment.record.join()) {
                let treatment = store.treatmentAppointmentStore.indexedTreatments.get(this.data.treatment)
                if (treatment) object.treatment = treatment
            }

            if (this.data.date != object.date) object.date = this.data.date
            if (this.data.cost != object.cost) object.cost = this.data.cost

            const extra = this.data.extra ?? ""
            if (extra != object.extra) object.extra = extra
        })
    }

    intoObject(): TreatmentAppointment | undefined {
        let treatment = store.treatmentAppointmentStore.indexedTreatments.get(this.data.treatment)
        if (!treatment) return

        return new TreatmentAppointment(this.data.id, treatment, this.data.cost, this.data.date, this.data.extra ?? "")
    }
}