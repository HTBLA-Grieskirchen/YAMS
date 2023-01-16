import {observer} from "mobx-react";
import ProductType from "../../model/productType";
import {useState} from "react";
import notification from "../../libs/notification";
import {useStore} from "../../stores";
import Product from "../../model/product";
import {patchProduct} from "../../libs/database/product";
import Link from "next/link";
import paths from "../../util/paths";
import {Record} from "../../model/surreal";
import productTypeEditItem from "../producttype/ProductTypeEditItem";

const ProductEditItem = observer((
    {product, onConfirm, onCancel, noBottomPadding}:
        { product: Product, onConfirm?: (success: boolean) => void, onCancel?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [name, setName] = useState(product.name)
    const [price, setPrice] = useState(product.price)
    const [productType, setProductType] = useState(product.productType)
    const [submitted, setSubmitted] = useState(false)

    const [nameErrors, setNameErrors] = useState<string[]>([])
    const [priceErrors, setPriceErrors] = useState<string[]>([])
    const [productTypeErrors, setProductTypeErrors] = useState<string[]>([])

    const emptyProductTypeID = new Record(ProductType.TABLE, "").join()

    function check(name: string, price: number, productType: ProductType): [string[], string[], string[]] {

        const localNameErrors = []
        const localPriceErrors = []
        const localProductTypeErrors = []

        try{
            if (name.trim().length == 0) {
                localNameErrors.push("Name may not be empty")
            }
        }
        catch (Exception){
            console.log("ai ai ai"+Exception);
        }
        console.log(name.toString()+"Des is da Name, der hoffentlich ned leer is")

        if (productType.record.join() == emptyProductTypeID) {
            localProductTypeErrors.push("ProductType has to be selected")
        }
        if (price == 0){
            localPriceErrors.push("Price my not be 0")
        }


        return [localNameErrors, localPriceErrors, localProductTypeErrors]
    }

    function checkset(name: string, price: number, productType: ProductType): boolean {
        const [localNameErrors, localProductTypeErrors] = check(name, price, productType)

        setNameErrors(localNameErrors)
        setProductTypeErrors(localProductTypeErrors)

        return !localNameErrors.length && !localProductTypeErrors.length
    }

    function validate(): boolean {
        return checkset(name, price, productType)
    }

    async function submit() {
        setSubmitted(true)

        if (validate()) {
            const result = await patchProduct(product, name, productType, price)
            if (result.error) {
                notification.error({
                    title: "Update Product",
                    message: `${product.name} can not be updated. ${result.error.message}.`
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
            <input type="text" placeholder="Product Name" className="px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setName(e.target.value)
                       checkset(e.target.value, price, productType)
                   }} value={name}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col">
                <p className={`max-w-prose truncate text-ellipsis ${productType.name ? "text-gray-800" : "text-red-600/75"}`}>
                    {productType.name?.trim() ? productType.name : "Nothing selected"}
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
                            placeholder="Select a productType" onChange={e => {
                        let productType = store.purchaseStore.productTypes.find((productType) => productType.record.join() == e.target.value)!!
                        setProductType(productType)
                        checkset(name, price, productType)
                    }} value={productType.record.join()}>
                        {emptyProductTypeID == productType.record.join() &&
                            <option value={emptyProductTypeID} hidden>Select a Product Type</option>}
                        {store.purchaseStore.productTypes.map((productType) => <option key={productType.record.join()}
                                                                               value={productType.record.join()}>
                                {productType.name}
                            </option>
                        )}
                    </select> :
                    <Link href={paths.productTypes}><a className="
                        w-fit px-2
                        font-bold text-gray-800 hover:text-gray-900
                        rounded
                        shadow-sm hover:shadow-md
                        bg-gray-300 hover:bg-gray-200
                        transition">
                        Start to add Product Types
                    </a></Link>
                }
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col text-left">
                {nameErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {productTypeErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="w-16">
                <button className="text-green-600 hover:underline disabled:opacity-50 disabled:hover:no-underline"
                        type="submit"
                        disabled={submitted || !!check(name, price, productType).reduce((prev, current) => prev + current.length, 0)}>
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

export default ProductEditItem
