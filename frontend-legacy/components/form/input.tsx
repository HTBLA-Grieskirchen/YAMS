import { observer } from "mobx-react";
import React from "react";
import { ValidatableFieldData } from "../../libs/field/validatable";
import { Input } from "@heroui/react";

export const ValidatableInputField = observer(
  <T extends unknown>({
    data,
    label,
    placeholder,
    type,
    required,
    mapSetValue,
    mapDisplayValue,
    className,
  }: {
    data: ValidatableFieldData<T>;
    label?: string;
    placeholder?: string;
    type?: string;
    required?: boolean;
    mapSetValue?: (targetValue: string) => T;
    mapDisplayValue?: (dataValue: T) => string | undefined;
    className?: string;
  }) => {
    const actualType = type ?? "text";
    const actualSetValue = mapSetValue ?? ((targetValue) => targetValue as any);
    const actualDisplayValue =
      mapDisplayValue ?? ((dataValue) => String(dataValue));

    return (
      <Input
        type={actualType}
        label={label}
        placeholder={placeholder}
        isRequired={required}
        className={className}
        value={actualDisplayValue(data.value)}
        onValueChange={(value) => data.setValue(actualSetValue(value))}
        isInvalid={
          data.displayError != null && !!data.displayError.trim().length
        }
        errorMessage={data.displayError}
        variant="bordered"
        labelPlacement="outside"
      />
    );
  },
);
