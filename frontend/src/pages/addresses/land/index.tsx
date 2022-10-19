import {observer} from "mobx-react";
import Head from "next/head";
import {useQuery} from "../../../libs/dbConnection";
import Land from "../../../model/land";
import {LandCreation, LandListItem} from "../../../components/LandItem";
import {useState} from "react";

const LandOverview = observer(() => {
    const [landsRaw, refreshLands] = useQuery("SELECT * FROM land ORDER BY name", undefined, 1000)
    const lands: Land[] = landsRaw.length > 0 && landsRaw[0].result ? landsRaw[0].result.map((landRaw: any) => {
        if (landRaw.id !== undefined && landRaw.name !== undefined && landRaw.short !== undefined) {
            return new Land(landRaw.id, landRaw.name, landRaw.short)
        }
    }).filter((it: any) => it !== undefined) : []

    const [addingLand, setAddingLand] = useState(false)

    return <>
        <Head>
            <title>YAMS - Lands</title>
        </Head>

        <main className="flex flex-col w-fill m-5 p-3 rounded-lg bg-gray-200 shadow">
            <div className="flex">
                {addingLand ?
                    <div>
                        <LandCreation lands={lands} onFinish={(successful) => {
                            if (successful?.result) {
                                refreshLands()
                            }
                            setAddingLand(false)
                        }}/>
                    </div> :
                    <button onClick={e => setAddingLand(true)} className="m-1 w-24 h-10 hover:text-lg
                     border border-black
                     rounded
                     shadow
                     bg-blue-100 hover:bg-blue-300
                     transition-all">
                        New Land
                    </button>}
            </div>
            <div className="flex flex-col pt-3">
                {lands.length > 0 ?
                    <div className="divide-gray-400 divide-y">
                        {lands.map((land) => <div key={land.record()} className="p-2">
                            <LandListItem land={land} refresh={refreshLands}/>
                        </div>)}
                    </div> :
                    <p className="p-2 text-gray-600">No land available!</p>}
            </div>
        </main>
    </>
})

export default LandOverview