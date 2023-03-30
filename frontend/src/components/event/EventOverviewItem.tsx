import {observer} from "mobx-react"
import Event from "../../model/event"
import {useStore} from "../../stores";
import {capitalize, formatDuration} from "../../util/helpers";
import Link from "next/link";
import paths from "../../util/paths";
import {askSubmitDeleteEvent} from "./index";
import {useSubmissionState} from "../../libs/form/submit";

const EventOverviewItem = observer((
    {event}:
        { event: Event }
) => {
    const store = useStore()
    const language = store.settingsStore.language

    const eventWhen = event.when
    const dayDifference = event.daysFromNow

    const dateFormat = Math.abs(dayDifference) > 2 ?
        event.date.toLocaleDateString(language, {dateStyle: "long"}) :
        capitalize(new Intl.RelativeTimeFormat(language, {numeric: "auto"}).format(dayDifference, "day"))

    const deleteState = useSubmissionState()

    const participants = store.eventStore.participationStore.indexedByDest.get(event.record.join()) ?? []

    return <div className="py-4 first:pt-0 last:pb-0 flex flex-row justify-between group">
        <div>
            <h3 className="mb-3">
                <span className="text-lg font-medium">{event.seminar.title}</span>
                <span className="mx-4 text-base-content/75">
                    <i className="fa-solid fa-people-group mr-2"/>
                    {participants.length} {event.maxParticipants != null && ` of ${event.maxParticipants}`}
                </span>
                {eventWhen <= 0 ?
                    <span className="badge">Past</span> :
                    eventWhen <= 1 ?
                        <span className="badge badge-error">Now</span> :
                        eventWhen <= 2 ?
                            <span className="badge badge-warning">Upcoming</span> :
                            <span className="badge badge-success">Future</span>}
            </h3>
            <div className="text-base-content/75 flex flex-row">
            <span>
                <i className="fa-solid fa-calendar mr-2"/>
                {dateFormat} at {event.date.toLocaleTimeString(language, {timeStyle: "short"})}
                {!!event.seminar.duration &&
                    <span className="ml-4 text-base-content/75">
                        <i className="fa-solid fa-hourglass-end mr-2"/>{formatDuration(event.seminar.duration, language)}
                    </span>}
            </span>
                <div className="divider divider-horizontal mx-3"/>
                <span>
                <i className="fa-solid fa-location-dot mr-2"/>
                    {!!event.locationName ? event.locationName : `${event.location.street} ${event.location.streetNumber}`}
            </span>
            </div>
        </div>
        <div className="self-center dropdown dropdown-left group-last:dropdown-end group-last:dropdown-left">
            <label tabIndex={0} className="btn btn-sm btn-ghost m-1"><i
                className="text-lg fa-solid fa-ellipsis"/></label>
            <ul tabIndex={0} className="dropdown-content menu menu-compact shadow bg-base-100 rounded-box w-52">
                <li>
                    <Link href={paths.event(event.record.join())}>
                        <a>
                            <i className="fa-solid fa-magnifying-glass text-primary"/>
                            Detail
                        </a>
                    </Link>
                </li>
                <li>
                    <Link href={paths.event_edit(event.record.join())}>
                        <a>
                            <i className="fa-solid fa-pen"/>
                            Edit
                        </a>
                    </Link>
                </li>
                <li><a onClick={e => askSubmitDeleteEvent(event, deleteState)}
                       className={!deleteState.submitted ? "" : "pointer-events-none bg-base-200/50 opacity-50"}>
                    {deleteState.submitted ?
                        <i className="fa-solid fa-circle-notch animate-spin"/> :
                        <i className="fa-solid fa-trash-can text-error"/>}
                    Delete
                </a></li>
            </ul>
        </div>
    </div>
})

export default EventOverviewItem
