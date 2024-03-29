import Head from "next/head";
import {NavigationPage} from "../../types/layout";
import {useStore} from "../../stores";
import {observer, useLocalObservable} from "mobx-react";
import React from "react";
import paths from "../../util/paths";
import {useRouter} from "next/router";
import Link from "next/link";
import {runInAction} from "mobx";
import Address from "../../model/address";
import {AddressTableHeader, AddressTableRow} from "../../components/address/AddressTable";
import {MD5} from "object-hash";

export const categories: {
    [key: string]:
        { humanReadable: string, databaseField: string, validation?: (value: string) => string | null }
} = {
    "country": {
        humanReadable: "Country", databaseField: "country", validation: (val) => {
            if (val.trim().length == 0) return "Country may not be empty"

            return null
        }
    },
    "postalCode": {
        humanReadable: "Postal Code", databaseField: "postal_code", validation: (val) => {
            if (val.trim().length == 0) return "Postal code may not be empty"

            return null
        }
    },
    "city": {
        humanReadable: "City", databaseField: "city", validation: (val) => {
            if (val.trim().length == 0) return "City may not be empty"

            return null
        }
    },
    "street": {
        humanReadable: "Street", databaseField: "street", validation: (val) => {
            if (val.trim().length == 0) return "Street may not be empty"

            return null
        }
    },
    "streetNumber": {
        humanReadable: "Street Number", databaseField: "street_number", validation: (val) => {
            if (val.trim().length == 0) return "Street number may not be empty"

            return null
        }
    },
    "extra": {humanReadable: "Extra", databaseField: "extra"}
}
type AddressGroup = { facing: { [key: string]: string }, addresses: Address[] }


const Addresses: NavigationPage = observer(() => {
    const store = useStore()
    const addresses = store.addressStore.addresses

    const selectedCategories = useLocalObservable(() => {
        const map = new Map<string, boolean>()

        for (const category of Object.keys(categories)) {
            map.set(category, true)
        }

        return map
    })

    // Group all addresses
    const calcGroups = () => {
        const groupMap = new Map<string, AddressGroup>()
        const selectedCategoriesArray = Array.from(selectedCategories.entries())

        for (const address of addresses) {
            let extractedCategories: { [key: string]: string } = {}

            for (const [field, selected] of selectedCategoriesArray) {
                if (selected) {
                    extractedCategories[field] = (address as any)[field]
                }
            }

            const hash = MD5(extractedCategories)

            if (groupMap.has(hash)) {
                groupMap.get(hash)?.addresses.push(address)
            } else {
                groupMap.set(hash, {facing: extractedCategories, addresses: [address]})
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
            <title>All Addresses</title>
        </Head>

        <main className="p-6 overflow-y-auto">
            <div className="card card-compact bg-base-100 shadow p-4 max-h-full">
                <div className="tooltip tooltip-accent tooltip-right w-fit cursor-help"
                     data-tip="Group addresses by all the selected categories">
                    <h2 className="card-title">Account multiple categories:</h2>
                </div>
                <div className="grid gap-x-4 gap-y-2 grid-cols-2 md:grid-cols-4 lg:grid-cols-6 justify-items-end">
                    {Object.entries(categories).map(([field, {humanReadable}]) =>
                        <CategoryCheckbox key={field} display={humanReadable} field={field}/>)}
                </div>

                <div className="divider my-2"/>

                <div className="overflow-x-auto w-full mt-2">
                    {!Array.from(selectedCategories.values()).reduce((prev, cur) => prev || cur, false) ?
                        <div className="alert alert-warning">
                            <div>
                                <i className="fa-solid fa-warning flex-shrink-0 text-xl"/> No category selected!
                            </div>
                        </div> :
                        !groups.length ?
                            <div className="alert alert-info shadow-lg">
                                <div>
                                    <i className="fa-solid fa-info-circle flex-shrink-0 text-xl"/> No addresses known.
                                </div>
                            </div> :
                            <table className="table w-full">
                                <thead className="sticky top-0 z-30">
                                    <AddressTableHeader selectedCategories={selectedCategories}/>
                                </thead>

                                <tbody className="overflow-y-auto">
                                {groups.map(({facing, addresses}) =>
                                    <AddressTableRow key={JSON.stringify(facing)}
                                                     selectedCategories={selectedCategories} facingValues={facing}
                                                     backingAddresses={addresses}/>)}
                                </tbody>
                            </table>
                    }
                </div>
            </div>
        </main>
    </>
})

Addresses.NavPath = observer(() => {
    const path = paths.addresses

    const router = useRouter()
    const disabled = router.pathname == path

    return <>
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    <i className="fa-solid fa-map-location-dot mr-2"/>
                    Address
                </button>
            </Link>
        </li>
    </>
})

export default Addresses
