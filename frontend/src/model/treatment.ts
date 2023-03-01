import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import {makeAutoObservable, runInAction} from "mobx";
import Treatment_type from "./treatment_type";
import store from "../stores";

export default class Treatment implements SurrealObject {
    static readonly TABLE: string = "treatment"
    readonly table: string = Treatment.TABLE
    readonly record: Record
    treatment_type: Treatment_type
    description: string
    price: number
    symptomatic: string
    targets_animal:boolean

    constructor(id: string, treatment_type: Treatment_type, description: string, price:number, symptomatic: string, targets_animal:boolean) {
        this.record = makeRecordForTable(id, this.table)
        this.treatment_type = treatment_type
        this.description = description
        this.price = price
        this.symptomatic = symptomatic
        this.targets_animal = targets_animal

        makeAutoObservable(this)
    }
}

export class TreatmentResponse implements SurrealResponse<Treatment> {
    readonly data: {
        id: string,
        description: string,
        price: number
        symptomatic: string,
        targets_animal:boolean,
        treatment_type: string
    }

    private constructor(data: TreatmentResponse["data"]) {
        this.data = data
    }

    intoObject(): Treatment | undefined {
        let treatment_type = store.treatmentAppointmentStore.indexedTreatment_types.get(this.data.treatment_type)
        if (!treatment_type) return

        return new Treatment(this.data.id, treatment_type, this.data.description, this.data.price, this.data.symptomatic, this.data.targets_animal)
    }

    static from(item: any): TreatmentResponse | undefined {
        if (
            item.id === undefined || item.description == undefined || item.price == undefined ||
            item.symptomatic === undefined || item.targets_animal === undefined ||
            item.treatment_type === undefined
        ) return

        return new TreatmentResponse(item)
    }

    applyOn(object: Treatment): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Treatment properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.treatment_type != object.treatment_type.record.join()) {
                let treatment_type = store.treatmentAppointmentStore.indexedTreatment_types.get(this.data.treatment_type)
                if (treatment_type) object.treatment_type = treatment_type
            }

            if (this.data.description != object.description) object.description = this.data.description
            if (this.data.price != object.price) object.price = this.data.price
            if (this.data.symptomatic != object.symptomatic) object.symptomatic = this.data.symptomatic
            if (this.data.targets_animal != object.targets_animal) object.targets_animal = this.data.targets_animal
        })
    }
}
