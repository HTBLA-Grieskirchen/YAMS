import {observer} from "mobx-react";
import {useRouter} from "next/router";
import {useStore} from "../../../stores";
import paths from "../../../util/paths";
import Link from "next/link";
import React from "react";
import {NavigationPage} from "../../../types/layout";
import Event from "../../../model/event";
import Head from "next/head";
import EventsOverview from "../index";

const EventDetail: NavigationPage = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let event: Event | undefined

    if (typeof id !== "string" || (event = store.eventStore.indexedEvents.get(id)) === undefined)
        return <div className="p-6 overflow-y-auto h-full w-full flex flex-col items-center justify-center space-y-4">
            <i className="text-9xl text-error fa-solid fa-calendar-xmark"/>
            <p className="text-6xl font-bold text-error">Unknown Event</p>
        </div>

    return <>
        <Head>
            <title>Event | {event.seminar.title}</title>
        </Head>

        <main className="p-6 overflow-y-auto h-full">
            <div className="card card-compact p-2 shadow bg-base-100">
                <div className="card-body overflow-visible">
                    <h2 className="card-title">Not implemented</h2>

                    <div className="divider my-0"/>

                    <div>
                        Unsupported
                    </div>
                </div>
            </div>
        </main>
    </>
})

EventDetail.NavPath = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let event: Event | undefined

    if (typeof id !== "string" || (event = store.eventStore.indexedEvents.get(id)) === undefined) return <>
        {EventsOverview.NavPath && <EventsOverview.NavPath/>}
        <li>
            <button
                className={`btn btn-ghost btn-sm px-2 normal-case font-normal text-lg pointer-events-none`}>
                Invalid Event
            </button>
        </li>
    </>

    const path = paths.event(id)
    const disabled = router.asPath == path

    return <>
        {EventsOverview.NavPath && <EventsOverview.NavPath/>}
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    {event.seminar.title}
                </button>
            </Link>
        </li>
    </>
})

export default EventDetail
