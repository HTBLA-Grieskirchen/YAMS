import {observer} from "mobx-react";
import Head from "next/head";
import {useQuery} from "../../../libs/dbConnection";
import Land from "../../../model/land";
import Link from "next/link";

const LandOverview = observer(() => {
    const [landsRaw, refreshLands] = useQuery("SELECT * FROM land", undefined, 1000)
    const lands: Land[] = landsRaw.length > 0 && landsRaw[0].result ? landsRaw[0].result.map((landRaw: any) => {
        if (landRaw.id !== undefined && landRaw.name !== undefined && landRaw.short !== undefined) {
            return new Land(landRaw.id, landRaw.name, landRaw.short)
        }
    }).filter((it: any) => it !== undefined) : []

    return <>
        <Head>
            <title>YAMS - Lands</title>
        </Head>

        <main className="flex flex-row w-fill m-5 rounded-lg bg-gray-200 shadow place-content-between">
            <div className="p-5">
                {lands.map((land) => <div></div>)}
            </div>
            <div className="flex m-3">
                <Link href="/addresses/land/add">
                    <a className="p-2 m-1 hover:p-3 hover:m-0
                     border border-black
                     rounded
                     shadow
                     bg-blue-100 hover:bg-blue-300
                     transition-all">
                        New Land
                    </a>
                </Link>
            </div>
        </main>
    </>
})

export default LandOverview