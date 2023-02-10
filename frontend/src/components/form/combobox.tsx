import { observer } from "mobx-react";
import { Combobox } from "@headlessui/react";
import React, { useEffect } from "react";
import { ValidatableFieldData } from "../../libs/field/validatable";

export const ValidatableComboBox = observer(<T extends unknown, N extends unknown>(
    {data, label, placeholder, required, disabled, newValue, mapDisplayValue, className, setQuery, children}:
        {
            className?: string, children?: React.ReactElement<typeof Combobox.Option>[]
            data: ValidatableFieldData<T | N>, label?: string, placeholder?: string, required?: boolean, disabled?: boolean,
            mapDisplayValue: (dataValue: T | N) => string, newValue?: { data: N, prompt: string }, setQuery: (query: string) => void
        }
) => {
    const isNotNewState = newValue === undefined || data.value != newValue.data
    const hasOptions = newValue !== undefined && isNotNewState || children !== undefined && !!children.length

    useEffect(() => {
        if (!isNotNewState) {
            setQuery("")
        }
    }, [setQuery, isNotNewState])

    return <div className={`dropdown w-full ${!!className ? className : ""}`}>
        <Combobox value={data.value} onChange={data.setValue} disabled={disabled}>
            <div className="form-control w-full">
                <label className="label">
                    <span className="label-text">{label}</span>
                </label>

                <div className="indicator w-full">
                    {required &&
                        <span className="indicator-item badge badge-ghost text-error px-1.5">
                            <i className="fa-solid fa-asterisk text-xs"/>
                        </span>}
                    <label className={`w-full ${hasOptions ? "input-group" : ""}`}>
                        <Combobox.Input className="input input-bordered w-full"
                                        onChange={e => setQuery(e.target.value)}
                                        displayValue={mapDisplayValue}
                                        placeholder={isNotNewState ? placeholder : "Creating new..."}/>
                        {hasOptions &&
                            <Combobox.Button className="btn btn-square">
                                <i className="fa-solid fa-angle-down"/>
                            </Combobox.Button>}
                    </label>
                </div>
            </div>


            {hasOptions &&
                <Combobox.Options
                    className="dropdown-content overflow-y-scroll menu shadow bg-base-200 p-2 rounded-btn max-w-md max-h-80 flex-nowrap">
                    {newValue !== undefined && isNotNewState && <>
                        <li className="menu-title mt-2">
                            <span>Custom</span>
                        </li>
                        <Combobox.Option value={newValue.data}>
                            <a>{newValue.prompt}</a>
                        </Combobox.Option>
                    </>}
                    {children !== undefined && !!children.length && <>
                        <li className="menu-title mt-2">
                            <span>Existing</span>
                        </li>
                        {children}
                    </>}
                </Combobox.Options>}
        </Combobox>
    </div>
})