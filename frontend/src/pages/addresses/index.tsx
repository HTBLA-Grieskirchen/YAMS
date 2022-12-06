import Head from "next/head";
import {NextLayoutPage} from "../../types/layout";
import AddressLayout from "./_layout";
import {useStore} from "../../stores";
import AddressTableHeader from "../../components/address/AddressTableHeader";
import AddressAddItem from "../../components/address/AddressAddItem";
import AddressListEntry from "../../components/address/AddressListEntry";
import {observer} from "mobx-react";

const Addresses: NextLayoutPage = observer(() => {
    const store = useStore()
    const addresses = store.addressStore.addresses

    return <>
        <Head>
            <title>YAMS - Address</title>
        </Head>

        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <AddressTableHeader/>
                <div className="table-row-group w-full bg-white divide-y-2 ">
                    {<AddressAddItem/>}
                    {addresses.length > 0 ?
                        addresses.map((address) =>
                            <AddressListEntry key={address.record.join()} address={address}/>) :
                        <tr>
                            <td colSpan={10} className="py-1">
                                <div className="w-full flex">
                                    <p className="mx-auto text-gray-600 whitespace-nowrap font-normal">Start to add
                                        addresses</p>
                                </div>
                            </td>
                        </tr>}
                </div>
            </div>
        </main>
    </>
})

Addresses.Layout = AddressLayout

export default Addresses