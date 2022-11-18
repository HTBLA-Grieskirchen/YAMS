import {makeAutoObservable} from "mobx"

export default class Client {
    last_name: string
    first_name: string
    birthdate: Date
    email: string
    mobile_number: string
    consent: boolean
    id: string

    constructor(last_name: string, first_name: string, birthdate: Date, email: string, mobile_number: string, consent: boolean, id: string) {
        makeAutoObservable(this)

        this.last_name = last_name
        this.first_name = first_name
        this.birthdate = birthdate
        this.email = email
        this.mobile_number = mobile_number
        this.consent = consent
        this.id = id
    }
}