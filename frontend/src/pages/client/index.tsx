import {observer} from "mobx-react";
import Head from "next/head";
import {NavigationPage} from "../../types/layout";
import {useStore} from "../../stores";
import paths from "../../util/paths";
import Link from "next/link";
import React from "react";
import {useRouter} from "next/router";
import {ClientTableHeader, ClientTableRow} from "../../components/client/ClientTable";

const ClientOverview: NavigationPage = observer(() => {
    const store = useStore()
    const clients = store.clientStore.clients

    return (
        <>
            <Head>
                <title>All Clients</title>
            </Head>

            <main className="w-full p-6">
                {clients.length > 0 ?
                    <table className="table w-full">
                        <thead className="sticky top-0 z-30">
                        <ClientTableHeader/>
                        </thead>
                        <tbody className="overflow-y-auto">
                        {clients.map((client) =>
                            <ClientTableRow key={client.record.join()} client={client}
                                            refresher={store.clientStore.refresh}/>
                        )}
                        </tbody>
                    </table>
                    :
                    <p className="text-lg font-medium mb-1 mr-2 text-red-900">No clients available!</p>
                }
            </main>
        </>
    )
})

ClientOverview.NavMenu = observer(() => {
    return <>
        <li>
            <Link href={paths.new_client}>
                <a><i className="fa-solid fa-plus"/>Add</a>
            </Link>
        </li>
    </>
})

ClientOverview.NavPath = observer(() => {
    const path = paths.clients

    const router = useRouter()
    const disabled = router.pathname == path

    return <>
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    <i className="fa-solid fa-person mr-2"/>
                    Client
                </button>
            </Link>
        </li>
    </>
})

export default ClientOverview