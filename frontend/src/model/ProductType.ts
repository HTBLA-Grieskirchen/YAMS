import {makeAutoObservable} from "mobx";

export default class ProductType {
    static TABLE_NAME: string = "productType"
    readonly id: string
    bezeichnung: string


    constructor(id: string, bezeichnung: string) {
        this.id = id
        this.bezeichnung = bezeichnung

        makeAutoObservable(this)
    }

    record(): string {
        return ProductType.TABLE_NAME + ":" + this.id
    }
}