import {makeAutoObservable} from "mobx";
import {makeRecordForTable, Record, SurrealObject} from "./surreal";

export default class Country implements SurrealObject {
    readonly table: string = "country"
    readonly record: Record
    name: string
    short: string

    constructor(id: string, name: string, short: string) {
        this.record = makeRecordForTable(id, this.table)
        this.name = name
        this.short = short

        makeAutoObservable(this)
    }
}