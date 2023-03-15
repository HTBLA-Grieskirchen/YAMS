import {makeAutoObservable, runInAction} from "mobx"
import {makeRecordForTable, Record, SurrealResponse} from "./surreal"

export default class Race {
    static readonly TABLE: string = "race"
    readonly table: string = Race.TABLE
    readonly record: Record
    description: string
    animal_species: string

    constructor(id: string, description: string, animal_species: string) {
        this.record = makeRecordForTable(id, this.table)
        this.description = description
        this.animal_species = animal_species

        makeAutoObservable(this)
    }
}

export class RaceResponse implements SurrealResponse<Race> {
    readonly data: {
        id: string,
        description: string,
        animal_species: string
    }

    private constructor(data: RaceResponse["data"]) {
        this.data = data
    }

    static from(item: any): RaceResponse | undefined {
        if (
            item.id === undefined ||
            item.description === undefined || item.animal_species === undefined
        ) return

        return new RaceResponse(item)
    }

    applyOn(object: Race): void {
        if (object.record.join() != this.data.id) return

        runInAction(() => {
            if (this.data.description != object.description) object.description = this.data.description
            if (this.data.animal_species != object.animal_species) object.animal_species = this.data.animal_species
        })
    }

    intoObject(): Race | undefined {
        return new Race(this.data.id, this.data.description, this.data.animal_species)
    }
}