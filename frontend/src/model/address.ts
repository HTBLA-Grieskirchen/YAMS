import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import City from "./city";
import {makeAutoObservable} from "mobx";
import store from "../stores";

export default class Address implements SurrealObject {
    static readonly TABLE: string = "address"
    readonly table: string = Address.TABLE
    readonly record: Record
    city: City
    street: string
    extra: string

    constructor(id: string, city: City, street: string, extra: string) {
        this.record = makeRecordForTable(id, this.table)
        this.city = city
        this.street = street
        this.extra = extra

        makeAutoObservable(this)
    }
}

export class AddressResponse implements SurrealResponse<Address> {
    public data: {
        id: string,
        city: string,
        street: string,
        extra?: string
    }

    private constructor(data: AddressResponse["data"]) {
        this.data = data
    }

    static from(item: any): AddressResponse | undefined {
        if (
            item.id === undefined ||
            item.city === undefined || item.street === undefined
        ) return

        return new AddressResponse(item)
    }

    applyOn(object: Address): void {
        // Check if data is meant for object
        if (object.record.join() == this.data.id) return

        if (this.data.city != object.city.record.join()) {
            let city = store.addressStore.indexedCities.get(this.data.city)
            if (city) object.city = city
        }

        if (this.data.street != object.street) object.street = this.data.street

        const extra = this.data.extra ?? ""
        if (extra != object.extra) object.extra = extra
    }

    intoObject(): Address | undefined {
        let city = store.addressStore.indexedCities.get(this.data.city)
        if (!city) return

        return new Address(this.data.id, city, this.data.street, this.data.extra ?? "")
    }
}