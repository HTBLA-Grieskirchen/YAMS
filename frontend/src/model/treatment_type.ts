import {makeRecordForTable, Record, RecordError, SurrealObject, SurrealResponse} from "./surreal";
import {makeAutoObservable, runInAction} from "mobx";
import Country from "./country";
import store from "../stores";

export default class Treatment_type implements SurrealObject {
    static readonly TABLE: string = "treatment_type"
    readonly table: string = Treatment_type.TABLE
    readonly record: Record
    description: string



    constructor(id: string, description: string) {
        this.record = makeRecordForTable(id, this.table)
        this.description = description


        makeAutoObservable(this)
    }
}

export class TreatmentTypeResponse implements SurrealResponse<Treatment_type> {
    readonly data: {
        id: string,
        description: string
    }

    private constructor(data: TreatmentTypeResponse["data"]) {
        this.data = data
    }

    static from(item: any): TreatmentTypeResponse| undefined {
        if (
            item.id === undefined ||
            item.description === undefined
        ) return

        return new TreatmentTypeResponse(item)
    }

    applyOn(object: Treatment_type): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // City properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
           if (this.data.description != object.description) object.description = this.data.description

        })
    }

    intoObject(): Treatment_type | undefined {

        try {

            return new Treatment_type(this.data.id, this.data.description)

        }
        catch(ex){

            return undefined

        }
    }
}
