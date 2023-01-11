import {makeAutoObservable} from "mobx"
import {makeRecordForTable, Record} from "./surreal";

export default class Animal {
    static TABLE_NAME: string = "animal"
    readonly record: Record
    id: string
    birthdate: Date
    name: string
    race: string

    constructor(id: string, birthdate: Date, name: string, race: string) {
        this.record = makeRecordForTable(id, Animal.TABLE_NAME)
        makeAutoObservable(this)
        this.id = id
        this.birthdate = birthdate
        this.name = name
        this.race = race
    }
}