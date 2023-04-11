import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import City from "./city";
import {makeAutoObservable, runInAction} from "mobx";
import store from "../stores";
import Treatment_type from "./treatment_type";

export default class Treatment implements SurrealObject {
    static readonly TABLE: string = "treatment"
    readonly table: string = Treatment.TABLE
    readonly record: Record
    type: Treatment_type
    description: string
    symptomatic: string
    targets_animal: boolean
    price:Number
    duration:Number

    constructor(id: string, type: Treatment_type, description: string, symptomatic: string, targets_animal:boolean,price:Number,duration:Number) {
        this.record = makeRecordForTable(id, this.table)
        this.type= type
        this.description = description
        this.symptomatic = symptomatic
        this.targets_animal=targets_animal
        this.price=price
        this.duration=duration

        makeAutoObservable(this)
    }


}

export class TreatmentResponse implements SurrealResponse<Treatment> {
    readonly data: {
        id: string,
        treatment: string,
        description: string,
        symptomatic: string,
        targets_animal: string,
        price:string,
        duration:string
    }

    private constructor(data: TreatmentResponse["data"]) {
        this.data = data
    }

    static from(item: any): TreatmentResponse | undefined {
        if (
            item.id === undefined ||
            item.treatment === undefined || item.description === undefined || item.symptomatic === undefined ||
            item.targets_animal === undefined || item.price === undefined || item.duration=== undefined
        ) return

        return new TreatmentResponse(item)
    }

    applyOn(object: Treatment): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Address properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.treatment != object.type.record.join()) {
                let type = store.treatmentStore.indexedTypes.get(this.data.treatment)
                if (type) object.type = type
            }

            if (this.data.description != object.description) object.description = this.data.description
            if (this.data.symptomatic != object.symptomatic) object.symptomatic = this.data.symptomatic
            const targets_animal=(this.data.targets_animal=="true")
            if (targets_animal != object.targets_animal) object.targets_animal = targets_animal
            const price=parseInt(this.data.price)
            if (price!= object.price) object.price = price
            const duration=parseInt(this.data.price)
            if (duration != object.duration) object.duration = duration


        })
    }

    intoObject(): Treatment | undefined {
        let type = store.treatmentStore.indexedTypes.get(this.data.treatment)
        if (!type) {
            console.log("Find koa triedment")
            return
        }
        const targets_animal=(this.data.targets_animal=="true")
        const price=parseInt(this.data.price)
        const duration=parseInt(this.data.price)

        try {

            return new Treatment(this.data.id, type, this.data.description, this.data.symptomatic, targets_animal, price, duration)

        }
        catch(Error){

            return undefined

        }
    }
}