import Head from "next/head";
import {NextLayoutPage} from "../../types/layout";
import PurchaseLayout from "./_layout";
import {useStore} from "../../stores";
import PurchaseTableHeader from "../../components/purchase/PurchaseTableHeader";
import PurchaseAddItem from "../../components/purchase/PurchaseAddItem";
import PurchaseListEntry from "../../components/purchase/PurchaseListEntry";
import {observer} from "mobx-react";

const Purchases: NextLayoutPage = observer(() => {
    const store = useStore()
    const purchases = store.purchaseStore.purchases

    return <>
        <Head>
            <title>YAMS - Address</title>
        </Head>

        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <PurchaseTableHeader/>
                <div className="table-row-group w-full bg-white divide-y-2 ">
                    {<PurchaseAddItem/>}
                    {purchases.length > 0 ?
                        purchases.map((purchase) =>
                            <PurchaseListEntry key={purchase.record.join()} purchase={purchase}/>) :
                        <tr>
                            <td colSpan={10} className="py-1">
                                <div className="w-full flex">
                                    <p className="mx-auto text-gray-600 whitespace-nowrap font-normal">Start to add
                                        purchases</p>
                                </div>
                            </td>
                        </tr>}
                </div>
            </div>
        </main>
    </>
})

Purchases.Layout = PurchaseLayout

export default Purchases