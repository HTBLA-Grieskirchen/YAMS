import {observer} from "mobx-react";
import Product from "../model/Product";
import ProductType from "../model/ProductType";
import {Result} from "surrealdb.js";
import {query} from "../libs/dbConnection";
import {useState} from "react";

const ProductListItem = observer(({product, products, refresh}: { product: Product, products: Product[], refresh: () => void }) => {
    const [bezeichnung, setBezeichnung] = useState(product.bezeichnung)
    const [price, setPrice] = useState(product.price)
    const [productType, setProductType] = useState(product.productType)
    const [editing, setEditing] = useState(false)
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [changeSubmitted, setChangeSubmitted] = useState(false)
    const [validationError, setValidationError] = useState<string | null>(null)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        if ((await deleteProduct(product))?.result) {
            refresh()
        }
    }

    const changeSubmit = async () => {
        //const shorthandUpper = shorthand.trim().toUpperCase()
        //const fullNameTrimmed = fullName.trim()
        //const valError = await validate(land.id, fullNameTrimmed, shorthandUpper, lands)
        //setValidationError(valError)
        /*if (valError == null)*/ {
            setChangeSubmitted(true)
            if ((await updateProduct(new Product(product.id, bezeichnung, productType, price)))?.result) {
                setChangeSubmitted(false)
                setEditing(false)
                refresh()
                //setFullName(fullNameTrimmed)
                //setShorthand(shorthandUpper)
            }
        }
    }

    return <div className="flex flex-col">
        <div className="flex flex-row space-x-6 items-center">
            <div className="flex flex-row space-x-4">
                <div className="flex flex-col">
                    <label className="text-gray-700 text-sm sm:w-48 w-fit">Bezeichnung</label>
                    {editing ?
                        <input onChange={e => setBezeichnung(e.target.value)} value={bezeichnung} type="text"
                               placeholder={product.bezeichnung}
                               className="w-48 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">{product.bezeichnung}</p>}
                </div>
                <div className="flex flex-col">
                    <label className="text-gray-700 text-sm">Preis</label>
                    {editing ?
                        <input onChange={e => setPrice(Number(e.target.value))} value={price} type="number"
                               //placeholder={land.short}
                               className="w-24 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg">{product.price}</p>}
                </div>
                <div className="flex flex-col">
                    <label className="text-gray-700 text-sm">Produkt-Typ</label>
                    {editing ?
                        <input onChange={e => setProductType(new ProductType("adsfd",e.target.value))} value={1} type="text"
                            //placeholder={land.short}
                               className="w-24 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg">typ</p>}
                </div>
            </div>


            {editing ?
                <form onSubmit={e => {
                    e.preventDefault()
                    changeSubmit()
                }} className="flex flex-row h-full self-end pb-1 space-x-1">
                    <button type="submit" disabled={changeSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-green-700 text-green-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-check"/>
                    </button>
                    <button type="button" onClick={e => {
                        setBezeichnung(product.bezeichnung)
                        setPrice(product.price)
                        setProductType(product.productType)
                        setValidationError(null)
                        setEditing(false)
                        setChangeSubmitted(false)
                    }}
                            className="align-text-bottom text-2xl hover:text-3xl hover:text-orange-700 text-orange-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-cancel"/>
                    </button>
                </form> :
                <div className="flex flex-row h-full items-center space-x-1">
                    <button type="button" onClick={e => {
                        setEditing(true)
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-2xl hover:text-3xl hover:text-gray-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-edit"/>
                    </button>
                    <button type="button" onClick={e => {
                        deleteSubmit()
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-red-700 text-red-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-remove"/>
                    </button>
                </div>}
        </div>
        {validationError &&
            <div className="flex flex-col space-y-0">
                {validationError.split("\n").map((error) => error.trim().length > 0 &&
                    <p className="text-red-600">* {error}</p>)}
            </div>}
    </div>
})

const ProductCreation = observer(({
                                   products,
                                   onFinish
                               }: { products: Product[], onFinish: (result: Result<any> | null) => void }) => {
    const [bezeichnung, setBezeichnung] = useState("")
    const [price, setPrice] = useState(0)
    const [productType, setProductType] = useState(new ProductType("",""))
    const [submitted, setSubmitted] = useState(false)
    const [validationError, setValidationError] = useState<string | null>(null)

    const onSubmit = async () => {
        //const shorthandUpper = shorthand.trim().toUpperCase()
        //const fullNameTrimmed = fullName.trim()
        //const valError = await validate("", fullNameTrimmed, shorthandUpper, lands)
        //setValidationError(valError)
        //if (valError == null) {
            setSubmitted(true)
            onFinish(await sendProduct(bezeichnung, price, productType))
        //}
    }

    return <div className="flex flex-col space-y-2">
        <form onSubmit={e => {
            e.preventDefault()
            onSubmit()
        }} onKeyDown={e => {
            if (e.key == "Escape") {
                onFinish(null)
            }
        }}>
            <div className="flex flex-row space-x-4 items-end">
                <div className="flex flex-col space-y-1">
                    <label className="text-gray-700">Bezeichnung</label>
                    <input onChange={e => setBezeichnung(e.target.value)} value={bezeichnung} type="text"
                           placeholder="aökdfajndva"
                           className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"/>
                </div>
                <div className="flex flex-col space-y-1">
                    <label className="text-gray-700">Preis</label>
                    <input onChange={e => setPrice(Number(e.target.value))} value={price} type="number" //placeholder="19"
                           className="w-32 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"/>
                </div>
                <div className="flex flex-col space-y-1">
                    <label className="text-gray-700">Typ</label>
                    <input onChange={e => setProductType(new ProductType("aödf",e.target.value))}  type="text" //placeholder="19" value={}
                           className="w-32 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"/>
                </div>
                <button type="submit" disabled={submitted}
                        className="align-text-bottom text-3xl hover:text-4xl hover:text-green-700 text-green-600 w-8 h-8 transition-all">
                    <i className="fa-solid fa-check"/>
                </button>
                <button onClick={e => {
                    onFinish(null)
                }}
                        className="align-text-bottom text-2xl hover:text-3xl hover:text-orange-700 text-orange-600 w-8 h-8 transition-all">
                    <i className="fa-solid fa-cancel"/>
                </button>
            </div>
        </form>
        {validationError &&
            <div className="flex flex-col space-y-0">
                {validationError.split("\n").map((error) => error.trim().length > 0 &&
                    <p className="text-red-600">* {error}</p>)}
            </div>}
    </div>
})

async function sendProduct(bezeichnung: string, price: number, productType:ProductType): Promise<Result<any>> {
    const response = await query("CREATE product SET description = $bezeichnung, price = $price, type=&type", {
        description: bezeichnung,
        price: price,
        type: productType
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function updateProduct(product: Product): Promise<Result<any>> {
    const response = await query("UPDATE type::thing($productTable, $productID) SET description = $bezeichnung, price = $price, type:productType", {
        productTable: Product.TABLE_NAME,
        productID: product.id,
        description: product.bezeichnung,
        price: product.price
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function deleteProduct(product: Product): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM city WHERE product = type::thing($productTable, $productID)", {
        productTable: Product.TABLE_NAME,
        productID: product.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Land is still used in some cities")
        }
    }

    const response = await query("DELETE type::thing($productTable, $productID)", {
        productTable: Product.TABLE_NAME,
        productID: product.id
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

/*async function validate(selfID: string, fullName: string, short: string, lands: Product[]): Promise<string | null> {
    let error = ""
    if (fullName.trim().length == 0) {
        error += "Full Name may not be empty\n"
    }
    if (short.trim().length == 0) {
        error += "Shorthand may not be empty\n"
    }

    if (error.length != 0) {
        return error
    }

    if (lands.find((land) => land.short == short && land.id != selfID)) {
        return "That Shorthand already exists"
    } else if ((await query("SELECT * FROM land WHERE short = $short", {
        short: short
    }))[0]?.result.length > 0) {
        return "That Shorthand already exists"
    }
    return null
}
*/
export {ProductListItem, ProductCreation}