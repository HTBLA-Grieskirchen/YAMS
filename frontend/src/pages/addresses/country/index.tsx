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

        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <CountryTableHeader/>
                <div className="table-row-group w-full bg-white divide-y-2 ">
                    {<CountryAddItem/>}
                    {countries.map((country) =>
                        <CountryListEntry key={country.record.join()} country={country}/>)}
                </div>
            </div>
        </main>
    </>
})

CountryOverview.Layout = AddressLayout

export default CountryOverview