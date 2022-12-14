import {NextLayoutPage} from "../../../types/layout";
import {observer} from "mobx-react";
import AddressLayout from "../_layout";
import {useStore} from "../../../stores";
import Head from "next/head";
import CityTableHeader from "../../../components/city/CityTableHeader";
import CityAddItem from "../../../components/city/CityAddItem";
import CityListEntry from "../../../components/city/CityListEntry";

const CityOverview: NextLayoutPage = observer(() => {
    const store = useStore()
    const cities = store.addressStore.cities

    return <>
        <Head>
            <title>YAMS - City</title>
        </Head>

        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <CityTableHeader/>
                <div className="table-row-group w-full bg-white divide-y-2 ">
                    {<CityAddItem/>}
                    {cities.length > 0 ?
                        cities.map((city) =>
                            <CityListEntry key={city.record.join()} city={city}/>) :
                        <tr>
                            <td colSpan={10} className="py-1">
                                <div className="w-full flex">
                                    <p className="mx-auto text-gray-600 whitespace-nowrap font-normal">Start to add
                                        cities</p>
                                </div>
                            </td>
                        </tr>}
                </div>
            </div>
        </main>
    </>
})

CityOverview.Layout = AddressLayout

export default CityOverview