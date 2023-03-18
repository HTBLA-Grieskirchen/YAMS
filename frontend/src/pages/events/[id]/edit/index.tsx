import {NavigationPage} from "../../../../types/layout";
import {observer} from "mobx-react";
import {useRouter} from "next/router";
import {useStore} from "../../../../stores"
import Event from "../../../../model/event"
import Head from "next/head";
import React from "react";
import EventCreationForm from "../../../../components/event/creation/EventForm";
import paths from "../../../../util/paths";
import Link from "next/link";
import EventDetail from "../index";

const EventEdit: NavigationPage = observer(() => {
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
            <title>Edit Event | {event.seminar.title}</title>
        </Head>

        <main className="p-6 overflow-y-auto h-full">
            <div className="card card-compact p-2 shadow bg-base-100">
                <div className="card-body overflow-visible">
                    <h2 className="card-title">Event Modification</h2>

                    <div className="divider my-0"/>

                    <EventCreationForm event={event}/>
                </div>
            </div>
        </main>
    </>
})

EventEdit.NavPath = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let event: Event | undefined

    if (typeof id !== "string" || (event = store.eventStore.indexedEvents.get(id)) === undefined) return <>
        {EventDetail.NavPath && <EventDetail.NavPath/>}
    </>

    const path = paths.event_edit(id)
    const disabled = router.asPath == path

    return <>
        {EventDetail.NavPath && <EventDetail.NavPath/>}
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    Edit
                </button>
            </Link>
        </li>
    </>
})

export default EventEdit