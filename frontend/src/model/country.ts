import {makeAutoObservable, runInAction} from "mobx";
import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";

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

    static assertDatabaseFields(item: any) {
        return item.id !== undefined && item.name !== undefined && item.short !== undefined
    }

    updateDatabaseFields(item: any) {
        this.name = item.name
        this.short = item.short
    }
}

export class CountryResponse implements SurrealResponse<Country> {
    readonly data: {
        id: string,
        name: string,
        short: string
    }

    private constructor(data: CountryResponse["data"]) {
        this.data = data
    }

    static from(item: any): CountryResponse | undefined {
        if (
            item.id === undefined ||
            item.name === undefined || item.short === undefined
        ) return

        return new CountryResponse(item)
    }

    applyOn(object: Country): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Country properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.name != object.name) object.name = this.data.name
            if (this.data.short != object.short) object.short = this.data.short
        })
    }

    intoObject(): Country | undefined {
        return new Country(this.data.id, this.data.name, this.data.short)
    }
}
