import { makeAutoObservable, runInAction } from "mobx"
import { makeRecordForTable, Record, SurrealResponse } from "./surreal";
import store from "../stores";
import Client from "./client";

export default class ClientRelation {
    static readonly TABLE: string = "client_relation"
    readonly table: string = ClientRelation.TABLE
    readonly record: Record

    from: Client
    dest: Client
    relationType: string

    constructor(
        id: string,
        from: Client,
        dest: Client,
        type: string
    ) {
        this.record = makeRecordForTable(id, this.table)
        this.from = from
        this.dest = dest
        this.relationType = type

        makeAutoObservable(this)
    }
}

export class ClientRelationResponse implements SurrealResponse<ClientRelation> {
    readonly data: {
        id: string,
        in: string,
        out: string,
        relation_type: string,
    }

    private constructor(data: ClientRelationResponse["data"]) {
        this.data = data
    }

    static from(item: any): ClientRelationResponse | undefined {
        if (
            item.id === undefined || item.in === undefined || item.out === undefined ||
            item.relation_type === undefined
        ) return

        return new ClientRelationResponse(item)
    }

    applyOn(object: ClientRelation): void {
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
                const dest = store.clientStore.indexedClients.get(this.data.out)
                if (dest) object.dest = dest
            }

            if (this.data.relation_type != object.relationType) object.relationType = this.data.relation_type
        })
    }

    intoObject(): ClientRelation | undefined {
        const from = store.clientStore.indexedClients.get(this.data.in)
        const dest = store.clientStore.indexedClients.get(this.data.out)
        if (!from || !dest) return

        return new ClientRelation(this.data.id,
            from,
            dest,
            this.data.relation_type
        )
    }
}