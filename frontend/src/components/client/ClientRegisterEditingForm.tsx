import React, {useState} from "react";
import Address from "../../model/address";
import {query} from "../../libs/database";
import Select from "react-select";
import {observer, useLocalObservable} from "mobx-react";
import Client from "../../model/client";
import {ValidatableInputField} from "../form/input";
import {ValidatableComboBox} from "../form/combobox";
import {Combobox} from "@headlessui/react";
import {useRouter} from "next/router";
import {useStore} from "../../stores";
import {ValidatableFieldData} from "../../libs/field/validatable";
import {isValidEmail, isValidMobilenumber} from "../../util/validation";
import ClientRegisterAddressForm, {
    clientRegisterAddressDataFromAddress,
    emptyClientRegisterAddressFieldData,
    NewClientRegisterAddress
} from "./register/ClientRegisterAddress";
import dialog from "../../libs/dialog";
import {Result} from "surrealdb.js";
import notification from "../../libs/notification";
import {makeRecordForTable, Record} from "../../model/surreal";
import {createAddress} from "../../libs/database/address";
import {createClient} from "../../libs/database/client";
import paths from "../../util/paths";

const EditClientForm = observer(({client}: {client: Client}) => {
    const router = useRouter()
    const store = useStore()
    const addresses = store.addressStore.addresses

    const form = useLocalObservable(() => ({
        submitted: false,
        setSubmitted(submitted: boolean) {
            this.submitted = submitted
        }
    }))

    const firstname = useLocalObservable(() =>
        new ValidatableFieldData<string>(client.firstName, (value: string) => {
            if (value.trim().length < 1) {
                return "Firstname may not be empty"
            } else {
                return null
            }
        }))

    const lastname = useLocalObservable(() =>
        new ValidatableFieldData<string>(client.lastName, (value: string) => {
            if (value.trim().length < 1) {
                return "Lastname may not be empty"
            } else {
                return null
            }
        }))

    const email = useLocalObservable(() =>
        new ValidatableFieldData<string>(client.email, (value: string) => {
            if (value.trim().length < 1) {
                return "Email may not be empty"
            } else if (!isValidEmail(value.trim())) {
                return "This is not a valid email"
            } else {
                return null
            }
        }))

    const mobilenumber = useLocalObservable(() =>
        new ValidatableFieldData<string>(client.mobileNumber, (value: string) => {
            if (value.trim().length < 1) {
                return "Mobile number may not be empty"
            } else if (!isValidMobilenumber(value.trim())) {
                return "Invalid mobile number"
            } else {
                return null
            }
        }))

    const birthdate = useLocalObservable(() =>
        new ValidatableFieldData<Date | null>(client.birthdate, (value) => {
            if (value == null) {
                return "Birthdate has to be picked"
            } else {
                return null
            }
        }))

    const consent = useLocalObservable(() => ({
        value: false,
        setValue(value: boolean) {
            this.value = value
        }
    }))

    const address = useLocalObservable(() =>
        new ValidatableFieldData<Address | NewClientRegisterAddress | null>(clientRegisterAddressDataFromAddress(client.address), (value) => {
            if (value == null) {
                return "You have to assign an address"
            } else {
                return null
            }
        }))

    const newAddress = useLocalObservable(() => emptyClientRegisterAddressFieldData)

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

    const anyMutated = () => {
        let mutated = [firstname, lastname, mobilenumber, birthdate, email].some((data) => data.modified)
        mutated = mutated || address.value == newAddress && Object.values(newAddress).some((data) => data.modified)
        return mutated
    }

    const allValid = () => {
        let valid = [firstname, lastname, mobilenumber, birthdate, email, address].every((data) => data.valid)
        valid = valid && (address.value != newAddress || Object.values(newAddress).every((data) => data.valid))
        return valid
    }

    const onBack = () => {
        if (anyMutated()) {
            dialog((close) => <div className="modal-box">
                <h3 className="font-bold text-lg">
                    <i className="fa-solid fa-exclamation-triangle text-warning mr-1"/>
                    This action will discard all changes!
                </h3>
                <p className="py-4">Are you sure you want to abort the client creation and return to the previous view?
                    All data will be lost!</p>
                <div className="modal-action">
                    <button className="btn btn-neutral" onClick={e => close()}>Stay here</button>
                    <button className="btn btn-primary" onClick={e => {
                        close()
                        router.back()
                    }}>
                        {"Leave client creation!"}
                    </button>
                </div>
            </div>)
        } else {
            router.back()
        }
    }

    async function submit() {
        form.setSubmitted(true)

        let response: Result<any>
        const abortWithError = async (errorMessage: string) => {
            notification.error({
                title: "Client cannot be updated+!",
                message: `"${errorMessage}". Do you want to try again?`
            }, 15, {
                "Retry": {
                    action: async () => {
                        await submit()
                        return true
                    },
                    disabled: () => form.submitted
                }
            })

            await query("CANCEL TRANSACTION")
            form.setSubmitted(false)
        }

        await query("BEGIN TRANSACTION")

        let addressRecord: Record
        if (address.value == newAddress) {
            const potentialDuplicate = store.addressStore.addresses.find((item) => {
                return (item.extra ?? "") == newAddress.extra.value.trim() &&
                    item.street == newAddress.street.value.trim() &&
                    item.streetNumber == newAddress.streetNumber.value.trim() &&
                    item.postalCode == newAddress.zip.value.trim() &&
                    item.city == newAddress.city.value.trim() &&
                    item.country == newAddress.country.value.trim()
            })

            if (potentialDuplicate !== undefined) {
                addressRecord = potentialDuplicate.record
            } else {
                response = await createAddress(
                    newAddress.street.value.trim(),
                    newAddress.streetNumber.value.trim(),
                    newAddress.extra.value.trim(),
                    newAddress.zip.value.trim(),
                    newAddress.city.value.trim(),
                    newAddress.country.value.trim()
                )

                if (response.error) {
                    await abortWithError(response.error.message)
                    return
                }

                try {
                    addressRecord = makeRecordForTable(response.result[0].id, Address.TABLE)
                } catch (error: unknown) {
                    if (typeof error === "string") {
                        await abortWithError(error)
                    } else if (error instanceof Error) {
                        await abortWithError(error.message)
                    }
                    return
                }
            }
        } else {
            addressRecord = (address.value as Address).record
        }

        response = await createClient(
            firstname.value.trim(),
            lastname.value.trim(),
            email.value.trim(),
            mobilenumber.value.trim(),
            birthdate.value as Date,
            consent.value,
            addressRecord
        )

        if (response.error) {
            await abortWithError(response.error.message)
            return
        }

        await query("COMMIT TRANSACTION")
        await store.clientStore.refresh()
        await router.push(paths.clients)
        form.setSubmitted(false)
    }

    const handleButtonEditClient = async () => {

        let result = await query('UPDATE type::thing($ClientTable, $ClientID) SET first_name = $firstname, last_name = $lastname, ' +
        'email = $email, street_number=$streetNumber, postal_code=$postalCode, city = $city, street=$street, date=$birthdate, consent=$consent' ,
            {

                clientTable: client.record.table,
                clientID: client.record.id,
                firstname: firstname,
                lastname: lastname,
                email: email

            }
        )
    }

    return <>
        <form>
            <div className="flex justify-between space-x-4">
                <ValidatableInputField data={firstname} label="Firstname" required/>
                <ValidatableInputField data={lastname} label="Lastname" required/>

                <ValidatableInputField data={birthdate} label="Birthdate"
                                       required className="basis-96 shrink grow-0"
                                       type="date" mapDisplayValue={(value) => value == null ? "" : value.toISOString().split("T")[0]}
                                       mapSetValue={(value) => {
                                           console.log("Setted Birthdate: ", value)
                                           const [year, month, day] = value.split("-").map(Number)
                                           return new Date(year, month - 1, day + 1)
                                       }}/>
            </div>

            <div className="flex justify-between space-x-4">
                <ValidatableInputField data={mobilenumber} label="Mobile Number" required type="tel"/>
                <ValidatableInputField data={email} label="Email" required type="email"/>

                <div className="form-control grow my-auto">
                    <label className="label cursor-pointer w-full">
                        <span className="label-text mr-4 ml-2">Signed Consent</span>
                        <input type="checkbox" checked={consent.value}
                               onChange={e => consent.setValue(e.target.checked)}
                               className="checkbox checkbox-primary"/>
                    </label>
                </div>
            </div>

            <div className="flex justify-between space-x-4">
                <ValidatableComboBox data={address} label="Address"
                                     required className="max-w-md"
                                     newValue={{data: newAddress, prompt: "Create new address"}}
                                     mapDisplayValue={(value: typeof address.value) => {
                                         return value == null || !(value instanceof Address) ? (value != newAddress ?
                                             `${client.address.street} ${client.address.streetNumber}${client.address.extra ? ` (${client.address.extra})` : ""}, ` +
                                             `${client.address.postalCode} ${client.address.city}, ${client.address.country}`   : "") :
                                             `${value.street} ${value.streetNumber}${value.extra ? ` (${value.extra})` : ""}, ` +
                                             `${value.postalCode} ${value.city}, ${value.country}`
                                     }} setQuery={setAddressSearchQuery}>
                    {filteredAddresses.map((item) =>
                        <Combobox.Option className={`${item == address.value ? "active" : ""}`}
                                         key={item.record.join()} value={item}>
                            <a>{`${item.street} ${item.streetNumber}${item.extra ? ` (${item.extra})` : ""}, ` +
                                `${item.postalCode} ${item.city}, ${item.country}`}</a>
                        </Combobox.Option>
                    )}
                </ValidatableComboBox>
            </div>
            {address.value != null && !(address.value instanceof Address) && <ClientRegisterAddressForm addressData={address.value}/>}
        </form>

        <div className="card-actions justify-end">
            <button className="btn btn-error" type="button" onClick={e => onBack()}>
                <i className="fa-solid fa-circle-left mr-2"/>Back
            </button>

            <button className={`btn btn-success ${form.submitted ? "loading" : ""}`} type="submit"
                    disabled={!allValid()}>
                {!form.submitted && <i className="fa-solid fa-user-plus mr-2"/>}Update
            </button>
        </div>
    </>
})

export default EditClientForm