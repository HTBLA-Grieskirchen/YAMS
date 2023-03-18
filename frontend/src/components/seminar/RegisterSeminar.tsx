import {ValidatableFieldData} from "../../libs/field/validatable";
import {observer} from "mobx-react";
import {ValidatableInputField} from "../form/input";
import React from "react";
import parseDuration from "parse-duration";

export function createSeminarFormDate(
    title: string,
    price: number,
    duration: number | null
) {
    return {
        title: new ValidatableFieldData(title, (value) => {
            if (value.trim().length < 1) {
                return "Title may not be empty"
            } else {
                return null
            }
        }),
        price: new ValidatableFieldData(price, (value) => {
            const isNumber = !Number.isNaN(value)
            if (isNumber && value < 0) {
                return "Only positive values are allowed"
            } else if (!isNumber) {
                return "Has to be a valid number"
            } else {
                return null
            }
        }),
        duration: new ValidatableFieldData(duration, (value) => {
            const isNumber = !Number.isNaN(value) && value != null
            if (!isNumber && value != null) {
                return "This has to be a valid duration"
            } else if (isNumber && value <= 0) {
                return "This has to be a positive duration"
            }
            {
                return null
            }
        }),
    }
}

export function defaultSeminarFormData() {
    return createSeminarFormDate("", Number.NaN, null)
}

export type NewSeminarFormData = ReturnType<typeof createSeminarFormDate>

const RegisterSeminarForm = observer((
    {addressData}:
        { addressData: NewSeminarFormData }
) => {
    return <div className="flex space-x-4">
        <ValidatableInputField data={addressData.title} label="Title" placeholder="Spine Injury Treatment"
                               className="max-w-lg shrink grow-0"/>
        <ValidatableInputField data={addressData.duration} label="Duration" placeholder="2h 15m"
                               className="basis-96 shrink grow-0" mapDisplayValue={(_) => undefined}
                               mapSetValue={(val) => val.trim().length < 1 ? null : (parseDuration(val) ?? Number.NaN)}/>
        <ValidatableInputField data={addressData.price} label="Price" placeholder="20.50"
                               className="basis-96 shrink grow-0" mapDisplayValue={(_) => undefined} required
                               mapSetValue={(val) => val.trim().length < 1 ? Number.NaN : Number(val)}/>
    </div>
})

export default RegisterSeminarForm
