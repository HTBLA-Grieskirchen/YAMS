import {NextLayoutPage} from "../../../types/layout";
import {observer} from "mobx-react";
import PurchaseLayout from "../_layout";
import {useStore} from "../../../stores";
import Head from "next/head";
import ProductTableHeader from "../../../components/product/ProductTableHeader";
import ProductAddItem from "../../../components/product/ProductAddItem";
import ProductListEntry from "../../../components/product/ProductListEntry";

const ProductOverview: NextLayoutPage = observer(() => {
    const store = useStore()
    const products = store.purchaseStore.products

    return <>
        <Head>
            <title>YAMS - Product</title>
        </Head>

        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <ProductTableHeader/>
                <div className="table-row-group w-full bg-white divide-y-2 ">
                    {<ProductAddItem/>}
                    {products.length > 0 ?
                        products.map((product) =>
                            <ProductListEntry key={product.record.join()} product={product}/>) :
                        <tr>
                            <td colSpan={10} className="py-1">
                                <div className="w-full flex">
                                    <p className="mx-auto text-gray-600 whitespace-nowrap font-normal">Start to add
                                        products</p>
                                </div>
                            </td>
                        </tr>}
                </div>
            </div>
        </main>
    </>
})

ProductOverview.Layout = PurchaseLayout

export default ProductOverview