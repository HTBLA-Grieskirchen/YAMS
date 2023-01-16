import {makeAutoObservable, runInAction} from "mobx";
//import ProductOverview from "../pages/Product";
import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import store from "../stores";
import Product from "./product";

export default class Purchase {
    readonly id: string
    static readonly TABLE: string = "purchase"
    readonly table: string = Purchase.TABLE
    readonly record: Record
    product: Product
    amount:number


    constructor(id: string, product: Product, amount:number) {
        this.record = makeRecordForTable(id, this.table)
        this.id = id
        this.product=product
        this.amount=amount;

        makeAutoObservable(this)
    }
}

export class PurchaseResponse implements SurrealResponse<Purchase> {
    readonly data: {
        id: string,
        product: string,
        amount: number
    }

    private constructor(data: PurchaseResponse["data"]) {
        this.data = data
    }

    static from(item: any): PurchaseResponse | undefined {
        if (
            item.id === undefined ||
            item.product === undefined || item.amount === undefined
        ) return

        return new PurchaseResponse(item)
    }

    applyOn(object: Purchase): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Purchased properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.product != object.product.record.join()) {
                let product = store.purchaseStore.indexedProducts.get(this.data.product)
                if (product) object.product = product
            }

            if (this.data.amount != object.amount) object.amount = this.data.amount

        })
    }

    intoObject(): Purchase | undefined {
        let product = store.purchaseStore.indexedProducts.get(this.data.product)
        if (!product) return

        return new Purchase(this.data.id, product, this.data.amount)
    }
}