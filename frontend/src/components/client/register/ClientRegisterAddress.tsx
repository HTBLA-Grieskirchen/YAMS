import React from "react";
import {observer} from "mobx-react";
import {ValidatableFieldData} from "../../../libs/field/validatable";
import {ValidatableInputField} from "../../form/input";
import Address from "../../../model/address";

function clientRegisterAddressData(
    initialZipCode: string,
    initialCity: string,
    initialCountry: string,
    initialStreet: string, initialStreetNumber: string, initialExtra: string) {
    return {
        zip: new ValidatableFieldData<string>(initialZipCode, (value) => {
            if (value.trim().length < 1) {
                return "Postal code may not be empty"
            } else {
                return null
            }
        }),
        city: new ValidatableFieldData<string>(initialCity, (value) => {
            if (value.trim().length < 1) {
                return "City may not be empty"
            } else {
                return null
            }
        }),
        country: new ValidatableFieldData<string>(initialCountry, (value) => {
            if (value.trim().length < 1) {
                return "Country may not be empty"
            } else {
                return null
            }
        }),

        street:
            new ValidatableFieldData<string>(initialStreet, (value) => {
                if (value.trim().length < 1) {
                    return "Street may not be empty"
                } else {
                    return null
                }
            }),
        streetNumber:
            new ValidatableFieldData<string>(initialStreetNumber, (value) => {
                if (value.trim().length < 1) {
                    return "Street number may not be empty"
                } else {
                    return null
                }
            }),
        extra: new ValidatableFieldData<string>(initialExtra, (value) => null),
    }
}

export function clientRegisterAddressDataFromAddress(address: Address): NewClientRegisterAddress {
    return clientRegisterAddressData(address.postalCode, address.city, address.country, address.street, address.streetNumber, address.extra ?? "")
}

export const emptyClientRegisterAddressFieldData = clientRegisterAddressData("", "", "", "", "", "")
export type NewClientRegisterAddress = typeof emptyClientRegisterAddressFieldData

const ClientRegisterAddressForm = observer((
    {addressData}:
        { addressData: NewClientRegisterAddress }
) => {
    return <>
        <div className="flex justify-between space-x-4">
            <ValidatableInputField data={addressData.street} label="Street" placeholder="Example Street" required/>
            <ValidatableInputField data={addressData.streetNumber} label="House number" placeholder="96a" required
                                   className="basis-96 shrink grow-0"/>

            <ValidatableInputField data={addressData.extra} label="Extra information" placeholder="Floor 2"/>
        </div>

        <div className="flex justify-between space-x-4">
            <ValidatableInputField data={addressData.zip} label="Postal code" placeholder="4321" required
                                   className="basis-96 shrink grow-0"/>
            <ValidatableInputField data={addressData.city} label="City" placeholder="Vienna" required/>

            <ValidatableInputField data={addressData.country} label="Country" placeholder="Austria" required/>
        </div>
    </>
})
export default ClientRegisterAddressForm
