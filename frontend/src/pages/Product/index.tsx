import {observer} from "mobx-react";
import Head from "next/head";
import {useQuery} from "../../libs/dbConnection";
import {useState} from "react";
import Product from "../../model/Product";
import ProductType from "../../model/ProductType";
import ProductTableHeader from "../../components/address/AddressTableHeader";
import ProductAddItem from "../../components/address/AddressAddItem";
import ProductListEntry from "../../components/address/AddressListEntry";
import {ProductCreation, ProductListItem} from "../../components/ProductItem";

const ProductOverview = observer(() => {
    const [productsRaw, refreshProducts] = useQuery("SELECT * FROM product ORDER BY description", undefined, 1000)
    const [typesRaw, refreshTypes] = useQuery("SELECT * FROM product_type") //unten dann alle productsRaw types suchen und mid den typesraw vergleichen, dann filtern auf das eine --> das returnen
    const types: ProductType[] = typesRaw.length > 0 && typesRaw[0].result ? typesRaw[0].result.map((typeRaw: any) => {
        return new ProductType(typeRaw.id, typeRaw.description)
    }).filter((it: any) => it !== undefined) : []

    const products: Product[] = productsRaw.length > 0 && productsRaw[0].result ? productsRaw[0].result.map((productRaw: any) => {
        if (productRaw.id !== undefined && productRaw.description !== undefined && productRaw.price !== undefined && productRaw.type !== undefined) {
            console.log("Produkt wies aus da DB kumt: "+productRaw.type)
            productRaw.type = new ProductType(productRaw.type, "Typ2")
            console.log("Produkt wies bearbeitet wird: "+productRaw.type)
            return new Product(productRaw.id, productRaw.description, productRaw.type ,productRaw.price )
        }
    }).filter((it: any) => it !== undefined) : []

    const [addingProduct, setAddingProduct] = useState(false)

    return <>
        <Head>
            <title>YAMS - Products</title>
        </Head>

        <main className="flex flex-col w-fill m-5 p-3 rounded-lg bg-gray-200 shadow">
            <div className="flex">
                {addingProduct ?
                    <div>
                        <ProductCreation products={products} onFinish={(successful) => {
                            if (successful?.result) {
                                refreshProducts()
                            }
                            setAddingProduct(false)
                        }} types={types}/>
                    </div> :
                    <button onClick={e => setAddingProduct(true)} className="m-1 w-24 h-10 hover:text-lg
                     border border-black
                     rounded
                     shadow
                     bg-blue-100 hover:bg-blue-300
                     transition-all">
                        New Product
                    </button>}
            </div>
            <div className="flex flex-col pt-3">
                {products.length > 0 ?
                    <div className="divide-gray-400 divide-y">
                        {products.map((product) => <div key={product.record()} className="p-2">
                            <ProductListItem product={product} products={products} refresh={refreshProducts}/>
                        </div>)}
                    </div> :
                    <p className="p-2 text-gray-600">No Product available!</p>}
            </div>
        </main>
    </>
})

export default ProductOverview