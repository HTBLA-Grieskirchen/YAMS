import {makeAutoObservable} from "mobx";
import ProductOverview from "../pages/Product";
import ProductType from "./ProductType";

export default class Product {
    static TABLE_NAME: string = "product"
    readonly id: string
    bezeichnung: string
    productType: ProductType
    price:number


    constructor(id: string, bezeichnung: string, productType:ProductType, price:number) {
        this.id = id
        this.bezeichnung = bezeichnung
        this.productType=productType
        this.price=price;

        makeAutoObservable(this)
    }

    record(): string {
        return Product.TABLE_NAME + ":" + this.id
    }
}