import {makeAutoObservable, runInAction} from "mobx"
import {makeRecordForTable, Record, SurrealResponse} from "./surreal";
import Race from "./race";
import store from "../stores";

export default class Animal {
    static readonly TABLE: string = "animal"
    readonly table: string = Animal.TABLE
    readonly record: Record
    birthdate: Date
    name: string
    race: Race

    constructor(id: string, birthdate: Date, name: string, race: Race) {
        this.record = makeRecordForTable(id, this.table)
        this.birthdate = birthdate
        this.name = name
        this.race = race

        makeAutoObservable(this)
    }
}

export class AnimalResponse implements SurrealResponse<Animal> {
    readonly data: {
        id: string,
        birthdate: Date,
        name: string,
        race: string
    }

    private constructor(data: AnimalResponse["data"]) {
        this.data = data
    }

    static from(item: any): AnimalResponse | undefined {
        if (
            item.id === undefined ||
            item.birthdate === undefined || item.name === undefined ||
            item.race === undefined
        ) return

        return new AnimalResponse(item)
    }

    applyOn(object: Animal): void {
        if (object.record.join() != this.data.id) return

        runInAction(() => {
            if (this.data.race != object.race.record.join()) {
                let race = store.animalStore.indexedRaces.get(this.data.race)
                if (race) object.race = race
            }

            if (this.data.name != object.name) object.name = this.data.name
            const birthdate = new Date(this.data.birthdate)
            if (birthdate != object.birthdate) object.birthdate = birthdate
        })
    }

    intoObject(): Animal | undefined {
        let race = store.animalStore.indexedRaces.get(this.data.race)
        if (!race) return

        return new Animal(this.data.id, new Date(this.data.birthdate), this.data.name, race)
    }
}