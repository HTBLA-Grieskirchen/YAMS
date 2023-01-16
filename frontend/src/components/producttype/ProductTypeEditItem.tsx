import {observer} from "mobx-react";
import ProductType from "../../model/productType";
import {useState} from "react";
import {patchProductType} from "../../libs/database/productType";
import notification from "../../libs/notification";
import {useStore} from "../../stores";

const ProductTypeEditItem = observer((
    {productType, onConfirm, onCancel, noBottomPadding}:
        { productType: ProductType, onConfirm?: (success: boolean) => void, onCancel?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [name, setName] = useState(productType.name)
    const [submitted, setSubmitted] = useState(false)

    const [nameErrors, setNameErrors] = useState<string[]>([])

    function check(name: string): [string[], string[]] {

        const localCodeErrors = []
        const localNameErrors = []

        if (name.length == 0) {
            localNameErrors.push("Name may not be empty")
        }

        if (store.purchaseStore.productTypes.find((item) => item.name == name && item.record != productType.record)) {
            localCodeErrors.push("That code already exists")
        }

        return [localCodeErrors, localNameErrors]
    }

    function validate(): boolean {
        const [tempCodeErrors, tempNameErrors] = check(name)
        setNameErrors(tempNameErrors)

        return !tempCodeErrors.length && !tempNameErrors.length
    }

    async function submit() {
        setSubmitted(true)


        if (validate()) {

            const result = await patchProductType(productType, name)
            if (result.error) {
                notification.error({
                    title: "Update ProductType",
                    message: `${productType.name} can not be updated. ${result.error.message}.`
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
                await store.purchaseStore.refresh()
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
            <input type="text" placeholder="ProductType Code"
                   className="w-16 px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setName(e.target.value)
                       const [localCodeErrors, localNameErrors] = check(e.target.value)
                       setNameErrors(localNameErrors)
                   }}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col text-left">
                {nameErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
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

export default ProductTypeEditItem
