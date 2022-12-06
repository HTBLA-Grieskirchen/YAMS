import {makeAutoObservable} from "mobx";
import {makeRecordForTable, Record, SurrealObject} from "./surreal";

export default class Country implements SurrealObject {
    static readonly TABLE: string = "country"
    readonly table: string = Country.TABLE
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