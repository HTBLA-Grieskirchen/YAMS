import { observer } from "mobx-react";
import { useRouter } from "next/router";
import { useStore } from "../../../stores";
import paths from "../../../util/paths";
import Link from "next/link";
import React from "react";
import { NavigationPage } from "../../../types/layout";
import Event from "../../../model/event";
import Head from "next/head";
import EventsOverview from "../index";
import { useSubmissionState } from "../../../libs/form/submit";
import { askSubmitDeleteEvent } from "../../../components/event";
import { formatDuration } from "../../../util/helpers";
import EventDetailItem from "../../../components/event/EventDetailItem";
import { askAddParticipantEvent } from "../../../components/participation";
import { NavbarMenuEntry } from "../../../components/layout/MainNavbar";
import EventParticipants from "../../../components/event/EventParticipants";

const EventDetail: NavigationPage = observer(() => {
    const router = useRouter()
    const store = useStore()
    const language = store.settingsStore.language

    const {id} = router.query
    let event: Event | undefined

    if (typeof id !== "string" || (event = store.eventStore.indexedEvents.get(id)) === undefined)
        return <div className="p-6 overflow-y-auto h-full w-full flex flex-col items-center justify-center space-y-4">
            <i className="text-9xl text-error fa-solid fa-calendar-xmark"/>
            <p className="text-6xl font-bold text-error">Unknown Event</p>
        </div>

    const timeRemaining = event.date.valueOf() - Date.now()
    const eventWhen = event.when


    return <>
        <Head>
            <title>Event | {event.seminar.title}</title>
        </Head>

        <main className="p-6 overflow-y-auto h-full flex-col space-y-4">
            <div className="card card-compact p-2 shadow bg-base-100">
                <div className="card-body overflow-visible">
                    <div className="card-title">
                        <h2 className="mb-1">{event.seminar.title}</h2>
                        <span className="text-sm text-base-content/75">
                            {eventWhen <= 0 ?
                                <span className="badge">Past</span> :
                                eventWhen <= 1 ?
                                    <>
                                        <span className="badge badge-error">Now</span>
                                        {!!event.seminar.duration && <>
                                            <i className="mx-1 pl-1 fa-solid fa-hourglass-half"/>
                                            <span className="mx-1">for</span>
                                            {Math.floor(-timeRemaining / 60_000) == 420 ?
                                                <i className="fa-solid fa-cannabis"/> :
                                                formatDuration(event.seminar.duration + timeRemaining, language)}</>}
                                    </> :
                                    eventWhen <= 2 ?
                                        <>
                                            <span className="badge badge-warning">Upcoming</span>
                                            {!!event.seminar.duration && <>
                                                <i className="mx-1 pl-1 fa-solid fa-hourglass-start"/>
                                                <span className="mx-1">in</span>
                                                {formatDuration(timeRemaining, language)}</>}
                                        </> :
                                        <span className="badge badge-success">Future</span>}
                        </span>
                    </div>

                    <div className="divider my-0"/>

                    <div>
                        <EventDetailItem event={event}/>
                    </div>
                </div>
            </div>

            <EventParticipants event={event}/>
        </main>
    </>
})

EventDetail.NavMenu = observer(() => {
    const router = useRouter()
    const store = useStore()
    const deleteState = useSubmissionState()
    const addParticipantState = useSubmissionState()

    const {id} = router.query
    let event: Event | undefined

    if (typeof id !== "string" || (event = store.eventStore.indexedEvents.get(id)) === undefined) return <></>

    const participants = store.eventStore.participationStore.indexedByDest.get(event.record.join()) ?? []
    const potentialClients = store.clientStore.clients.filter((client) => participants.every((participation => participation.from != client)))

    return <>
        <NavbarMenuEntry disabled={deleteState.submitted}>
            <a onClick={e => askSubmitDeleteEvent(event, deleteState, async () => {
                await router.replace(paths.events)
            })}>
                <i className="fa-solid fa-users-slash text-error"/>Cancel
            </a>
        </NavbarMenuEntry>
        <NavbarMenuEntry>
            <Link href={paths.event_edit(event.record.join())}>
                <a><i className="fa-solid fa-pencil-alt text-primary"/>Edit</a>
            </Link>
        </NavbarMenuEntry>
        <NavbarMenuEntry disabled={addParticipantState.submitted || !potentialClients.length}>
            <a onClick={e => askAddParticipantEvent(event, addParticipantState)}>
                <i className="fa-solid fa-user-plus"/>Participant
            </a>
        </NavbarMenuEntry>
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
