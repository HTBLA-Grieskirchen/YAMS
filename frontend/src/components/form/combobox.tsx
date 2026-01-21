import { observer } from "mobx-react";
import React, { useEffect, useMemo } from "react";
import { ValidatableFieldData } from "../../libs/field/validatable";
import { Autocomplete, AutocompleteItem } from "@heroui/react";

export interface ComboBoxItem<T> {
    id: string;
    label: string;
    value: T;
}

export const ValidatableComboBox = observer(<T extends any, N extends any>(
    {data, label, placeholder, required, disabled, newValue, items, className, setQuery}:
        {
            className?: string, 
            items: ComboBoxItem<T>[]
            data: ValidatableFieldData<any>, label?: string, placeholder?: string, required?: boolean, disabled?: boolean,
            newValue?: { data: N, prompt: string }, setQuery: (query: string) => void
        }
) => {
    const isNotNewState = newValue === undefined || data.value != newValue.data

    useEffect(() => {
        if (!isNotNewState) {
            setQuery("")
        }
    }, [setQuery, isNotNewState])

    const allItems = useMemo(() => {
        const result: any[] = [];
        if (newValue !== undefined && isNotNewState) {
            result.push({
                id: "new-value-option",
                label: newValue.prompt,
                value: newValue.data
            });
        }
        
        return [...result, ...items];
    }, [items, newValue, isNotNewState]);

    return (
        <Autocomplete
            label={label}
            placeholder={isNotNewState ? placeholder : "Creating new..."}
            className={className}
            isRequired={required}
            isDisabled={disabled}
            isInvalid={data.displayError != null && !!data.displayError.trim().length}
            errorMessage={data.displayError}
            variant="bordered"
            labelPlacement="outside"
            onInputChange={setQuery}
            selectedKey={allItems.find(i => i.value === data.value)?.id}
            onSelectionChange={(key) => {
                const item = allItems.find(i => i.id === key);
                if (item) {
                    data.setValue(item.value);
                }
            }}
        >
            {allItems.map((item) => (
                <AutocompleteItem key={item.id} textValue={item.label}>
                    {item.label}
                </AutocompleteItem>
            ))}
        </Autocomplete>
    )
})
