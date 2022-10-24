class Address {
    public constructor(id: string, value: string, label: string) {
        this._id = id
        this._value = value
        this._label = label
    }

    private _id: string

    get id(): string {
        return this._id;
    }

    set id(value: string) {
        this._id = value;
    }

    private _value: string

    get value(): string {
        return this._value;
    }

    set value(value: string) {
        this._value = value;
    }

    private _label: string

    get label(): string {
        return this._label;
    }

    set label(value: string) {
        this._label = value;
    }
}

export default Address