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

            {/*<main className="flex flex-col min-w-fit w-fit m-5 p-3 rounded-lg bg-gray-200 shadow">*/}
            {/*    <div className="flex flex-col pt-3">*/}
            {/*        {clients.length > 0 ?*/}
            {/*            <div className="divide-gray-400 divide-y">*/}
            {/*                {clients.map((client) =>*/}
            {/*                    <div key={client.record.join()} className="p-2">*/}
            {/*                        <ClientItem client={client} refresher={store.clientStore.refresh}/>*/}
            {/*                    </div>*/}
            {/*                )}*/}
            {/*            </div>*/}
            {/*            :*/}
            {/*            <p className="p-2 text-gray-600">*/}
            {/*                No client available!*/}
            {/*            </p>}*/}
            {/*    </div>*/}
            {/*</main>*/}

            <main className="w-full p-6">
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