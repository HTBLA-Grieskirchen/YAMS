import dialog from "../../libs/dialog";
import notification from "../../libs/notification";
import Event from "../../model/event"
import { SubmissionState } from "../../libs/form/submit";
import Client from "../../model/client";
import {
    deleteEventParticipation,
    relateClientParticipateEvent,
    updateEventParticipation
} from "../../libs/database/eventParticipation";
import { ValidatableComboBox } from "../form/combobox";
import { Combobox } from "@headlessui/react";
import React, { useState } from "react";
import store, { useStore } from "../../stores";
import { observer, useLocalObservable } from "mobx-react";
import { ValidatableFieldData } from "../../libs/field/validatable";
import { clientSearched } from "../client";
import { ValidatableInputField } from "../form/input";
import EventParticipation from "../../model/participation";


export function askAddParticipantEvent(event: Event, state?: SubmissionState, callback?: () => Promise<void>) {
    const DialogComponent = observer((
        {close}:
            { close: () => void }
    ) => {
        const store = useStore()
        const participants = store.eventStore.participationStore.indexedByDest.get(event.record.join()) ?? []
        const potentialClients = store.clientStore.clients.filter((client) => participants.every((participation => participation.from != client)))

        const targetClient = useLocalObservable(() =>
            new ValidatableFieldData<Client | null>(null, (value) => {
                if (value != null) return null

                return "You have to select a client"
            }))
        const [targetClientQuery, setTargetClientQuery] = useState("")
        const filteredClients = targetClientQuery.trim().length < 1 ? potentialClients :
            potentialClients.filter((client) => {
                return clientSearched(client, targetClientQuery)
            })

        const price = useLocalObservable(() =>
            new ValidatableFieldData(event.seminar.price, (value) => {
                const isNumber = !Number.isNaN(value)
                if (isNumber && value < 0) {
                    return "Only positive values are allowed"
                } else if (!isNumber) {
                    return "Has to be a valid number"
                } else {
                    return null
                }
            }))

        return <form className="modal-box max-w-none w-fit overflow-visible" onSubmit={e => {
            e.preventDefault()
            if (!targetClient.value) return

            submitAddParticipantEvent(event, targetClient.value, price.value, state, callback).then((success) => success && close())
        }}>
            <h3 className="font-bold text-lg">
                Add new participant...
            </h3>
            <div className="flex grow space-x-4 py-4">
                <ValidatableComboBox data={targetClient} label="Client"
                                     placeholder="Max Mustermann, 01.02.2003"
                                     className="max-w-md basis-96"
                                     mapDisplayValue={(value: typeof targetClient.value) => {
                                         return value == null || !(value instanceof Client) ? "" :
                                             `${value.firstName} ${value.lastName}, ${value.birthdate.toLocaleDateString()}`
                                     }} setQuery={setTargetClientQuery}>
                    {filteredClients.map((item) =>
                        <Combobox.Option className={`${item == targetClient.value ? "active" : ""}`}
                                         key={item.record.join()} value={item}>
                            <a className="flex items-center">
                                <div
                                    className={`badge badge-xs ${item.consent ? "badge-success" : "badge-error"}`}></div>
                                <div className="flex flex-col">
                                    <p className="font-medium">{`${item.firstName} ${item.lastName}, ${item.birthdate.toLocaleDateString()}`}</p>
                                    <p>{item.mobileNumber}, <span className="text-primary">{item.email}</span></p>
                                </div>
                            </a>
                        </Combobox.Option>
                    )}
                </ValidatableComboBox>
                <ValidatableInputField data={price} label="Price" placeholder={event.seminar.price.toString()}
                                       className="basis-48 shrink grow-0" required
                                       mapDisplayValue={(num) => price.modified ? undefined : num.toString()}
                                       mapSetValue={(val) => val.trim().length < 1 ? Number.NaN : Number(val)}/>
            </div>
            <div className="modal-action">
                <button className="btn btn-error" type="button" onClick={e => close()}>Cancel</button>
                <button className={`btn btn-success ${!!state?.submitted ? "loading" : ""}`} type="submit"
                        disabled={!targetClient.valid || !price.valid || !!state?.submitted}>
                    Add Participant!
                </button>
            </div>
        </form>
    })

    dialog((close) => <DialogComponent close={close}/>)
}

export async function submitAddParticipantEvent(event: Event, client: Client, cost: number, state?: SubmissionState, callback?: () => Promise<void>) {
    state?.submit()

    const result = await relateClientParticipateEvent(client.record, event.record, cost.toString())
    if (result.error) {
        notification.error({
            title: "Participant could not be added!",
            message: `"${result.error.message}". Do you want to try again?`
        }, 15, {
            "Retry": {
                action: async () => {
                    await submitAddParticipantEvent(event, client, cost, state)
                    return true
                },
                disabled: () => state?.submitted ?? false
            }
        })
    } else {
        await store.eventStore.participationStore.refresh()
        callback && await callback()
    }

    state?.clear()

    return !result.error
}

export function askEditEventParticipation(participation: EventParticipation, state?: SubmissionState, callback?: () => Promise<void>) {
    const DialogComponent = observer((
        {close}:
            { close: () => void }
    ) => {
        const event = participation.dest
        const client = participation.from

        const price = useLocalObservable(() =>
            new ValidatableFieldData(participation.cost, (value) => {
                const isNumber = !Number.isNaN(value)
                if (isNumber && value < 0) {
                    return "Only positive values are allowed"
                } else if (!isNumber) {
                    return "Has to be a valid number"
                } else {
                    return null
                }
            }))

        return <form className="modal-box max-w-none w-fit overflow-visible" onSubmit={e => {
            e.preventDefault()
            submitUpdateEventParticipation(participation, price.value, state, callback).then((success) => success && close())
        }}>
            <h3 className="font-bold text-lg">
                Change participation...
            </h3>
            <div className="flex grow space-x-4 py-4">
                <div className={`form-control w-full max-w-md basis-96`}>
                    <label className="label">
                        <span className="label-text">Client</span>
                    </label>
                    <div className="indicator w-full">
                        <p className={`ml-1 mt-2 text-lg font-bold w-full pointer-events-none`}>
                            {`${client.firstName} ${client.lastName}, ${client.birthdate.toLocaleDateString()}`}
                        </p>
                    </div>
                </div>
                <ValidatableInputField data={price} label="Price" placeholder={event.seminar.price.toString()}
                                       className="basis-48 shrink grow-0" required
                                       mapDisplayValue={(num) => price.modified ? undefined : num.toString()}
                                       mapSetValue={(val) => val.trim().length < 1 ? Number.NaN : Number(val)}/>
            </div>
            <div className="modal-action">
                <button className="btn btn-error" type="button" onClick={e => close()}>Cancel</button>
                <button className={`btn btn-success ${!!state?.submitted ? "loading" : ""}`} type="submit"
                        disabled={!price.valid || !!state?.submitted}>
                    Submit!
                </button>
            </div>
        </form>
    })

    dialog((close) => <DialogComponent close={close}/>)
}

export async function submitUpdateEventParticipation(participation: EventParticipation, cost: number, state?: SubmissionState, callback?: () => Promise<void>) {
    state?.submit()

    const result = await updateEventParticipation(participation, cost.toString())
    if (result.error) {
        notification.error({
            title: "Participation could not be changed!",
            message: `"${result.error.message}". Do you want to try again?`
        }, 15, {
            "Retry": {
                action: async () => {
                    await submitUpdateEventParticipation(participation, cost, state)
                    return true
                },
                disabled: () => state?.submitted ?? false
            }
        })
    } else {
        await store.eventStore.participationStore.refresh()
        callback && await callback()
    }

    state?.clear()

    return !result.error
}

export async function submitDeleteEventParticipation(participation: EventParticipation, state?: SubmissionState, callback?: () => Promise<void>) {
    state?.submit()

    const result = await deleteEventParticipation(participation.record)
    if (result.error) {
        notification.error({
            title: "Participant could not be removed!",
            message: `"${result.error.message}". Do you want to try again?`
        }, 15, {
            "Retry": {
                action: async () => {
                    await submitDeleteEventParticipation(participation, state)
                    return true
                },
                disabled: () => state?.submitted ?? false
            }
        })
    } else {
        await store.eventStore.participationStore.refresh()
        callback && await callback()
    }

    state?.clear()

    return !result.error
}
