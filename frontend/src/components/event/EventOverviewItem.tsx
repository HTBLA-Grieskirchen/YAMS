import {observer, useLocalObservable} from "mobx-react"
import Event from "../../model/event"
import {useStore} from "../../stores";
import {capitalize, formatDuration} from "../../util/helpers";
import dialog from "../../libs/dialog";
import notification from "../../libs/notification";
import {deleteEvent} from "../../libs/database/event";
import Link from "next/link";
import paths from "../../util/paths";

const EventOverviewItem = observer((
    {event}:
        { event: Event }
) => {
    const store = useStore()
    const language = store.settingsStore.language

    const timeRemaining = event.date.valueOf() - Date.now()
    const dayDifference = Math.round(
        new Date(event.date).setUTCHours(0, 0, 0) - new Date().setUTCHours(0, 0, 0)
    ) / (1000 * 3600 * 24)

    const dateFormat = Math.abs(dayDifference) > 2 ?
        event.date.toLocaleDateString(language, {dateStyle: "long"}) :
        capitalize(new Intl.RelativeTimeFormat(language, {numeric: "auto"}).format(dayDifference, "day"))

    const deleteState = useLocalObservable(() => ({
        submitted: false,
        submit() {
            this.submitted = true
        },
        clear() {
            this.submitted = false
        }
    }))

    function askSubmitDelete() {
        dialog((close) => <div className="modal-box">
            <h3 className="font-bold text-lg">
                <i className="fa-solid fa-exclamation-triangle text-warning mr-1"/>
                This action will cancel this event!
            </h3>
            <p className="py-4">
                Are you sure you want to cancel <span className="font-bold">{event.seminar.title}</span>?
                This action cannot be undone.
            </p>
            <div className="modal-action">
                <button className="btn btn-error" onClick={e => close()}>Cancel</button>
                <button className="btn btn-success" onClick={e => {
                    close()
                    submitDelete().then()
                }}>{"I'm sure!"}
                </button>
            </div>
        </div>)
    }

    async function submitDelete() {
        deleteState.submit()

        const result = await deleteEvent(event.record)
        if (result.error) {
            notification.error({
                title: "Event could not be cancelled!",
                message: `"${result.error.message}". Do you want to try again?`
            }, 15, {
                "Retry": {
                    action: async () => {
                        await submitDelete()
                        return true
                    },
                    disabled: () => deleteState.submitted
                }
            })
        } else {
            await store.eventStore.refresh()
        }

        deleteState.clear()
    }

    // TODO: Add participant counting
    return <div className="py-4 first:pt-0 last:pb-0 flex flex-row justify-between group">
        <div>
            <h3 className="mb-3">
                <span className="text-lg font-medium">{event.seminar.title}</span>
                <span className="mx-4 text-base-content/75">
                    <i className="fa-solid fa-people-group mr-2"/>
                    0 {event.maxParticipants != null && ` of ${event.maxParticipants}`}
                </span>
                {timeRemaining < -(event.seminar.duration ?? 0) ?
                    <span className="badge">Past</span> :
                    timeRemaining < 0 ?
                        <span className="badge badge-error">Now</span> :
                        timeRemaining < 1000 * 3600 * 24 * 7 ?
                            <span className="badge badge-warning">Upcoming</span> :
                            <span className="badge badge-success">Future</span>}
            </h3>
            <div className="text-base-content/75 flex flex-row">
            <span>
                <i className="fa-solid fa-calendar mr-2"/>
                {dateFormat} at {event.date.toLocaleTimeString(language, {timeStyle: "short"})}
                {!!event.seminar.duration &&
                    <span className="ml-4 text-base-content/75">
                    <i className="fa-solid fa-clock mr-2"/>{formatDuration(event.seminar.duration, language)}
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
                <li><a onClick={e => askSubmitDelete()}
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
