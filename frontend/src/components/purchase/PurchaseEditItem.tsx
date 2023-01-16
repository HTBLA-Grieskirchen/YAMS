import {observer} from "mobx-react";
import {useState} from "react";
import notification from "../../libs/notification";
import {useStore} from "../../stores";
import Product from "../../model/product";
import Link from "next/link";
import paths from "../../util/paths";
import {Record} from "../../model/surreal";
import Purchase from "../../model/purchase";
import {patchPurchase} from "../../libs/database/purchase";

const PurchaseEditItem = observer((
    {purchase, onConfirm, onCancel, noBottomPadding}:
        { purchase: Purchase, onConfirm?: (success: boolean) => void, onCancel?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [amount, setAmount] = useState(purchase.amount)
    const [product, setProduct] = useState(purchase.product)
    const [submitted, setSubmitted] = useState(false)

    const [amountErrors, setAmountErrors] = useState<string[]>([])
    const [productErrors, setProductErrors] = useState<string[]>([])

    const emptyProductID = new Record(Product.TABLE, "").join()

    function check(amount: string, product: Product): [string[], string[]] {

        const localNameErrors: string[] = []
        const localProductErrors = []

        if (product.record.join() == emptyProductID) {
            localProductErrors.push("Product has to be selected")
        }

        return [localNameErrors, localProductErrors]
    }

    function checkset(amount: string, product: Product): boolean {
        const [localAmountErrors, localProductErrors] = check(amount, product)

        setAmountErrors(localAmountErrors)
        setProductErrors(localProductErrors)

        return !localAmountErrors.length && !localProductErrors.length
    }

    function validate(): boolean {
        return checkset(amount+"", product)
    }

    async function submit() {
        setSubmitted(true)

        if (validate()) {
            const result = await patchPurchase(purchase, amount, product)
            if (result.error) {
                notification.error({
                    title: "Update Purchase",
                    message: `${purchase.amount} can not be updated. ${result.error.message}.`
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
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <input type="text" placeholder="Amount information"
                   className="w-40 px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setAmount(parseInt(e.target.value))
                       checkset(e.target.value, product)
                   }} value={amount}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col">
                <p className={`max-w-prose truncate text-ellipsis ${product.record.id.trim() ? "text-gray-800" : "text-red-600/75"}`}>
                    {product.record.id.trim() ? product.name: "Nothing selected"}
                </p>
                {store.purchaseStore.products.length > 0 ?
                    <select className="form-select form-select-sm
                        block
                        w-fit pr-4 m-0
                        text-sm font-normal text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300 rounded
                        transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
                            placeholder="Select a product" onChange={e => {
                        let product = store.purchaseStore.products.find((product) => product.record.join() == e.target.value)!!
                        setProduct(product)
                        checkset(amount+"", product)
                    }} value={product.record.join()}>
                        {emptyProductID == product.record.join() &&
                            <option value={emptyProductID} hidden>Select a product</option>}
                        {store.purchaseStore.products.map((product) => <option key={product.record.join()}
                                                                         value={product.record.join()}>
                                {product.name}
                            </option>
                        )}
                    </select> :
                    <Link href={paths.products}><a className="
                        w-fit px-2
                        font-bold text-gray-800 hover:text-gray-900
                        rounded
                        shadow-sm hover:shadow-md
                        bg-gray-300 hover:bg-gray-200
                        transition">
                        Start to add products
                    </a></Link>
                }
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            {product.record.join() == emptyProductID ?
                <p className={`max-w-prose truncate text-ellipsis text-red-600/75`}>
                    Select a product
                </p> :
                <div className="flex flex-col">
                    <p className="max-w-prose truncate text-ellipsis text-gray-500">{product.productType.name}</p>
                </div>
            }
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col text-left">
                {amountErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {productErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="w-16">
                <button className="text-green-600 hover:underline disabled:opacity-50 disabled:hover:no-underline"
                        type="submit"
                        disabled={submitted || !!check(amount+"", product).reduce((prev, current) => prev + current.length, 0)}>
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

export default PurchaseEditItem
