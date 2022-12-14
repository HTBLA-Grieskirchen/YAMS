import {observer} from "mobx-react";
import {useState} from "react";
import notification from "../../libs/notification";
import {useStore} from "../../stores";
import City from "../../model/city";
import Link from "next/link";
import paths from "../../util/paths";
import {Record} from "../../model/surreal";
import Address from "../../model/address";
import {patchAddress} from "../../libs/database/address";

const AddressEditItem = observer((
    {address, onConfirm, onCancel, noBottomPadding}:
        { address: Address, onConfirm?: (success: boolean) => void, onCancel?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [street, setStreet] = useState(address.street)
    const [extra, setExtra] = useState(address.extra)
    const [city, setCity] = useState(address.city)
    const [submitted, setSubmitted] = useState(false)

    const [streetErrors, setStreetErrors] = useState<string[]>([])
    const [extraErrors, setExtraErrors] = useState<string[]>([])
    const [cityErrors, setCityErrors] = useState<string[]>([])

    const emptyCityID = new Record(City.TABLE, "").join()

    function check(street: string, extra: string, city: City): [string[], string[], string[]] {
        street = street.toUpperCase()

        const localStreetErrors = []
        const localNameErrors: string[] = []
        const localCityErrors = []

        if (street.trim().length == 0) {
            localStreetErrors.push("Street may not be empty")
        }
        if (city.record.join() == emptyCityID) {
            localCityErrors.push("City has to be selected")
        }

        return [localStreetErrors, localNameErrors, localCityErrors]
    }

    function checkset(street: string, extra: string, city: City): boolean {
        const [localStreetErrors, localExtraErrors, localCityErrors] = check(street, extra, city)

        setStreetErrors(localStreetErrors)
        setExtraErrors(localExtraErrors)
        setCityErrors(localCityErrors)

        return !localStreetErrors.length && !localExtraErrors.length && !localCityErrors.length
    }

    function validate(): boolean {
        return checkset(street, extra, city)
    }

    async function submit() {
        setSubmitted(true)

        if (validate()) {
            const result = await patchAddress(address, street, extra, city)
            if (result.error) {
                notification.error({
                    title: "Update Address",
                    message: `${address.street} can not be updated. ${result.error.message}.`
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
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-900 border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <input type="text" placeholder="Street and number"
                   className="w-64 px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setStreet(e.target.value)
                       checkset(e.target.value, extra, city)
                   }} value={street}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <input type="text" placeholder="Extra information"
                   className="w-40 px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setExtra(e.target.value)
                       checkset(street, e.target.value, city)
                   }} value={extra}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col">
                <p className={`max-w-prose truncate text-ellipsis ${city.record.id.trim() ? "text-gray-800" : "text-red-600/75"}`}>
                    {city.record.id.trim() ? city.plz : "Nothing selected"}
                </p>
                {store.addressStore.cities.length > 0 ?
                    <select className="form-select form-select-sm
                        block
                        w-fit pr-4 m-0
                        text-sm font-normal text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300 rounded
                        transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
                            placeholder="Select a city" onChange={e => {
                        let city = store.addressStore.cities.find((city) => city.record.join() == e.target.value)!!
                        setCity(city)
                        checkset(street, extra, city)
                    }} value={city.record.join()}>
                        {emptyCityID == city.record.join() &&
                            <option value={emptyCityID} hidden>Select a city</option>}
                        {store.addressStore.cities.map((city) => <option key={city.record.join()}
                                                                         value={city.record.join()}>
                                {city.name}
                            </option>
                        )}
                    </select> :
                    <Link href={paths.cities}><a className="
                        w-fit px-2
                        font-bold text-gray-800 hover:text-gray-900
                        rounded
                        shadow-sm hover:shadow-md
                        bg-gray-300 hover:bg-gray-200
                        transition">
                        Start to add cities
                    </a></Link>
                }
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            {city.record.join() == emptyCityID ?
                <p className={`max-w-prose truncate text-ellipsis text-red-600/75`}>
                    Select a city
                </p> :
                <div className="flex flex-col">
                    <p className="max-w-prose truncate text-ellipsis text-gray-800">{city.country.short}</p>
                    <p className="max-w-prose truncate text-ellipsis text-gray-500">{city.country.name}</p>
                </div>
            }
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col text-left">
                {streetErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {extraErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {cityErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="w-16">
                <button className="text-green-600 hover:underline disabled:opacity-50 disabled:hover:no-underline"
                        type="submit"
                        disabled={submitted || !!check(street, extra, city).reduce((prev, current) => prev + current.length, 0)}>
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

export default AddressEditItem
