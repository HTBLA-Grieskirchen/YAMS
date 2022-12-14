import {observer} from "mobx-react";
import Product from "../model/Product";
import ProductType from "../model/ProductType";
import {Result} from "surrealdb.js";
import {query} from "../libs/dbConnection";
import {useState} from "react";
import Land from "../model/land";

const ProductListItem = observer(({product, products, refresh}: { product: Product, products: Product[], refresh: () => void }) => {
    const [name, setName] = useState(product.name)
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
        const valError = await validate(product.id, product.name, product.price, product.productType, products)
        setValidationError(valError)
        console.log("ProduktTyp und so, obs eh gspeichert wird: "+product.productType+" bzw. "+product.productType.name)
        console.log("ProduktTyp der hoffentlich geht: "+productType+" bzw. "+productType.name)
        if (valError == null) {
            setChangeSubmitted(true)
            if ((await updateProduct(new Product(product.id, product.name, product.productType, product.price)))?.result) {
                setChangeSubmitted(false)
                setEditing(false)
                refresh()
                setName(name)
                setPrice(price)
                setProductType(productType)
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
                        <input onChange={e => setName(e.target.value)} value={name} type="text"
                               placeholder={product.name}
                               className="w-48 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">{product.name}</p>}
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
                        <input onChange={e => {
                            console.log(e.target.value)
                            setProductType(new ProductType("adsfd",e.target.value))
                        }} value={1} type="text"
                            //placeholder={land.short}
                               className="w-24 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg">{product.productType.name}</p>}
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
                        setName(product.name)
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
                                   products, types,
                                   onFinish
                               }: { products: Product[], types: ProductType[], onFinish: (result: Result<any> | null) => void }) => {
    const [name, setName] = useState("")
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
        //    setSubmitted(true)
        //    onFinish(await sendProduct(name, price, productType))
        //}
        let productTypeNeu;
        for(let i = 0; i < types.length; i++){
            if(types[i].id == productType.id){
                productTypeNeu = types[i]
            }
        }
        if(productTypeNeu == undefined){
            productTypeNeu = productType
        }
        console.log("ProductTypeNeu: "+productTypeNeu)
        const valError = await validate("", name, price, productTypeNeu, products)
        setValidationError(valError)
        if (valError == null) {
            setSubmitted(true)
            onFinish(await sendProduct(name, price, productTypeNeu))
        }

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
                    <input onChange={e => setName(e.target.value)} value={name} type="text"
                           placeholder="Produktname"
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

async function sendProduct(name: string, price: number, productType: ProductType): Promise<Result<any>> {
    const responseType = await query("CREATE product_type SET description = $description", {
        description: productType.name
    })

    console.log(responseType[0].result[0])

    const response = await query("CREATE product SET description = $description, price = $price, type=type::thing('product_type', $type)", {
        description: name,
        price: price,
        type: responseType[0].result[0].id
    })

    console.log(response[0].result[0])

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function updateProduct(product: Product): Promise<Result<any>> {
    const response = await query("UPDATE type::thing($productTable, $productID) SET description = $description, price = $price, type:productType", {
        productTable: Product.TABLE_NAME,
        productID: product.id,
        description: product.name,
        price: product.price
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function deleteProduct(product: Product): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM purchased WHERE product = type::thing($productTable, $productID)", {
        productTable: Product.TABLE_NAME,
        productID: product.id
    })

    const response = await query("DELETE type::thing($productTable, $productID)", {
        productTable: Product.TABLE_NAME,
        productID: product.id
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function validate(selfID: string, name: string, price: number, productType: ProductType, products: Product[]): Promise<string | null> {
    let error = ""
    if (name.trim().length == 0) {
        error += "Full Name may not be empty\n"
    }
    if (price == 0) {
        error += "Price may not be 0\n"
    }

    if (error.length != 0) {
        return error
    }

    //if (lands.find((land) => land.short == short && land.id != selfID)) {
    //    return "That Shorthand already exists"
    //} else
    if ((await query("SELECT * FROM product WHERE name = description", {
        description: name
    }))[0]?.result.length > 0) {
        return "That description already exists"
    }
    return null
}

export {ProductListItem, ProductCreation}