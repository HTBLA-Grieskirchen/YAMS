import {makeAutoObservable} from "mobx"

export default class Client {
    static TABLE_NAME: string = "client"
    last_name: string
    first_name: string
    birthdate: Date
    email: string
    mobile_number: string
    consent: boolean
    id: string
    animals: []

    constructor(id: string, last_name: string, first_name: string, birthdate: Date, email: string, mobile_number: string, consent: boolean, animals: []) {
        makeAutoObservable(this)

        this.last_name = last_name
        this.first_name = first_name
        this.birthdate = birthdate
        this.email = email
        this.mobile_number = mobile_number
        this.consent = consent
        this.id = id
        this.animals = animals
    }

    record(): string {
        return Client.TABLE_NAME + ":" + this.id
    }
}