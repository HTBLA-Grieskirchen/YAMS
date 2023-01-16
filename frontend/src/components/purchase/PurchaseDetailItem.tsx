import {observer} from "mobx-react";
import {useState} from "react";
import {useStore} from "../../stores";
import notification from "../../libs/notification";
import {Tooltip} from "@material-tailwind/react";
import Purchase from "../../model/purchase";
import {deletePurchase} from "../../libs/database/purchase";

const PurchaseDetailItem = observer((
    {purchase, onEdit, noBottomPadding}:
        { purchase: Purchase, onEdit?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [deleting, setDeleting] = useState(false)
    // TODO: Reimplement once client and events are available
    const hasDependants = false

    async function remove() {
        setDeleting(true)

        const result = await deletePurchase(purchase)
        if (result.error) {
            notification.error({
                title: "Delete Purchase",
                message: `${purchase.amount} can not be deleted. ${result.error.message}`
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
            await store.purchaseStore.refresh()
        }

        setDeleting(false)
    }

    return <div className="table-row">
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-900 border-t-2 border-t-gray-200`}>
            <p className="w-64 break-words text-ellipsis line-clamp-2 ">{purchase.amount}</p>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium whitespace-nowrap border-t-2 border-t-gray-200`}>
            <div className="flex flex-col">
                <p className="max-w-prose truncate text-ellipsis text-gray-500">{purchase.product.name}</p>
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium whitespace-nowrap border-t-2 border-t-gray-200`}>
            <div className="flex flex-col">
                <p className="max-w-prose truncate text-ellipsis text-gray-500">{purchase.product.productType.name}</p>
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

export default PurchaseDetailItem
