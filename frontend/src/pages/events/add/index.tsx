import {observer} from "mobx-react";
import Head from "next/head";
import {NavigationPage} from "../../../types/layout";
import {useStore} from "../../../stores";
import paths from "../../../util/paths";
import Link from "next/link";
import React from "react";
import {useRouter} from "next/router";

const EventsCreation: NavigationPage = observer(() => {
    const store = useStore()
    const clients = store.clientStore.clients

    return (
        <>
            <Head>
                <title>All Events</title>
            </Head>

            <main className="p-6 overflow-y-auto">
                <div className="flex flex-col pt-3">
                    <p>Love my life</p>
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

export default EventsCreation