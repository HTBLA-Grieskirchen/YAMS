import {makeAutoObservable} from "mobx"

export default class Client {
    lastName: string
    firstName: string
    birthdate: Date
    email: string
    mobileNumber: string
    consent: boolean
    id: string

    constructor(lastName: string, firstName: string, birthdate: Date, email: string, mobileNumber: string, consent: boolean, id: string) {
        makeAutoObservable(this)

        this.lastName = lastName
        this.firstName = firstName
        this.birthdate = birthdate
        this.email = email
        this.mobileNumber = mobileNumber
        this.consent = consent
        this.id = id
    }
}