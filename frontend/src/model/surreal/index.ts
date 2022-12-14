export interface SurrealObject {
    readonly table: string
    readonly record: Record
}

export interface SurrealResponse<T extends SurrealObject> {
    applyOn(object: T): void

    intoObject(): T | undefined
}

export class RecordError extends Error {
    name = "RecordError"
}

export class Record {
    readonly table: string
    readonly id: string

    constructor(table: string, id: string) {
        this.table = table
        this.id = id
    }

    join() {
        return this.table + ":" + this.id
    }

    public equals(other: any): boolean {
        if (!isRecord(other)) {
            return false
        }

        return other.id === this.id && other.table === this.table
    }
}

export function makeRecord(record: string): Record | undefined {
    const parts = record.split(":", 2)
    if (parts.length < 2) {
        return undefined
    }

    return new Record(parts[0], parts[1])
}

export function isRecord(record: any): record is Record {
    return typeof record.table == "string" && typeof record.id == "string";
}

export function makeRecordForTable(id: string, table: string): Record {
    const record = makeRecord(id)
    if (!isRecord(record)) {
        throw new RecordError("Given id is not a record: " + id)
    }
    if (record.table !== table) {
        throw new RecordError(`Given id is not part of table "${table}": ${id}`)
    }

    return record
}
