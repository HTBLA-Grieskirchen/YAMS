import {observer} from "mobx-react";
import Head from "next/head";
import {CountryCreation, CountryListItem} from "../../../components/CountryItem";
import {useState} from "react";
import {NextLayoutPage} from "../../../types/layout";
import AddressLayout from "../_layout";
import {useStore} from "../../../stores";

const CountryOverview: NextLayoutPage = observer(() => {
    const store = useStore()
    const countries = store.addressStore.countries

    const [addingCountry, setAddingCountry] = useState(false)

    return <>
        <Head>
            <title>YAMS - Country</title>
        </Head>

        <main className="flex flex-col w-fill m-5 p-3 rounded-lg bg-gray-200 shadow">
            <div className="flex">
                {addingCountry ?
                    <div>
                        <CountryCreation countries={store.addressStore.countries} onFinish={(successful) => {
                            if (successful?.result) {
                                store.addressStore.refresh()
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
                        {countries.map((land) => <div key={land.record.join()} className="p-2">
                            <CountryListItem country={land} countries={countries} refresh={store.addressStore.refresh}/>
                        </div>)}
                    </div> :
                    <p className="p-2 text-gray-600">No land available!</p>}
            </div>
        </main>
    </>
})

CountryOverview.Layout = AddressLayout

export default CountryOverview