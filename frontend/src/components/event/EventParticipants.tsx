import { observer, useLocalObservable } from "mobx-react";
import Event from "../../model/event"
import { SmallSearchField } from "../form/search";
import React from "react";
import { useStore } from "../../stores";
import ClientParticipation from "../participation/ClientParticipation";
import EventParticipation from "../../model/participation";

const EventParticipants = observer((
    {event}:
        { event: Event }
) => {
    const store = useStore()

    const participantFilter = useLocalObservable(() => ({
        value: "",
        setValue(value: string) {
            this.value = value
        }
    }))
    const participants = store.eventStore.participationStore.indexedByDest.get(event.record.join()) ?? []
    const filteredParticipants = participants.filter((participation) => participationSearchedClient(participation, participantFilter.value))

    return <div className="flex w-full overflow-x-auto">
        <div className="card card-compact bg-base-100 shadow p-2 max-h-full w-full">
            <div className="card-body overflow-visible overflow-x-auto">
                <div className="flex flex-col lg:flex-row justify-between space-y-2 lg:space-y-0">
                    <h2 className="card-title">Participants</h2>
                    <SmallSearchField value={participantFilter.value}
                                      onChange={(value) => participantFilter.setValue(value)}/>
                </div>

                <div className="divider my-0"/>

                <div className="flex flex-col divide-y overflow-y-auto">
                    {filteredParticipants.map((participation) =>
                        <ClientParticipation key={participation.record.join()} participation={participation}/>)}
                </div>
            </div>
        </div>
    </div>
})
export default EventParticipants

function participationSearchedClient(participation: EventParticipation, query: string) {
    const client = participation.from

    return query.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
        const trimmedSegment = segment.trim().toLowerCase()
        if (trimmedSegment.length < 0) return true

        return client.firstName.toLowerCase().includes(trimmedSegment) ||
            client.lastName.toLowerCase().includes(trimmedSegment) ||
            client.email.toLowerCase().includes(trimmedSegment) ||
            client.mobileNumber.toLowerCase().includes(trimmedSegment) ||
            client.customerNumber.toString().toLowerCase().includes(trimmedSegment) ||
            client.birthdate.getFullYear().toString().toLowerCase().includes(trimmedSegment) ||
            participation.cost.toString().toLowerCase().includes(trimmedSegment)
    })
}
