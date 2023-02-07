import {observer} from "mobx-react";
import ProductType from "../../model/productType";
import {deleteProductType} from "../../libs/database/productType";
import {useState} from "react";
import {useStore} from "../../stores";
import notification from "../../libs/notification";
import {Tooltip} from "@material-tailwind/react";

const ProductTypeDetailItem = observer((
    {productType, onEdit, noBottomPadding}:
        { productType: ProductType, onEdit?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [deleting, setDeleting] = useState(false)
    const hasDependants = store.purchaseStore.products.filter((product) => product.productType == productType).length > 0

    async function remove() {
        setDeleting(true)

        const result = await deleteProductType(productType)
        if (result.error) {
            notification.error({
                title: "Delete ProductType",
                message: `${productType.name} can not be deleted. ${result.error.message}`
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
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200`}>
            <p className="max-w-prose truncate text-ellipsis">{productType.name}</p>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200`}>
            TestProductType
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

export default ProductTypeDetailItem
