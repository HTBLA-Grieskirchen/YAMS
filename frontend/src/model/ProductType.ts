import {makeAutoObservable} from "mobx";

export default class ProductType {
    static TABLE_NAME: string = "productType"
    readonly id: string
    name?: string


    constructor(id: string, name?: string) {
        this.id = id
        this.name = name

        makeAutoObservable(this)
    }

    constuctor(name: string){
        this.name = name
    }

    record(): string {
        return ProductType.TABLE_NAME + ":" + this.id
    }
}