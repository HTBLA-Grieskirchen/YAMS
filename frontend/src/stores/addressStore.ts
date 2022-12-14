import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import Address, {AddressResponse} from "../model/address";
import City, {CityResponse} from "../model/city";
import Country, {CountryResponse} from "../model/country";
import {ano, no} from "../util/consts";

export default class AddressStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    indexedAddresses: Map<string, Address>
    indexedCities: Map<string, City>
    indexedCountries: Map<string, Country>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexedAddresses = observable.map()
        this.indexedCities = observable.map()
        this.indexedCountries = observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,

            addresses: computed.struct,
            countries: computed.struct,
            cities: computed.struct,
        })
    }

    get addresses(): Address[] {
        return Array.from(this.indexedAddresses.values())
    }

    get countries(): Country[] {
        return Array.from(this.indexedCountries.values())
    }

    get cities(): City[] {
        return Array.from(this.indexedCities.values())
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
            const countryIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = CountryResponse.from(item)
                    if (!response) return

                    let country = this.indexedCountries.get(response.data.id)
                    if (country !== undefined) {
                        response.applyOn(country)
                    } else {
                        country = response.intoObject()
                        if (!country) return

                        this.indexedCountries.set(response.data.id, country)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedCountries.keys()).filter((id) => !countryIDs.has(id)).forEach((id) => this.indexedCountries.delete(id))

            // Update cities
            const cityIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = CityResponse.from(item)
                    if (!response) return

                    let city = this.indexedCities.get(response.data.id)
                    if (city !== undefined) {
                        response.applyOn(city)
                    } else {
                        city = response.intoObject()
                        if (!city) return

                        this.indexedCities.set(response.data.id, city)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedCities.keys()).filter((id) => !cityIDs.has(id)).forEach((id) => this.indexedCities.delete(id))

            // Update addresses
            const addressIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = AddressResponse.from(item)
                    if (!response) return

                    let address = this.indexedAddresses.get(response.data.id)
                    if (address !== undefined) {
                        response.applyOn(address)
                    } else {
                        address = response.intoObject()
                        if (!address) return

                        this.indexedAddresses.set(response.data.id, address)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedAddresses.keys()).filter((id) => !addressIDs.has(id)).forEach((id) => this.indexedAddresses.delete(id))
        })
    }
}