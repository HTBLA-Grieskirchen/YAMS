import {makeAutoObservable, runInAction} from "mobx"
import {makeRecordForTable, Record, SurrealResponse} from "./surreal";
import parseDuration from "parse-duration"

export default class Seminar {
    static readonly TABLE: string = "seminar"
    readonly table: string = Seminar.TABLE
    readonly record: Record

    /// Duration in ms
    duration: number | null
    price: number
    title: string


    constructor(
        id: string,
        duration: number | null,
        price: number,
        title: string
    ) {
        this.record = makeRecordForTable(id, this.table)
        this.duration = duration
        this.price = price
        this.title = title

        makeAutoObservable(this)
    }
}

export class SeminarResponse implements SurrealResponse<Seminar> {
    readonly data: {
        id: string,
        duration: string | null,
        price: number,
        title: string
    }

    private constructor(data: SeminarResponse["data"]) {
        this.data = data
    }

    static from(item: any): SeminarResponse | undefined {
        if (
            item.id === undefined ||
            item.price === undefined || item.title === undefined
        ) return

        return new SeminarResponse(item)
    }

    applyOn(object: Seminar): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Client properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            const duration = !!this.data.duration ? parseDuration(this.data.duration) : null
            if (duration != object.duration) object.duration = duration

            if (this.data.title != object.title) object.title = this.data.title
            if (this.data.price != object.price) object.price = this.data.price
        })
    }

    intoObject(): Seminar | undefined {
        return new Seminar(this.data.id,
            !!this.data.duration ? parseDuration(this.data.duration) : null,
            this.data.price,
            this.data.title
        )
    }
}