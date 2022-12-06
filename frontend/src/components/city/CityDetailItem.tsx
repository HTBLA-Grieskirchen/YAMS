import {observer} from "mobx-react";
import {useState} from "react";
import {useStore} from "../../stores";
import notification from "../../libs/notification";
import {Tooltip} from "@material-tailwind/react";
import City from "../../model/city";
import {deleteCity} from "../../libs/database/city";

const CityDetailItem = observer((
    {city, onEdit, noBottomPadding}:
        { city: City, onEdit?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [deleting, setDeleting] = useState(false)
    const hasDependants = store.addressStore.addresses.filter((address) => address.city == city).length > 0

    async function remove() {
        setDeleting(true)

        const result = await deleteCity(city)
        if (result.error) {
            notification.error({
                title: "Delete City",
                message: `${city.name} can not be deleted. ${result.error.message}`
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
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-900 whitespace-nowrap border-t-2 border-t-gray-200`}>
            <p className="w-16 shrink truncate text-ellipsis">{city.plz}</p>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-700 whitespace-nowrap border-t-2 border-t-gray-200`}>
            <p className="w-64 min-w-fit shrink truncate text-ellipsis">{city.name}</p>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium whitespace-nowrap border-t-2 border-t-gray-200`}>
            <div className="flex flex-col">
                <p className="max-w-prose truncate text-ellipsis text-gray-800">{city.country.short}</p>
                <p className="max-w-prose truncate text-ellipsis text-gray-500">{city.country.name}</p>
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200`}>

        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200`}>
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
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200`}>
            <div className="w-16">
                <button className="text-blue-600 hover:underline" onClick={e => onEdit && onEdit()}>
                    Edit
                </button>
            </div>
        </div>
    </div>
})

export default CityDetailItem
