import {makeAutoObservable, runInAction} from "mobx"
import {makeRecordForTable, Record, SurrealResponse} from "./surreal";
import Address from "./address";
import store from "../stores";
import Seminar from "./seminar";

export default class Event {
    static readonly TABLE: string = "event"
    readonly table: string = Event.TABLE
    readonly record: Record

    date: Date
    location: Address
    locationName: string | null
    maxParticipants: number | null
    seminar: Seminar


    constructor(
        id: string,
        date: Date,
        maxParticipants: number | null,
        locationName: string | null,
        location: Address,
        seminar: Seminar
    ) {
        this.record = makeRecordForTable(id, this.table)
        this.date = date
        this.maxParticipants = maxParticipants
        this.locationName = locationName
        this.location = location
        this.seminar = seminar

        makeAutoObservable(this)
    }

    get when() {
        const timeRemaining = this.date.valueOf() - Date.now()

        return timeRemaining < -(this.seminar.duration ?? 0) ?
            0 :
            timeRemaining < 0 ?
                1 :
                timeRemaining < 1000 * 3600 * 24 * 7 ?
                    2 :
                    3
    }

    get daysFromNow() {
        return Math.round(
            new Date(this.date).setUTCHours(0, 0, 0) - new Date().setUTCHours(0, 0, 0)
        ) / (1000 * 3600 * 24)
    }
}

export class EventResponse implements SurrealResponse<Event> {
    readonly data: {
        id: string,
        date: string,
        max_participants: number | null,
        location_name: string | null,
        location: string,
        seminar: string
    }

    private constructor(data: EventResponse["data"]) {
        this.data = data
    }

    static from(item: any): EventResponse | undefined {
        if (
            item.id === undefined ||
            item.date === undefined ||
            item.location === undefined || item.seminar === undefined
        ) return

        return new EventResponse(item)
    }

    applyOn(object: Event): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Client properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.location != object.location.record.join()) {
                const address = store.addressStore.indexedAddresses.get(this.data.location)
                if (address) object.location = address
            }

            if (this.data.seminar != object.seminar.record.join()) {
                const seminar = store.eventStore.indexedSeminars.get(this.data.seminar)
                if (seminar) object.seminar = seminar
            }


            const date = new Date(this.data.date)
            if (date != object.date) object.date = date

            if (this.data.location_name != object.locationName) object.locationName = this.data.location_name
            if (this.data.max_participants != object.maxParticipants) object.maxParticipants = this.data.max_participants
        })
    }

    intoObject(): Event | undefined {
        const address = store.addressStore.indexedAddresses.get(this.data.location)
        const seminar = store.eventStore.indexedSeminars.get(this.data.seminar)
        if (!address || !seminar) return

        return new Event(this.data.id,
            new Date(this.data.date),
            this.data.max_participants,
            this.data.location_name,
            address,
            seminar)
    }
}