import {observer} from "mobx-react";
import Head from "next/head";
import {NavigationPage} from "../../types/layout";
import {useStore} from "../../stores";
import paths from "../../util/paths";
import Link from "next/link";
import React, {useState} from "react";
import {useRouter} from "next/router";
import Event from "../../model/event"
import EventOverviewItem from "../../components/event/EventOverviewItem";
import {SmallSearchField} from "../../components/form/search";

type DisplayCategory = "upcoming" | "future" | "past"

const EventsOverview: NavigationPage = observer(() => {
    const store = useStore()

    const [filter, setFilter] = useState("")
    const [category, setCategory] = useState<DisplayCategory>("upcoming")
    const [showPast, setShowPast] = useState(true)

    const filteredEvents = store.eventStore.events.filter(event => eventSearched(event, filter))
    const categorizedEvents = category == "upcoming" ?
        filteredEvents.filter(event => showPast && event.when == 0)
            .sort((e1, e2) => e2.date.valueOf() - e1.date.valueOf())
            .slice(0, 4).reverse()
            .concat(
                filteredEvents.filter(event => event.when <= 2 && event.when > 0)
                    .sort((e1, e2) => e1.date.valueOf() - e2.date.valueOf())) :
        category == "future" ?
            filteredEvents.filter(event => event.when > 0)
                .sort((e1, e2) => e1.date.valueOf() - e2.date.valueOf()) :
            category == "past" ?
                filteredEvents.filter(event => event.when == 0)
                    .sort((e1, e2) => e2.date.valueOf() - e1.date.valueOf()) :
                []

    return (
        <>
            <Head>
                <title>Events Overview</title>
            </Head>

            <main className="p-6 pt-4 h-full overflow-y-auto">
                <div className="max-h-full flex flex-col gap-4">
                    <div className="w-full flex flex-row gap-4">
                        <div className="form-control">
                            <label className="label">
                                <span className="label-text">Which events to show</span>
                            </label>
                            <select className="select select-bordered w-full max-w-xs"
                                    onChange={e => setCategory(e.target.value as DisplayCategory)}>
                                <option value="upcoming">Upcoming</option>
                                <option value="future">All future</option>
                                <option value="past">All past</option>
                            </select>
                        </div>

                        {category == "upcoming" &&
                            <div className="form-control mt-10">
                                <label className="label cursor-pointer">
                                    <span className="label-text mr-3">Show past events</span>
                                    <input type="checkbox" className="checkbox checkbox-primary" checked={showPast}
                                           onChange={e => setShowPast(e.target.checked)}/>
                                </label>
                            </div>}
                    </div>
                    <div className="w-full flex flex-row gap-4 overflow-x-auto">
                        <div className="card card-compact bg-base-100 shadow p-2 max-h-full w-full">
                            <div className="card-body overflow-visible overflow-x-auto">
                                <div className="flex flex-col lg:flex-row justify-between space-y-2 lg:space-y-0">
                                    <h2 className="card-title">
                                        {category == "upcoming" ?
                                            "Upcoming events" :
                                            category == "future" ?
                                                "All future events" :
                                                category == "past" ?
                                                    "All past events" :
                                                    ""}
                                    </h2>
                                    <SmallSearchField value={filter} onChange={(value) => setFilter(value)}/>
                                </div>

                                <div className="divider my-0"/>

                                <div className="flex flex-col divide-y overflow-y-auto">
                                    {categorizedEvents.map((event) =>
                                        <EventOverviewItem key={event.record.join()} event={event}/>)}
                                </div>
                            </div>
                        </div>

                        <div className="card card-compact bg-base-100 shadow p-2 max-h-full w-fit">
                            <div className="card-body overflow-visible">
                                <h2 className="card-title">Calendar</h2>

                                <div className="divider my-0"/>

                                <div></div>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </>
    )
})

EventsOverview.NavMenu = observer(() => {
    return <>
        <li>
            <Link href={paths.event_new}>
                <a><i className="fa-solid fa-plus"/>Add</a>
            </Link>
        </li>
    </>
})

EventsOverview.NavPath = observer(() => {
    const path = paths.events

    const router = useRouter()
    const disabled = router.pathname == path

    return <>
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    <i className="fa-solid fa-calendar-days mr-2"/>
                    Events
                </button>
            </Link>
        </li>
    </>
})

function eventSearched(event: Event, query: string): boolean {
    return query.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
        const trimmedSegment = segment.trim().toLowerCase()
        if (trimmedSegment.length < 0) return true

        return event.seminar.title.toLowerCase().includes(trimmedSegment) ||
            event.locationName?.toLowerCase().includes(trimmedSegment) ||
            event.location.street.toLowerCase().includes(trimmedSegment) ||
            event.location.streetNumber.toLowerCase().includes(trimmedSegment)
    })
}

export default EventsOverview