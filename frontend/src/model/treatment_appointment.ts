import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import City from "./city";
import {makeAutoObservable, runInAction} from "mobx";
import store from "../stores";
import Treatment_type from "./treatment_type";
import Treatment from "./treatment";

export default class Treatment_Appointment implements SurrealObject {
    static readonly TABLE: string = "treatment_appointment"
    readonly table: string = Treatment_Appointment.TABLE
    readonly record: Record
    extra: string
    cost:Number
    date:Date
    treatment:Treatment

    constructor(id: string, extra:string, cost:Number, date:Date, treatment:Treatment) {
        this.record = makeRecordForTable(id, this.table)
        this.extra= extra
        this.cost = cost
        this.date = date
        this.treatment=treatment

        makeAutoObservable(this)
    }


}

export class TreatmentAppointmentResponse implements SurrealResponse<Treatment_Appointment> {
    readonly data: {
        id: string,
        extra: string,
        cost: string,
        date: string,
        treatment: string
    }

    private constructor(data: TreatmentAppointmentResponse["data"]) {
        this.data = data
    }

    static from(item: any): TreatmentAppointmentResponse | undefined {
        if (
            item.id === undefined ||
            item.treatment === undefined || item.extra === undefined || item.cost === undefined ||
            item.date === undefined
        ) return

        return new TreatmentAppointmentResponse(item)
    }

    applyOn(object: Treatment_Appointment): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Address properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.treatment != object.treatment.record.join()) {
                let treatment = store.treatmentStore.indexedTreatments.get(this.data.treatment)
                if (treatment) object.treatment = treatment
            }

            if (this.data.extra != object.extra) object.extra = this.data.extra
            const date = new Date(this.data.date)
            if (date!= object.date) object.date = date
            const cost=parseInt(this.data.cost)
            if (cost!= object.cost) object.cost = cost



        })
    }

    intoObject(): Treatment_Appointment | undefined {
        let treatment = store.treatmentStore.indexedTreatments.get(this.data.treatment)
        if (!treatment) {
            console.log("Find koa triedment")
            return
        }
        const cost=parseInt(this.data.cost)
        const date=new Date(this.data.date)


        try {

            return new Treatment_Appointment(this.data.id,this.data.extra, cost, date, treatment)

        }
        catch(Error){

            return undefined

        }
    }
}