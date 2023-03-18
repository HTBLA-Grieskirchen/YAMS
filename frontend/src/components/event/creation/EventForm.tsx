import React, {useState} from "react"
import {observer, useLocalObservable} from "mobx-react";
import Address from "../../../model/address";
import {useRouter} from "next/router";
import paths from "../../../util/paths";
import {useStore} from "../../../stores";
import {ValidatableFieldData} from "../../../libs/field/validatable";
import {isValidTime} from "../../../util/validation";
import dialog from "../../../libs/dialog";
import notification from "../../../libs/notification";
import {ValidatableInputField} from "../../form/input";
import {Combobox} from "@headlessui/react";
import {ValidatableComboBox} from "../../form/combobox";
import RegisterAddressForm, {clientRegisterAddressFieldData, NewAddressFormData} from "../../address/RegisterAddress";
import {query} from "../../../libs/database";
import {makeRecordForTable, Record} from "../../../model/surreal";
import {ensureAddress} from "../../../libs/database/address";
import Event from "../../../model/event"
import RegisterSeminarForm, {defaultSeminarFormData, NewSeminarFormData} from "../../seminar/RegisterSeminar";
import Seminar from "../../../model/seminar";
import {formatDuration} from "../../../util/helpers";
import {ensureSeminar} from "../../../libs/database/seminar";
import {createEvent, updateEvent} from "../../../libs/database/event";

const EventForm = observer((
    {event}:
        { event?: Event }
) => {
    const router = useRouter()
    const store = useStore()
    const language = store.settingsStore.language
    const addresses = store.addressStore.addresses
    const seminars = store.eventStore.seminars

    const form = useLocalObservable(() => ({
        submitted: false,
        setSubmitted(submitted: boolean) {
            this.submitted = submitted
        }
    }))

    const date = useLocalObservable(() =>
        new ValidatableFieldData(event?.date.toISOString().split("T")[0] ?? "", (value) => {
            if (Number.isNaN(Date.parse(value))) {
                return "Date has to be valid"
            } else {
                return null
            }
        }))
    const time = useLocalObservable(() =>
        new ValidatableFieldData(
            event?.date.toISOString().split("T")[1].split(/[Z.]/)[0].split(":").slice(0, 2).join(":") ?? "",
            (value) => {
                if (!isValidTime(value)) {
                    return "Time has to be valid"
                } else {
                    return null
                }
            }))
    const locationName = useLocalObservable(() =>
        new ValidatableFieldData<string>(event?.locationName ?? "", (value) => {
            return null
        }))
    const maxParticipants = useLocalObservable(() =>
        new ValidatableFieldData<number | null>(event?.maxParticipants ?? null, (value) => {
            const isNumber = !Number.isNaN(value) && value != null
            if (isNumber && value < 0) {
                return "Only positive values are allowed"
            } else if (isNumber && !Number.isInteger(value)) {
                return "Only whole numbers are allowed"
            } else if (!isNumber && value != null) {
                return "Has to be a valid number"
            } else {
                return null
            }
        }))

    const location = useLocalObservable(() =>
        new ValidatableFieldData<Address | NewAddressFormData | null>(event?.location ?? null, (value) => {
            if (value == null) {
                return "You have to assign a location for this event"
            } else {
                return null
            }
        }))

    const seminar = useLocalObservable(() =>
        new ValidatableFieldData<Seminar | NewSeminarFormData | null>(event?.seminar ?? null, (value) => {
            if (value == null) {
                return "You have to assign a seminar for this event"
            } else {
                return null
            }
        }))

    function anyMutated() {
        let mutated = [seminar, location, locationName, maxParticipants, date, time].some((data) => data.modified)
        mutated = mutated || location.value == newAddress && Object.values(newAddress).some((data) => data.modified)
        mutated = mutated || location.value == newAddress && Object.values(newAddress).some((data) => data.modified)
        return mutated
    }

    function allValid() {
        let valid = [seminar, location, locationName, maxParticipants, date, time].every((data) => data.valid)
        valid = valid && (location.value != newAddress || Object.values(newAddress).every((data) => data.valid))
        valid = valid && (seminar.value != newSeminar || Object.values(newSeminar).every((data) => data.valid))
        return valid
    }

    const newAddress = useLocalObservable(() => clientRegisterAddressFieldData)
    const [addressSearchQuery, setAddressSearchQuery] = useState("")
    const filteredAddresses = addressSearchQuery.trim().length < 1 ? addresses :
        addresses.filter((address) => {
            return addressSearchQuery.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
                const trimmedSegment = segment.trim().toLowerCase()
                if (trimmedSegment.length < 0) return true

                return address.city.toLowerCase().includes(trimmedSegment) ||
                    address.postalCode.toLowerCase().includes(trimmedSegment) ||
                    address.street.toLowerCase().includes(trimmedSegment) ||
                    address.streetNumber.toLowerCase().includes(trimmedSegment) ||
                    (address.extra?.toLowerCase()?.includes(trimmedSegment) ?? false) ||
                    address.country.toLowerCase().includes(trimmedSegment)
            })
        })

    const newSeminar = useLocalObservable(() => defaultSeminarFormData())
    const [seminarSearchQuery, setSeminarSearchQuery] = useState("")
    const filteredSeminars = seminarSearchQuery.trim().length < 1 ? seminars :
        seminars.filter((seminar) => {
            return seminarSearchQuery.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
                const trimmedSegment = segment.trim().toLowerCase()
                if (trimmedSegment.length < 0) return true

                return seminar.title.toLowerCase().includes(trimmedSegment) ||
                    seminar.price.toString().toLowerCase().includes(trimmedSegment) ||
                    formatDuration(seminar.duration ?? 0, language).toLowerCase().includes(trimmedSegment)
            })
        })

    function onBack() {
        if (anyMutated()) {
            dialog((close) => <div className="modal-box">
                <h3 className="font-bold text-lg">
                    <i className="fa-solid fa-exclamation-triangle text-warning mr-1"/>
                    This action will discard all changes!
                </h3>
                <p className="py-4">Are you sure you want to abort the event {!event ? "creation" : "modification"}
                    and return to the previous view? All data will be lost!</p>
                <div className="modal-action">
                    <button className="btn btn-neutral" onClick={e => close()}>Stay here</button>
                    <button className="btn btn-primary" onClick={e => {
                        close()
                        router.back()
                    }}>
                        Leave event {!event ? "creation" : "editing"}!
                    </button>
                </div>
            </div>)
        } else {
            router.back()
        }
    }

    function submit() {
        const data = {
            target: event,
            formData: {
                date: new Date(`${date.value}T${time.value}`),
                locationName: locationName.value,
                maxParticipants: maxParticipants.value,
                location: location.value instanceof Address ? location.value : {
                    extra: location.value!.extra.value.trim(),
                    street: location.value!.street.value.trim(),
                    streetNumber: location.value!.streetNumber.value.trim(),
                    postalCode: location.value!.zip.value.trim(),
                    city: location.value!.city.value.trim(),
                    country: location.value!.country.value.trim()
                },
                seminar: seminar.value instanceof Seminar ? seminar.value : {
                    title: seminar.value!.title.value.trim(),
                    price: seminar.value!.price.value,
                    duration: seminar.value!.duration.value
                }
            }
        }

        async function transact() {
            form.setSubmitted(true)

            try {
                await query("BEGIN TRANSACTION")

                let addressRecord: Record
                if (data.formData.location instanceof Address) {
                    addressRecord = data.formData.location.record
                } else {
                    addressRecord = await ensureAddress(data.formData.location)
                }

                let seminarRecord: Record
                if (data.formData.seminar instanceof Seminar) {
                    seminarRecord = data.formData.seminar.record
                } else {
                    seminarRecord = await ensureSeminar(data.formData.seminar)
                }

                let response = !!data.target ?
                    await updateEvent(
                        data.target.record,
                        data.formData.maxParticipants,
                        data.formData.date,
                        data.formData.locationName,
                        addressRecord,
                        seminarRecord
                    ) :
                    await createEvent(
                        data.formData.maxParticipants,
                        data.formData.date,
                        data.formData.locationName,
                        addressRecord,
                        seminarRecord
                    )

                if (response.error) {
                    throw response.error
                }

                const clientRecord = makeRecordForTable(response.result[0].id, Event.TABLE)


                await query("COMMIT TRANSACTION")
                await store.eventStore.refresh()
                await router.push(paths.event(clientRecord.join()))
            } catch (error) {
                await query("CANCEL TRANSACTION")

                let errorMessage
                if (typeof error === "string") {
                    errorMessage = error
                } else if (error instanceof Error) {
                    errorMessage = error.message
                }

                notification.error({
                    title: `Client cannot be ${!data.target ? "created" : "changed"}!`,
                    message: `"${errorMessage}". Do you want to try again?`
                }, 15, {
                    "Retry": {
                        action: async () => {
                            await transact()
                            return true
                        },
                        disabled: () => form.submitted
                    }
                })
            }

            form.setSubmitted(false)
        }

        if (anyMutated()) {
            transact().then()
        } else {
            router.back()
        }
    }

    return <>
        <form id="event-creation-form" onSubmit={e => {
            e.preventDefault()
            submit()
        }}>
            <div className="flex justify-between space-x-4">
                <ValidatableComboBox data={seminar} label="Seminar"
                                     placeholder="Main Seminar - 2h"
                                     required className="max-w-md"
                                     newValue={{data: newSeminar, prompt: "Define new seminar"}}
                                     mapDisplayValue={(value) => {
                                         return !(value instanceof Seminar) ? "" :
                                             `${value.title}${value.duration != null ? ` - ${formatDuration(value.duration, language)}` : ""}`
                                     }} setQuery={setSeminarSearchQuery}>
                    {filteredSeminars.map((item) =>
                        <Combobox.Option key={item.record.join()} value={item}>
                            <a className={`${item == seminar.value ? "active" : ""}`}>
                                <span className="font-medium">{item.title}</span>
                                {!!item.duration &&
                                    <><i
                                        className="fa-solid fa-clock -mr-1"/>{formatDuration(item.duration, language)}</>}
                            </a>
                        </Combobox.Option>
                    )}
                </ValidatableComboBox>

                <ValidatableInputField data={date} label="Date" placeholder="2001-01-01"
                                       required className="basis-96 shrink grow-0"
                                       type="date"/>
                <ValidatableInputField data={time} label="Time" placeholder="10:00"
                                       required className="basis-96 shrink grow-0"
                                       type="time"/>
                <ValidatableInputField data={maxParticipants} label="Max allowed participants" placeholder="15"
                                       className="basis-96 shrink grow-0"
                                       mapDisplayValue={(val) => val == null ? "" : val.toString()}
                                       mapSetValue={(val) => {
                                           const value = val.trim().length <= 0 ? null : Number.parseInt(val)
                                           return Number.isNaN(value) ? maxParticipants.value : value
                                       }}/>
            </div>

            {seminar.value == newSeminar && <RegisterSeminarForm addressData={newSeminar}/>}

            <div className="flex space-x-4">
                <ValidatableInputField data={locationName} label="Location name"
                                       placeholder="Medical University Linz" className="basis-96 shrink grow-0"/>

                <ValidatableComboBox data={location} label="Address"
                                     placeholder="Musterstraße 12, 3456 Maxhausen, Austria"
                                     required className="max-w-md"
                                     newValue={{data: newAddress, prompt: "Create new address"}}
                                     mapDisplayValue={(value: typeof location.value) => {
                                         return !(value instanceof Address) ? "" :
                                             `${value.street} ${value.streetNumber}${value.extra ? ` (${value.extra})` : ""}, ` +
                                             `${value.postalCode} ${value.city}, ${value.country}`
                                     }} setQuery={setAddressSearchQuery}>
                    {filteredAddresses.map((item) =>
                        <Combobox.Option key={item.record.join()} value={item}>
                            <a className={`${item == location.value ? "active" : ""}`}>
                                {`${item.street} ${item.streetNumber}${item.extra ? ` (${item.extra})` : ""}, ` +
                                    `${item.postalCode} ${item.city}, ${item.country}`}
                            </a>
                        </Combobox.Option>
                    )}
                </ValidatableComboBox>
            </div>

            {location.value == newAddress && <RegisterAddressForm addressData={newAddress}/>}
        </form>

        <div className="card-actions justify-end">
            <button className="btn btn-error" type="button" onClick={e => onBack()}>
                <i className="fa-solid fa-circle-left mr-2"/>Back
            </button>
            <button className={`btn btn-success ${form.submitted ? "loading" : ""}`} type="submit"
                    form="event-creation-form" disabled={!allValid()}>
                {!form.submitted &&
                    <i className={`fa-solid ${!event ? "fa-calendar-plus" : "fa-pen-to-square"} mr-2`}/>}
                {!event ? "Create" : "Change"}
            </button>
        </div>
    </>
})

export default EventForm
