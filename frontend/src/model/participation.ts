import {makeAutoObservable, runInAction} from "mobx"
import {makeRecordForTable, Record, SurrealResponse} from "./surreal";
import store from "../stores";
import Client from "./client";
import Event from "./event";


export default class EventParticipation {
    static readonly TABLE: string = "participated_in"
    readonly table: string = EventParticipation.TABLE
    readonly record: Record

    from: Client
    dest: Event
    cost: number

    constructor(
        id: string,
        from: Client,
        dest: Event,
        cost: number
    ) {
        this.record = makeRecordForTable(id, this.table)
        this.from = from
        this.dest = dest
        this.cost = cost

        makeAutoObservable(this)
    }
}

export class EventParticipationResponse implements SurrealResponse<EventParticipation> {
    readonly data: {
        id: string,
        in: string,
        out: string,
        cost: number,
    }

    private constructor(data: EventParticipationResponse["data"]) {
        this.data = data
    }

    static from(item: any): EventParticipationResponse | undefined {
        if (
            item.id === undefined || item.in === undefined || item.out === undefined ||
            item.cost === undefined
        ) return

        return new EventParticipationResponse(item)
    }

    applyOn(object: EventParticipation): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Client properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.in != object.from.record.join()) {
                const from = store.clientStore.indexedClients.get(this.data.in)
                if (from) object.from = from
            }

            if (this.data.out != object.dest.record.join()) {
                const dest = store.eventStore.indexedEvents.get(this.data.out)
                if (dest) object.dest = dest
            }

            if (this.data.cost != object.cost) object.cost = this.data.cost
        })
    }

    intoObject(): EventParticipation | undefined {
        const from = store.clientStore.indexedClients.get(this.data.in)
        const dest = store.eventStore.indexedEvents.get(this.data.out)
        if (!from || !dest) return

        return new EventParticipation(this.data.id,
            from,
            dest,
            this.data.cost
        )
    }
}