import store from "./index";
import {action, autorun, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import Address from "../model/address";
import City from "../model/city";
import Country from "../model/country";
import {ano, no} from "../util/consts";

export default class AddressStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    addresses: Address[]
    cities: City[]
    countries: Country[]

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.addresses = observable([])
        this.cities = observable([])
        this.countries = observable([])

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound
        })
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM address, city, country ORDER BY id")

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
            // Update countries
            const countries: Country[] = result[0].result.map((item: any) => {
                if (item.id !== undefined && item.name !== undefined && item.short !== undefined) {
                    return new Country(item.id, item.name, item.short)
                }
            }).filter((it: any) => it !== undefined)
            let countryMap: { [key: string]: Country } = {}
            countries.forEach((country) => countryMap[country.record.join()] = country)
            this.countries.length = 0
            this.countries.push(...countries)

            // Update cities
            const cities: City[] = result[0].result.map((item: any) => {
                if (item.id !== undefined &&
                    item.country !== undefined &&
                    item.name !== undefined &&
                    item.plz !== undefined) {
                    return new City(item.id, countryMap[item.country], item.name, item.short)
                }
            }).filter((it: any) => it !== undefined)
            let cityMap: { [key: string]: City } = {}
            cities.forEach((city) => cityMap[city.record.join()] = city)
            this.cities.length = 0
            this.cities.push(...cities)

            // Update addresses
            const addresses: Address[] = result[0].result.map((item: any) => {
                if (item.id !== undefined &&
                    item.city !== undefined &&
                    item.extra !== undefined &&
                    item.street !== undefined) {
                    return new Address(item.id, cityMap[item.city], item.street, item.extra)
                }
            }).filter((it: any) => it !== undefined)
            this.addresses.length = 0
            this.addresses.push(...addresses)
        })
    }
}