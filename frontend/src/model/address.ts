import {makeRecordForTable, Record, SurrealObject} from "./surreal";
import City from "./city";
import {makeAutoObservable} from "mobx";

export default class Address implements SurrealObject {
    static readonly TABLE: string = "address"
    readonly table: string = Address.TABLE
    readonly record: Record
    city: City
    street: string
    extra: string
    label: string

    constructor(id: string, city: City, street: string, extra: string) {
        this.record = makeRecordForTable(id, this.table)
        this.city = city
        this.street = street
        this.extra = extra
        this.label = this.street + " " + extra + ", " + this.city.plz + " " + this.city.name

        makeAutoObservable(this)
    }
}