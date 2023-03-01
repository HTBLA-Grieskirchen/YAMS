import {makeAutoObservable, runInAction} from "mobx";
import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";

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

    static assertDatabaseFields(item: any) {
        return item.id !== undefined && item.description !== undefined 
    }

    updateDatabaseFields(item: any) {
        this.description = item.description
    }
}

export class Treatment_typeResponse implements SurrealResponse<Treatment_type> {
    readonly data: {
        id: string,
        description: string
    }

    private constructor(data: Treatment_typeResponse["data"]) {
        this.data = data
    }

    static from(item: any): Treatment_typeResponse | undefined {
        if (
            item.id === undefined ||
            item.description === undefined
        ) return

        return new Treatment_typeResponse(item)
    }

    applyOn(object: Treatment_type): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Treatment_type properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.description != object.description) object.description = this.data.description
        })
    }

    intoObject(): Treatment_type | undefined {
        return new Treatment_type(this.data.id, this.data.description)
    }
}
