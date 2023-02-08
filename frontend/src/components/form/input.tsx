import {observer} from "mobx-react";
import React from "react";
import {ValidatableFieldData} from "../../libs/field/validatable";

export const ValidatableInputField = observer(<T extends unknown>(
    {data, label, placeholder, type, required, mapSetValue, mapDisplayValue, className}:
        {
            data: ValidatableFieldData<T>, label?: string, placeholder?: string, type?: string, required?: boolean,
            mapSetValue?: (targetValue: string) => T, mapDisplayValue?: (dataValue: T) => string, className?: string
        }
) => {
    const actualType = type ?? "text"
    const actualSetValue = mapSetValue ?? ((targetValue) => targetValue as any)
    const actualDisplayValue = mapDisplayValue ?? ((dataValue) => String(dataValue))

    return <div className={`form-control w-full ${!!className ? className : ""}`}>
        <label className="label">
            {label && <span className="label-text">{label}</span>}
        </label>
        <div className="indicator w-full">
            {required &&
                <span className="indicator-item badge badge-ghost text-error px-1.5">
                    <i className="fa-solid fa-asterisk text-xs"/>
                </span>}
            <input type={actualType} placeholder={placeholder} required={required}
                   className={`input input-bordered w-full ${data.displayError != null ? "input-error" : ""}`}
                   value={actualDisplayValue(data.value)}
                   onChange={e => data.setValue(actualSetValue(e.target.value))}/>
        </div>
        <label className="label">
            {data.displayError != null &&
                <span className="label-text text-error">
                    <i className="fa-solid fa-circle-exclamation"/> {data.displayError}
                </span>}
        </label>
    </div>
})