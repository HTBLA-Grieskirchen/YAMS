import store from "./index"
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx"
import {live} from "../libs/database"
import {ano, no} from "../util/consts"
import Animal, {AnimalResponse} from "../model/animal"
import Race, {RaceResponse} from "../model/race";

export default class AnimalStore {
    indexedAnimals: Map<string, Animal>
    indexedRaces: Map<string, Race>
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexedAnimals = observable.map()
        this.indexedRaces = observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,
            animals: computed.struct,
            races: computed.struct
        })
    }

    get animals(): Animal[] {
        return Array.from(this.indexedAnimals.values())
    }

    get races(): Race[] {
        return Array.from(this.indexedRaces.values())
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await this.root.animalStore.refresh()
        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM animal ORDER BY id")

        this.registerSyncData()
    }

    close() {
        const [_r, _u, clean] = this.dataLive

        clean()
    }

    private registerSyncData() {
        autorun(() => {
            this.syncData()
        })
    }

    private syncData() {
        const [result, _u, _c] = this.dataLive
        if (!(result.length > 0 && result[0].result)) return

        runInAction(() => {
            // Update races
            const raceIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = RaceResponse.from(item)
                    if (!response) return

                    let race = this.indexedRaces.get(response.data.id)
                    if (race !== undefined) {
                        response.applyOn(race)
                    } else {
                        race = response.intoObject()
                        if (!race) return

                        this.indexedRaces.set(response.data.id, race)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedRaces.keys()).filter((id) => !raceIDs.has(id)).forEach((id) => this.indexedRaces.delete(id))

            // Update animals
            const animalIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = AnimalResponse.from(item)
                    if (!response) return

                    let animal = this.indexedAnimals.get(response.data.id)
                    if (animal !== undefined) {
                        response.applyOn(animal)
                    } else {
                        animal = response.intoObject()
                        if (!animal) return

                        this.indexedAnimals.set(response.data.id, animal)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedAnimals.keys()).filter((id) => !animalIDs.has(id)).forEach((id) => this.indexedAnimals.delete(id))
        })
    }
}