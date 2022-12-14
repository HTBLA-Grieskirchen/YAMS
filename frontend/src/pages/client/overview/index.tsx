import {observer} from "mobx-react";
import Head from "next/head";
import {useLive} from "../../../libs/database";
import Client from "../../../model/client";
import ClientItem from "../../../components/client/ClientItem";
import {useState} from "react";
import {NextLayoutPage} from "../../../types/layout";
import ClientLayout from "../_layout";

const ClientOverview: NextLayoutPage = observer(() => {
    const [clientsRaw, refreshClients] = useLive("SELECT * FROM client ORDER BY last_name")
    const clients: Client[] = clientsRaw.response && clientsRaw.response.length > 0 && clientsRaw.response[0].result ? clientsRaw.response[0].result.map((client: any) => {
        if (client.id !== undefined && client.last_name !== undefined && client.first_name !== undefined && client.birthdate !== undefined
            && client.email !== undefined && client.mobile_number !== undefined && client.consent !== undefined) {
            return new Client(client.id, client.last_name, client.first_name, new Date(client.birthdate.split('T')[0]), client.email, client.mobile_number, client.consent, client.a)
        }
    }).filter((it: any) => it !== undefined) : []

    const [addEntry, setAddEntry] = useState(false)

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
                                <div key={client.record()} className="p-2">
                                    <ClientItem client={client} refresher={refreshClients}/>
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