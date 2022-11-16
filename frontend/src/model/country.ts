import {makeAutoObservable} from "mobx";

export default class Country {
    static TABLE_NAME: string = "country"
    readonly id: string
    name: string
    short: string

    constructor(id: string, name: string, short: string) {
        this.id = id
        this.name = name
        this.short = short

        makeAutoObservable(this)
    }

    record(): string {
        return Country.TABLE_NAME + ":" + this.id
    }
}