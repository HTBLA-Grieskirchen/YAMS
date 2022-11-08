import {observer} from "mobx-react";
import Head from "next/head";
import {useLive} from "../../../libs/database";
import Country from "../../../model/country";
import {CountryCreation, CountryListItem} from "../../../components/CountryItem";
import {useState} from "react";
import {NextPage} from "next";

const CountryOverview: NextPage = observer(() => {
    const [countriesRaw, refreshCountries] = useLive("SELECT * FROM country ORDER BY name")
    const countries: Country[] = countriesRaw.response && countriesRaw.response[0].result ? countriesRaw.response[0].result.map((landRaw: any) => {
        if (landRaw.id !== undefined && landRaw.name !== undefined && landRaw.short !== undefined) {
            return new Country(landRaw.id, landRaw.name, landRaw.short)
        }
    }).filter((it: any) => it !== undefined) : []

    const [addingCountry, setAddingCountry] = useState(false)

    return <>
        <Head>
            <title>YAMS - Country</title>
        </Head>

        <main className="flex flex-col w-fill m-5 p-3 rounded-lg bg-gray-200 shadow">
            <div className="flex">
                {addingCountry ?
                    <div>
                        <CountryCreation countries={countries} onFinish={(successful) => {
                            if (successful?.result) {
                                refreshCountries()
                            }
                            setAddingCountry(false)
                        }}/>
                    </div> :
                    <button onClick={e => setAddingCountry(true)} className="m-1 w-24 h-10 hover:text-lg
                     border border-black
                     rounded
                     shadow
                     bg-blue-100 hover:bg-blue-300
                     transition-all">
                        New Land
                    </button>}
            </div>
            <div className="flex flex-col pt-3">
                {countries.length > 0 ?
                    <div className="divide-gray-400 divide-y">
                        {countries.map((land) => <div key={land.record()} className="p-2">
                            <CountryListItem country={land} countries={countries} refresh={refreshCountries}/>
                        </div>)}
                    </div> :
                    <p className="p-2 text-gray-600">No land available!</p>}
            </div>
        </main>
    </>
})

export default CountryOverview