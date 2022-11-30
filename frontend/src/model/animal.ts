import {makeAutoObservable} from "mobx"

export default class Animal {
    static TABLE_NAME: string = "animal"
    id: string
    birthdate: Date
    name: string
    race: string

    constructor(id: string, birthdate: Date, name: string, race: string) {
        makeAutoObservable(this)
        this.id = id
        this.birthdate = birthdate
        this.name = name
        this.race = race
    }

    record(): string {
        return Animal.TABLE_NAME + ":" + this.id
    }
}