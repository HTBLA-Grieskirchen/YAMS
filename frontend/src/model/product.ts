import {makeAutoObservable, runInAction} from "mobx";
//import ProductOverview from "../pages/Product";
import ProductType from "./productType";
import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import store from "../stores";

export default class Product {
    readonly id: string
    static readonly TABLE: string = "product"
    readonly table: string = Product.TABLE
    readonly record: Record
    name: string
    productType: ProductType
    price:number


    constructor(id: string, name: string, productType: ProductType, price:number) {
        this.record = makeRecordForTable(id, this.table)
        this.id = id
        this.name = name
        this.productType=productType
        this.price=price;

        makeAutoObservable(this)
    }
}

export class ProductResponse implements SurrealResponse<Product> {
    readonly data: {
        id: string,
        name: string,
        price: number,
        productType: string
    }

    private constructor(data: ProductResponse["data"]) {
        this.data = data
    }

    static from(item: any): ProductResponse | undefined {
        if (
            item.id === undefined ||
            item.name === undefined || item.price === undefined ||
            item.productType === undefined
        ) return

        return new ProductResponse(item)
    }

    applyOn(object: Product): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Product properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.productType != object.productType.record.join()) {
                let productType = store.purchaseStore.indexedProductTypes.get(this.data.productType)
                if (productType) object.productType = productType
            }

            if (this.data.name != object.name) object.name = this.data.name
            if (this.data.price != object.price) object.price = this.data.price
        })
    }

    intoObject(): Product | undefined {
        let productType = store.purchaseStore.indexedProductTypes.get(this.data.productType)
        if (!productType) return

        return new Product(this.data.id, this.data.name, productType, this.data.price)
    }
}