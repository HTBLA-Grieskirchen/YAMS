import {observer} from "mobx-react";
import Head from "next/head";
import {useQuery} from "../../libs/dbConnection";
import {useState} from "react";
import Product from "../../model/Product";
import ProductType from "../../model/ProductType";
import {ProductCreation, ProductListItem} from "../../components/ProductItem";

const ProductOverview = observer(() => {
    const [productsRaw, refreshProducts] = useQuery("SELECT * FROM product ORDER BY discription", undefined, 1000)
    const products: Product[] = productsRaw.length > 0 && productsRaw[0].result ? productsRaw[0].result.map((productRaw: any) => {
        if (productRaw.id !== undefined && productRaw.bezeichnung !== undefined && productRaw.price !== undefined && productRaw.productType !== undefined) {
            return new Product(productRaw.id, productRaw.bezeichnung, productRaw.productType,productRaw.price )
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
                        }}/>
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