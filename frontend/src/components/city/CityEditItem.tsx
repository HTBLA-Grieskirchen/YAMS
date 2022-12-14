import {observer} from "mobx-react";
import Country from "../../model/country";
import {useState} from "react";
import notification from "../../libs/notification";
import {useStore} from "../../stores";
import City from "../../model/city";
import {patchCity} from "../../libs/database/city";
import Link from "next/link";
import paths from "../../util/paths";
import {Record} from "../../model/surreal";

const CityEditItem = observer((
    {city, onConfirm, onCancel, noBottomPadding}:
        { city: City, onConfirm?: (success: boolean) => void, onCancel?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [plz, setPLZ] = useState(city.plz)
    const [name, setName] = useState(city.name)
    const [country, setCountry] = useState(city.country)
    const [submitted, setSubmitted] = useState(false)

    const [plzErrors, setPLZErrors] = useState<string[]>([])
    const [nameErrors, setNameErrors] = useState<string[]>([])
    const [countryErrors, setCountryErrors] = useState<string[]>([])

    const emptyCountryID = new Record(Country.TABLE, "").join()

    function check(code: string, name: string, country: Country): [string[], string[], string[]] {
        code = code.toUpperCase()

        const localPLZErrors = []
        const localNameErrors = []
        const localCountryErrors = []

        if (name.trim().length == 0) {
            localNameErrors.push("Name may not be empty")
        }
        if (code.trim().length == 0) {
            localPLZErrors.push("Postal code may not be empty")
        }
        if (country.record.join() == emptyCountryID) {
            localCountryErrors.push("Country has to be selected")
        }

        return [localPLZErrors, localNameErrors, localCountryErrors]
    }

    function checkset(code: string, name: string, country: Country): boolean {
        const [localPLZErrors, localNameErrors, localCountryErrors] = check(code, name, country)

        setPLZErrors(localPLZErrors)
        setNameErrors(localNameErrors)
        setCountryErrors(localCountryErrors)

        return !localPLZErrors.length && !localNameErrors.length && !localCountryErrors.length
    }

    function validate(): boolean {
        return checkset(plz, name, country)
    }

    async function submit() {
        setSubmitted(true)

        if (validate()) {
            const result = await patchCity(city, name, plz, country)
            if (result.error) {
                notification.error({
                    title: "Update City",
                    message: `${city.name} can not be updated. ${result.error.message}.`
                }, 10, {
                    "Retry": {
                        action: async () => {
                            await submit()
                            return true
                        },
                        disabled: () => submitted
                    }
                })
            } else {
                await store.addressStore.refresh()
            }
            onConfirm && onConfirm(!result.error)

        }

        setSubmitted(false)
    }

    return <form className="table-row group" onSubmit={e => {
        e.preventDefault()
        submit()
    }} onKeyDown={e => {
        if (e.key == "Escape") {
            onCancel && onCancel()
        }
    }}>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-900 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <input type="text" placeholder="Postal Code"
                   className="w-16 px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setPLZ(e.target.value)
                       checkset(e.target.value, name, country)
                   }} value={plz}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <input type="text" placeholder="City Name" className="px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setName(e.target.value)
                       checkset(plz, e.target.value, country)
                   }} value={name}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col">
                <p className={`max-w-prose truncate text-ellipsis ${country.short.trim() ? "text-gray-800" : "text-red-600/75"}`}>
                    {country.short.trim() ? country.short : "Nothing selected"}
                </p>
                {store.addressStore.countries.length > 0 ?
                    <select className="form-select form-select-sm
                        block
                        w-fit pr-4 m-0
                        text-sm font-normal text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300 rounded
                        transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
                            placeholder="Select a country" onChange={e => {
                        let country = store.addressStore.countries.find((country) => country.record.join() == e.target.value)!!
                        setCountry(country)
                        checkset(plz, name, country)
                    }} value={country.record.join()}>
                        {emptyCountryID == country.record.join() &&
                            <option value={emptyCountryID} hidden>Select a country</option>}
                        {store.addressStore.countries.map((country) => <option key={country.record.join()}
                                                                               value={country.record.join()}>
                                {country.name}
                            </option>
                        )}
                    </select> :
                    <Link href={paths.countries}><a className="
                        w-fit px-2
                        font-bold text-gray-800 hover:text-gray-900
                        rounded
                        shadow-sm hover:shadow-md
                        bg-gray-300 hover:bg-gray-200
                        transition">
                        Start to add countries
                    </a></Link>
                }
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col text-left">
                {plzErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {nameErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {countryErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="w-16">
                <button className="text-green-600 hover:underline disabled:opacity-50 disabled:hover:no-underline"
                        type="submit"
                        disabled={submitted || !!check(plz, name, country).reduce((prev, current) => prev + current.length, 0)}>
                    Confirm
                </button>
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="w-16">
                <button className="text-red-600 hover:underline" onClick={e => onCancel && onCancel()}>
                    Cancel
                </button>
            </div>
        </div>
    </form>
})

export default CityEditItem
