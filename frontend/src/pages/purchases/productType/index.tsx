import {observer} from "mobx-react";
import Head from "next/head";
import {NextLayoutPage} from "../../../types/layout";
import PurchaseLayout from "../_layout";
import {useStore} from "../../../stores";
import ProductTypeTableHeader from "../../../components/producttype/ProductTypeTableHeader";
import ProductTypeAddItem from "../../../components/producttype/ProductTypeAddItem";
import ProductTypeListEntry from "../../../components/producttype/ProductTypeListEntry";

const ProductTypeOverview: NextLayoutPage = observer(() => {
    const store = useStore()
    const productTypes = store.purchaseStore.productTypes

    return <>
        <Head>
            <title>YAMS - ProductType</title>
        </Head>

        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <ProductTypeTableHeader/>
                <div className="table-row-group w-full bg-white divide-y-2 ">
                    {<ProductTypeAddItem/>}
                    {productTypes.length > 0 ?
                        productTypes.map((productType) =>
                            <ProductTypeListEntry key={productType.record.join()} productType={productType}/>) :
                        <tr>
                            <td colSpan={10} className="py-1">
                                <div className="w-full flex">
                                    <p className="mx-auto text-gray-600 whitespace-nowrap font-normal">Start to add
                                        productTypes</p>
                                </div>
                            </td>
                        </tr>}
                </div>
            </div>
        </main>
    </>
})

ProductTypeOverview.Layout = PurchaseLayout

export default ProductTypeOverview