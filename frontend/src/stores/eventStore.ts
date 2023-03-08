import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import {ano, no} from "../util/consts";
import Event, {EventResponse} from "../model/event"
import Seminar, {SeminarResponse} from "../model/seminar";

export default class EventStore {
    indexedEvents: Map<string, Event>
    indexedSeminars: Map<string, Seminar>
    private root: typeof store
    private eventsLive: Awaited<ReturnType<typeof live>>
    private seminarsLive: Awaited<ReturnType<typeof live>>

    constructor(root: typeof store) {
        this.root = root
        this.seminarsLive = [[], ano, no]
        this.eventsLive = [[], ano, no]

        this.indexedEvents = observable.map()
        this.indexedSeminars = observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,
            events: computed.struct,
            seminars: computed.struct
        })
    }

    get events(): Event[] {
        return Array.from(this.indexedEvents.values())
    }

    get seminars(): Seminar[] {
        return Array.from(this.indexedSeminars.values())
    }

    async refresh() {
        const [_r1, refreshEvents, _c1] = this.eventsLive
        const [_r2, refreshSeminars, _c2] = this.seminarsLive


        await refreshSeminars()
        await this.root.addressStore.refresh()
        await refreshEvents()
    }

    async setup() {
        this.seminarsLive = await live("SELECT * FROM seminar ORDER BY id")
        this.eventsLive = await live("SELECT * FROM event ORDER BY id")
        this.registerSyncData()
    }

    close() {
        const [_r12r, _uadjf, cleanEvents] = this.eventsLive
        const [_r11, _uoid, cleanSeminars] = this.seminarsLive

        cleanEvents()
        cleanSeminars()
    }

    private registerSyncData() {
        autorun(() => {
            this.syncData()
        })
    }

    private syncData() {
        const [resultEvents, _u1, _c1] = this.eventsLive
        const [resultSeminars, _u2, _c2] = this.seminarsLive
        if (!(resultSeminars.length > 0 && resultSeminars[0].result)) return

        runInAction(() => {
            // Update seminars
            if (!(resultSeminars.length > 0 && resultSeminars[0].result)) return
            const seminarsIDs: Set<string> = new Set(
                resultEvents[0].result.map((item: any) => {
                    const response = SeminarResponse.from(item)
                    if (!response) return

                    let seminar = this.indexedSeminars.get(response.data.id)
                    if (seminar !== undefined) {
                        response.applyOn(seminar)
                    } else {
                        seminar = response.intoObject()
                        if (!seminar) return

                        this.indexedSeminars.set(response.data.id, seminar)
                    }
                    return response.data.id
                }).filter((it: any) => !!it))
            Array.from(this.indexedSeminars.keys()).filter((id) => !seminarsIDs.has(id)).forEach((id) => this.indexedSeminars.delete(id))

            // Update events
            if (!(resultEvents.length > 0 && resultEvents[0].result)) return
            const eventIDs: Set<string> = new Set(
                resultEvents[0].result.map((item: any) => {
                    const response = EventResponse.from(item)
                    if (!response) return

                    let event = this.indexedEvents.get(response.data.id)
                    if (event !== undefined) {
                        response.applyOn(event)
                    } else {
                        event = response.intoObject()
                        if (!event) return

                        this.indexedEvents.set(response.data.id, event)
                    }
                    return response.data.id
                }).filter((it: any) => !!it))
            Array.from(this.indexedEvents.keys()).filter((id) => !eventIDs.has(id)).forEach((id) => this.indexedEvents.delete(id))
        })
    }
}