import {observer} from "mobx-react";
import Head from "next/head";
import {NavigationPage} from "../../../types/layout";
import {useStore} from "../../../stores";
import paths from "../../../util/paths";
import Link from "next/link";
import React from "react";
import {useRouter} from "next/router";
import EventOverview from "../index";
import EventCreationForm from "../../../components/event/creation/EventForm";

const EventsCreation: NavigationPage = observer(() => {
    const store = useStore()
    const clients = store.clientStore.clients

    return (
        <>
            <Head>
                <title>New Event</title>
            </Head>

            <main className="p-6 overflow-y-auto h-full">
                <div className="card card-compact p-2 shadow bg-base-100">
                    <div className="card-body overflow-visible">
                        <h2 className="card-title">Event Creation</h2>

                        <div className="divider my-0"/>

                        <EventCreationForm/>
                    </div>
                </div>
            </main>
        </>
    )
})

EventsCreation.NavMenu = observer(() => {
    return <>
        <li>
            <Link href={paths.event_new}>
                <a><i className="fa-solid fa-plus"/>Add</a>
            </Link>
        </li>
    </>
})

EventsCreation.NavPath = observer(() => {
    const path = paths.event_new

    const router = useRouter()
    const disabled = router.pathname == path

    return <>
        {EventOverview.NavPath && <EventOverview.NavPath/>}
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    <i className="fa-solid fa-calendar-plus mr-2"/>
                    Add
                </button>
            </Link>
        </li>
    </>
})

export default EventsCreation