import {observer} from "mobx-react";
import Head from "next/head";
import ClientItem from "../../../components/client/ClientItem";
import {NextLayoutPage} from "../../../types/layout";
import ClientLayout from "../_layout";
import {useStore} from "../../../stores";

const ClientOverview: NextLayoutPage = observer(() => {
    const store = useStore()
    const clients = store.clientStore.clients

    return (
        <>
            <Head>
                <title>YAMS - Clients</title>
            </Head>

            <main className="flex flex-col min-w-fit w-[52%] m-5 p-3 rounded-lg bg-gray-200 shadow">
                <div className="flex flex-col pt-3">
                    {clients.length > 0 ?
                        <div className="divide-gray-400 divide-y">
                            {clients.map((client) =>
                                <div key={client.record.join()} className="p-2">
                                    <ClientItem client={client} refresher={store.clientStore.refresh}/>
                                </div>
                            )}
                        </div>
                        :
                        <p className="p-2 text-gray-600">
                            No client available!
                        </p>}
                </div>
            </main>
        </>
    )
})

ClientOverview.Layout = ClientLayout

export default ClientOverview