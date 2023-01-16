import {makeAutoObservable, runInAction} from "mobx";
import {makeRecordForTable, Record, SurrealObject, SurrealResponse} from "./surreal";
import Country from "./country";

export default class ProductType {
    static readonly TABLE: string = "product_type"
    readonly table: string = ProductType.TABLE
    readonly record: Record
    readonly id: string
    name: string


    constructor(id: string, name: string) {
        this.record = makeRecordForTable(id, this.table)
        this.id = id
        this.name = name

        makeAutoObservable(this)
    }

    /*constructor(name: string){
        this.name = name
    }*/

    static assertDatabaseFields(item: any) {
        return item.id !== undefined && item.name !== undefined
    }

    updateDatabaseFields(item: any) {
        this.name = item.name
    }

}

export class ProductTypeResponse implements SurrealResponse<ProductType> {
    readonly data: {
        id: string,
        name: string,
    }

    private constructor(data: ProductTypeResponse["data"]) {
        this.data = data
    }

    static from(item: any): ProductTypeResponse | undefined {
        if (
            item.id === undefined ||
            item.name === undefined
        ) return

        return new ProductTypeResponse(item)
    }

    applyOn(object: ProductType): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Country properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.name != object.name) object.name = this.data.name
        })
    }

    intoObject(): ProductType | undefined {
        return new ProductType(this.data.id, this.data.name)
    }
}
