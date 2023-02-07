import Head from "next/head";
import {NextLayoutPage} from "../../types/layout";
import {useStore} from "../../stores";
import {observer, useLocalObservable} from "mobx-react";
import React from "react";
import {runInAction} from "mobx";
import Purchase from "../../model/purchase";
import {PurchaseTableHeader, PurchaseTableRow} from "../../components/purchase/PurchaseTable";
import {MD5} from "object-hash";

export const categories: {
    [key: string]:
        { humanReadable: string, databaseField: string, validation?: (value: string) => string | null }
} = {
    "product_type": {
        humanReadable: "Product_type", databaseField: "product_type", validation: (val) => {
            if (val.trim().length == 0) return "Product-Type may not be empty"

            return null
        }
    },
    "description-type": {
        humanReadable: "Description-Type", databaseField: "description_type", validation: (val) => {
            if (val.trim().length == 0) return "Description-type may not be empty"

            return null
        }
    },
    "product": {
        humanReadable: "Product", databaseField: "product", validation: (val) => {
            if (val.trim().length == 0) return "Product may not be empty"

            return null
        }
    },
    "description": {
        humanReadable: "Description", databaseField: "description", validation: (val) => {
            if (val.trim().length == 0) return "Description may not be empty"

            return null
        }
    },
}
type PurchaseGroup = { facing: { [key: string]: string }, purchases: Purchase[] }

const Purchases: NextLayoutPage = observer(() => {
    const store = useStore()
    const purchases = store.purchaseStore.purchases

    const selectedCategories = useLocalObservable(() => {
        const map = new Map<string, boolean>()

        for (const category of Object.keys(categories)) {
            map.set(category, true)
        }

        return map
    })

    // Group all purchases
    const calcGroups = () => {
        const groupMap = new Map<string, PurchaseGroup>()
        const selectedCategoriesArray = Array.from(selectedCategories.entries())

        for (const purchase of purchases) {
            let extractedCategories: { [key: string]: string } = {}

            for (const [field, selected] of selectedCategoriesArray) {
                if (selected) {
                    extractedCategories[field] = (purchase as any)[field]
                }
            }

            const hash = MD5(extractedCategories)

            if (groupMap.has(hash)) {
                groupMap.get(hash)?.purchases.push(purchase)
            } else {
                groupMap.set(hash, {facing: extractedCategories, purchases: [purchase]})
            }
        }

        return Array.from(groupMap.values())
    }
    const groups = calcGroups()

    // Checkboxes
    function CategoryCheckbox({display, field}: { display: string, field: string }) {
        return <div className="form-control">
            <label className="label cursor-pointer">
                <span className="label-text pr-4">{display}</span>
                <input type="checkbox" checked={selectedCategories.get(field)} className="checkbox checkbox-primary"
                       onChange={e => runInAction(() => selectedCategories.set(field, !selectedCategories.get(field)))}/>
            </label>
        </div>
    }

    return <>
        <Head>
            <title>YAMS - Purchase</title>
        </Head>

        <main>
            <div className="card card-compact bg-neutral m-2">
                <div className="card-body">
                    <div className="tooltip tooltip-accent tooltip-right w-fit cursor-help"
                         data-tip="Group purchases by all the selected categories">
                        <h2 className="card-title">Account multiple categories:</h2>
                    </div>
                    <div className="grid gap-x-4 gap-y-2 grid-cols-2 md:grid-cols-4 lg:grid-cols-6 justify-items-end">
                        {Object.entries(categories).map(([field, {humanReadable}]) =>
                            <CategoryCheckbox key={field} display={humanReadable} field={field}/>)}
                    </div>
                </div>
            </div>


            <div className="overflow-auto w-full p-2">
                {!Array.from(selectedCategories.values()).reduce((prev, cur) => prev || cur, false) ?
                    <div className="alert alert-warning shadow-lg">
                        <div>
                            <i className="fa-solid fa-warning flex-shrink-0 text-xl"/> No category selected!
                        </div>
                    </div> :
                    !groups.length ?
                        <div className="alert alert-info shadow-lg">
                            <div>
                                <i className="fa-solid fa-info-circle flex-shrink-0 text-xl"/> No purchases known.
                            </div>
                        </div> :
                        <table className="table w-full">
                            <thead>
                            <PurchaseTableHeader selectedCategories={selectedCategories}/>
                            </thead>

                            <tbody>
                            {groups.map(({facing, purchases}) =>
                                <PurchaseTableRow key={JSON.stringify(facing)}
                                                 selectedCategories={selectedCategories} facingValues={facing}
                                                 backingPurchases={purchases}/>)}
                            </tbody>
                        </table>
                }
            </div>
        </main>
    </>
})

export default Purchases