import {makeAutoObservable} from "mobx";

export default class Land {
    static TABLE_NAME: string = "land"
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
        return Land.TABLE_NAME + ":" + this.id
    }
}