import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal"
import {makeAutoObservable, runInAction} from "mobx"

export default class Address implements SurrealObject {
    static readonly TABLE: string = "address"
    readonly table: string = Address.TABLE
    readonly record: Record

    country: string
    postalCode: string
    city: string
    street: string
    streetNumber: string
    extra: string | null

    constructor(
        id: string,
        country: string,
        postalCode: string,
        city: string,
        street: string,
        streetNumber: string,
        extra?: string | null
    ) {
        this.record = makeRecordForTable(id, this.table)
        this.country = country
        this.postalCode = postalCode
        this.city = city
        this.street = street
        this.streetNumber = streetNumber
        this.extra = extra ?? null

        makeAutoObservable(this)
    }
}

export class AddressResponse implements SurrealResponse<Address> {
    readonly data: {
        id: string,
        country: string,
        city: string,
        postal_code: string,
        street: string,
        street_number: string,
        extra: string | null
    }

    private constructor(data: AddressResponse["data"]) {
        this.data = data
    }

    static from(item: any): AddressResponse | undefined {
        if (
            item.id === undefined ||
            item.country === undefined ||
            item.postal_code === undefined || item.city === undefined ||
            item.street === undefined || item.street_number == undefined
        ) return

        return new AddressResponse(item)
    }

    applyOn(object: Address): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Address properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.country != object.country) object.country = this.data.country
            if (this.data.postal_code != object.postalCode) object.postalCode = this.data.postal_code
            if (this.data.city != object.city) object.city = this.data.city
            if (this.data.street != object.street) object.street = this.data.street
            if (this.data.street_number != object.streetNumber) object.streetNumber = this.data.street_number
            if (this.data.street != object.street) object.street = this.data.street

            const extra = this.data.extra ?? null
            if (extra != object.extra) object.extra = extra
        })
    }

    intoObject(): Address | undefined {
        return new Address(
            this.data.id,
            this.data.country,
            this.data.postal_code,
            this.data.city,
            this.data.street,
            this.data.street_number,
            this.data.extra ?? null
        )
    }
}