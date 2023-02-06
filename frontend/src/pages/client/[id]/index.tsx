import { observer } from "mobx-react";
import Link from "next/link";
import paths from "../../../util/paths";
import { useRouter } from "next/router";
import React from "react";
import { NavigationPage } from "../../../types/layout";
import ClientOverview from "../index";
import Client from "../../../model/client";
import { useStore } from "../../../stores";

const ClientDetail: NavigationPage = observer(() => {
    return <p>Unknown Client (Not implemented)</p>
})

ClientDetail.NavMenu = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let client: Client | undefined

    if (typeof id !== "string" || (client = store.clientStore.indexedClients.get(id)) === undefined) return <></>

    return <>
        <li>
            <Link href={paths.clientRelations(id)}>
                <a><i className="fa-solid fa-diagram-project"/>Relations</a>
            </Link>
        </li>
    </>
})

ClientDetail.NavPath = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let client: Client | undefined

    if (typeof id !== "string" || (client = store.clientStore.indexedClients.get(id)) === undefined) return <>
        {ClientOverview.NavPath && <ClientOverview.NavPath/>}
        <li>
            <button
                className={`btn btn-ghost btn-sm px-2 normal-case font-normal text-lg pointer-events-none`}>
                Invalid Client
            </button>
        </li>
    </>

    const path = paths.client(id)
    const disabled = router.asPath == path

    return <>
        {ClientOverview.NavPath && <ClientOverview.NavPath/>}
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost btn-sm px-2 normal-case font-normal text-lg ${disabled ? "pointer-events-none" : ""}`}>
                    Client {`"${client.firstName} ${client.lastName}"`}
                </button>
            </Link>
        </li>
    </>
})

export default ClientDetail
