import {observer} from "mobx-react";
import Country from "../../model/country";
import {deleteCountry} from "../../libs/database/country";
import {useState} from "react";
import {useStore} from "../../stores";
import notification from "../../libs/notification";
import {Tooltip} from "@material-tailwind/react";

const CountryDetailItem = observer((
    {country, onEdit, nopb}:
        { country: Country, onEdit?: () => void, nopb?: boolean }
) => {
    const store = useStore()
    const [deleting, setDeleting] = useState(false)
    const hasDependants = store.addressStore.cities.filter((city) => city.country == country).length > 0

    async function remove() {
        setDeleting(true)

        const result = await deleteCountry(country)
        if (result.error) {
            notification.error({
                title: "Delete Country",
                message: `${country.name} can not be deleted. ${result.error.message}`
            }, 10, {
                "Retry": {
                    action: async () => {
                        await remove()
                        return true
                    },
                    disabled: () => deleting
                }
            })
        } else {
            await store.addressStore.refresh()
        }

        setDeleting(false)
    }

    return <div className="table-row">
        <div
            className={`table-cell ${nopb ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-900 whitespace-nowrap border-t-2 border-t-gray-200`}>
            <p className="w-16 shrink truncate text-ellipsis">{country.short}</p>
        </div>
        <div
            className={`table-cell ${nopb ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200`}>
            <p className="max-w-prose truncate text-ellipsis">{country.name}</p>
        </div>
        <div
            className={`table-cell ${nopb ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200`}>

        </div>
        <div
            className={`table-cell ${nopb ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200`}>
            <div className="w-16">
                {hasDependants ?
                    <Tooltip content="Is used in other objects" animate={{
                        mount: {scale: 1, y: 0},
                        unmount: {scale: 0, y: 10}
                    }} className="bg-black text-white px-2 py-1">
                        <button className="text-red-600/50 hover:cursor-not-allowed">
                            Delete
                        </button>
                    </Tooltip> :
                    <button
                        className="text-red-600 hover:underline disabled:text-red-600/50 disabled:hover:no-underline"
                        onClick={e => remove()} disabled={deleting}>
                        Delete
                    </button>
                }
            </div>
        </div>
        <div
            className={`table-cell ${nopb ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200`}>
            <div className="w-16">
                <button className="text-blue-600 hover:underline" onClick={e => onEdit && onEdit()}>
                    Edit
                </button>
            </div>
        </div>
    </div>
})

export default CountryDetailItem
