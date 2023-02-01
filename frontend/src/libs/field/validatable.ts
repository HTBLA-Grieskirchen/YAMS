import {makeAutoObservable} from "mobx";

export class ValidatableFieldData<T> {
    value: T
    displayError: string | null
    private readonly initial: T
    private readonly validate: (value: T) => string | null
    private readonly checkChanged: (initial: T, value: T) => boolean

    public constructor(
        initial: T,
        validate: (value: T) => string | null,
        changed: (initial: T, value: T) => boolean =
            (initial, value) => initial != value
    ) {
        this.initial = initial
        this.value = initial
        this.displayError = null
        this.validate = validate
        this.checkChanged = changed

        makeAutoObservable(this)
    }

    get valid() {
        return this.validate(this.value) == null
    }

    get modified() {
        return this.checkChanged(this.initial, this.value)
    }

    setValue(value: T) {
        this.value = value
        this.displayError = this.validate(value)
    }
}