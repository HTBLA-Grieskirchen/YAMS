import {observer} from "mobx-react";
import Head from "next/head";
import {NextLayoutPage} from "../../../types/layout";
import AddressLayout from "../_layout";
import {useStore} from "../../../stores";
import CountryTableHeader from "../../../components/country/CountryTableHeader";
import CountryAddItem from "../../../components/country/CountryAddItem";
import CountryListEntry from "../../../components/country/CountryListEntry";

const CountryOverview: NextLayoutPage = observer(() => {
    const store = useStore()
    const countries = store.addressStore.countries

    return <>
        <Head>
            <title>YAMS - Country</title>
        </Head>

        <main className="w-full p-2 bg-white">
            <div className="flex flex-col w-full shadow-md">
                <div className="table table-auto w-full">
                    <CountryTableHeader/>
                    <div className="table-row-group w-full bg-white divide-y-2 ">
                        {<CountryAddItem/>}
                        {countries.length > 0 ?
                            countries.map((country) =>
                                <CountryListEntry key={country.record.join()} country={country}/>) :
                            <tr>
                                <td colSpan={10} className="py-1">
                                    <div className="w-full flex">
                                        <p className="mx-auto text-gray-600 whitespace-nowrap font-normal">Start to add
                                            countries</p>
                                    </div>
                                </td>
                            </tr>}
                    </div>
                </div>
            </div>
        </main>
    </>
})

CountryOverview.Layout = AddressLayout

export default CountryOverview