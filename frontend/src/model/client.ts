import { makeAutoObservable, runInAction } from "mobx"
import { makeRecordForTable, Record, SurrealResponse } from "./surreal";
import Address from "./address";
import store from "../stores";
import Animal from "./animal";

export default class Client {
    static readonly TABLE: string = "client"
    readonly table: string = Client.TABLE
    readonly record: Record

    lastName: string
    firstName: string
    birthdate: Date
    email: string
    mobileNumber: string
    customerNumber: number

    address: Address
    consent: boolean
    animals: Animal[]

    constructor(
        id: string,
        firstName: string,
        lastName: string,
        birthdate: Date,
        email: string,
        mobileNumber: string,
        address: Address,
        consent: boolean,
        customer_number: number,
        animals?: Animal[]
    ) {
        this.record = makeRecordForTable(id, this.table)
        this.lastName = lastName
        this.firstName = firstName
        this.birthdate = birthdate
        this.email = email
        this.mobileNumber = mobileNumber
        this.customerNumber = customer_number
        this.address = address
        this.consent = consent
        this.animals = animals ?? []

        makeAutoObservable(this)
    }
}

export class ClientResponse implements SurrealResponse<Client> {
    readonly data: {
        id: string,
        first_name: string,
        last_name: string,
        email: string,
        mobile_number: string,
        birthdate: string,
        address: string,
        consent: boolean,
        customer_number: number,
        animals?: []
    }

    private constructor(data: ClientResponse["data"]) {
        this.data = data
    }

    static from(item: any): ClientResponse | undefined {
        if (
            item.id === undefined ||
            item.first_name === undefined || item.last_name === undefined || item.birthdate === undefined ||
            item.email === undefined || item.mobile_number === undefined || item.address === undefined ||
            item.consent === undefined || item.customer_number === undefined
        ) return

        return new ClientResponse(item)
    }

    applyOn(object: Client): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Client properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.address != object.address.record.join()) {
                const address = store.addressStore.indexedAddresses.get(this.data.address)
                if (address) object.address = address
            }

            if (this.data.first_name != object.firstName) object.firstName = this.data.first_name
            if (this.data.last_name != object.lastName) object.lastName = this.data.last_name
            if (this.data.email != object.email) object.email = this.data.email
            if (this.data.mobile_number != object.mobileNumber) object.mobileNumber = this.data.mobile_number
            if (this.data.customer_number != object.customerNumber) object.customerNumber = this.data.customer_number
            if (this.data.consent != object.consent) object.consent = this.data.consent

            const animals = this.data.animals
                ?.map(animal => store.animalStore.indexedAnimals.get(animal))
                .filter((item): item is Animal => !!item) ?? []
            if (animals != object.animals) object.animals = animals

            const birthdate = new Date(this.data.birthdate)
            if (birthdate != object.birthdate) object.birthdate = birthdate
        })
    }

    intoObject(): Client | undefined {
        const address = store.addressStore.indexedAddresses.get(this.data.address)
        if (!address) return

        const animals = this.data.animals?.map(animal => store.animalStore.indexedAnimals.get(animal)).filter((item): item is Animal => !!item)

        return new Client(this.data.id,
            this.data.first_name,
            this.data.last_name,
            new Date(this.data.birthdate),
            this.data.email,
            this.data.mobile_number,
            address,
            this.data.consent,
            this.data.customer_number,
            animals ?? [])
    }
}