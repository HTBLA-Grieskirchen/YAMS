import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import {makeAutoObservable, runInAction} from "mobx";
import Country from "./country";
import store from "../stores";

export default class City implements SurrealObject {
    static readonly TABLE: string = "city"
    readonly table: string = City.TABLE
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

export class CityResponse implements SurrealResponse<City> {
    readonly data: {
        id: string,
        name: string,
        plz: string,
        country: string
    }

    private constructor(data: CityResponse["data"]) {
        this.data = data
    }

    static from(item: any): CityResponse | undefined {
        if (
            item.id === undefined ||
            item.name === undefined || item.plz === undefined ||
            item.country === undefined
        ) return

        return new CityResponse(item)
    }

    applyOn(object: City): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // City properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.country != object.country.record.join()) {
                let country = store.addressStore.indexedCountries.get(this.data.country)
                if (country) object.country = country
            }

            if (this.data.name != object.name) object.name = this.data.name
            if (this.data.plz != object.plz) object.plz = this.data.plz
        })
    }

    intoObject(): City | undefined {
        let country = store.addressStore.indexedCountries.get(this.data.country)
        if (!country) return

        return new City(this.data.id, country, this.data.name, this.data.plz)
    }
}
