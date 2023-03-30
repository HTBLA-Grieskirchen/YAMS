import { observer } from "mobx-react"
import Event from "../../model/event"
import { useStore } from "../../stores";
import { capitalize, formatDuration } from "../../util/helpers";

const EventDetailItem = observer((
    {event}:
        { event: Event }
) => {
    const store = useStore()
    const language = store.settingsStore.language

    const dayDifference = event.daysFromNow

    const dateFormat = Math.abs(dayDifference) > 2 ?
        event.date.toLocaleDateString(language, {dateStyle: "long"}) :
        capitalize(new Intl.RelativeTimeFormat(language, {numeric: "auto"}).format(dayDifference, "day"))

    const participants = store.eventStore.participationStore.indexedByDest.get(event.record.join()) ?? []

    return <div className="w-full flex flex-col md:flex-row justify-between">
        <div className="flex-col">
            <div>
                <i className="fa-solid fa-calendar mr-2"/>
                {dateFormat} at {event.date.toLocaleTimeString(language, {timeStyle: "short"})}
            </div>
            {!!event.seminar.duration &&
                <div className="">
                    <i className="fa-solid fa-hourglass-end mr-2"/>{formatDuration(event.seminar.duration, language)}
                </div>}
            <div>
                <i className="fa-solid fa-people-group mr-2"/>
                {participants.length} {event.maxParticipants != null && ` of ${event.maxParticipants}`}
            </div>
            <div>
                <i className="fa-solid fa-dollar-sign mr-2"/>
                {participants.map((participation) => participation.cost).reduce((prev, curr) => prev + curr, 0)}
            </div>
        </div>
        <div className="flex-col">
            <div className="md:text-right">
                <i className="fa-solid fa-location-dot mr-2"/>
                {!!event.locationName && `${event.locationName}`}
            </div>
            <div className="md:text-right">
                {`${event.location.street} ${event.location.streetNumber}`}{!!event.location.extra && `, ${event.location.extra}`}
            </div>
            <div className="md:text-right">
                {`${event.location.postalCode}, ${event.location.city}`}
            </div>
            <div className="md:text-right">
                {`${event.location.country}`}
            </div>
        </div>
    </div>
})

export default EventDetailItem
