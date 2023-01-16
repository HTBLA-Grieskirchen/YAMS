import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import Purchase, {PurchaseResponse} from "../model/purchase";
import Product, {ProductResponse} from "../model/product";
import ProductType, {ProductTypeResponse} from "../model/productType";
import {ano, no} from "../util/consts";

export default class PurchaseStore {
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    indexedPurchases: Map<string, Purchase>
    indexedProducts: Map<string, Product>
    indexedProductTypes: Map<string, ProductType>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]

        this.indexedPurchases = observable.map()
        this.indexedProducts = observable.map()
        this.indexedProductTypes = observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,

            purchases: computed.struct,
            products: computed.struct,
            productTypes: computed.struct,
        })
    }

    get purchases(): Purchase[] {
        return Array.from(this.indexedPurchases.values())
    }

    get products(): Product[] {
        return Array.from(this.indexedProducts.values())
    }

    get productTypes(): ProductType[] {
        return Array.from(this.indexedProductTypes.values())
    }

    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM purchased, product, product_type ORDER BY id")

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
                    const response = ProductTypeResponse.from(item)
                    if (!response) return

                    let country = this.indexedProductTypes.get(response.data.id)
                    if (country !== undefined) {
                        response.applyOn(country)
                    } else {
                        country = response.intoObject()
                        if (!country) return

                        this.indexedProductTypes.set(response.data.id, country)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedProductTypes.keys()).filter((id) => !countryIDs.has(id)).forEach((id) => this.indexedProductTypes.delete(id))

            // Update cities
            const cityIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = ProductResponse.from(item)
                    if (!response) return

                    let city = this.indexedProducts.get(response.data.id)
                    if (city !== undefined) {
                        response.applyOn(city)
                    } else {
                        city = response.intoObject()
                        if (!city) return

                        this.indexedProducts.set(response.data.id, city)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedProducts.keys()).filter((id) => !cityIDs.has(id)).forEach((id) => this.indexedProducts.delete(id))

            // Update addresses
            const addressIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = PurchaseResponse.from(item)
                    if (!response) return

                    let address = this.indexedPurchases.get(response.data.id)
                    if (address !== undefined) {
                        response.applyOn(address)
                    } else {
                        address = response.intoObject()
                        if (!address) return

                        this.indexedPurchases.set(response.data.id, address)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedPurchases.keys()).filter((id) => !addressIDs.has(id)).forEach((id) => this.indexedPurchases.delete(id))
        })
    }
}