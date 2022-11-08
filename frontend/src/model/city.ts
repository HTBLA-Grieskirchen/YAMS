import {makeRecordForTable, Record, SurrealObject} from "./surreal";
import {makeAutoObservable} from "mobx";
import Country from "./country";

export default class City implements SurrealObject {
    readonly table: string = "city"
    readonly record: Record
    country: Country
    name: string
    plz: string

    constructor(id: string, country: Country, name: string, plz: string) {
        this.record = makeRecordForTable(id, this.table)
        this.country = country
        this.name = name
        this.plz = plz

        makeAutoObservable(this)
    }
}
