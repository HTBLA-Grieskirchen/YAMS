import { NavigationPage } from "../../../../types/layout";
import { observer } from "mobx-react";
import React from "react";
import { useRouter } from "next/router";
import { useStore } from "../../../../stores";
import Client from "../../../../model/client";
import Link from "next/link";
import paths from "../../../../util/paths";
import ClientDetail from "../index";

const ClientRelations: NavigationPage = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let client: Client | undefined

    if (typeof id !== "string" || (client = store.clientStore.indexedClients.get(id)) === undefined) return <div
        className="p-2">
        <p className="text-2xl font-bold">Unknown Client</p>
    </div>

    const relations = store.clientStore.relationStore.indexedByFrom.get(client.record.join())

    return <p>
        {relations && relations.map((relation) => <li key={relation.record.join()}>{relation.relationType}</li>)}
    </p>
})

ClientRelations.NavMenu = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let client: Client | undefined

    if (typeof id !== "string" || (client = store.clientStore.indexedClients.get(id)) === undefined) return <></>

    return <>
        <li>
            <Link href={paths.clientRelations(id)}>
                <a><i className="fa-solid fa-plus"/>Add</a>
            </Link>
        </li>
    </>
})

ClientRelations.NavPath = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let client: Client | undefined

    if (typeof id !== "string" || (client = store.clientStore.indexedClients.get(id)) === undefined) return <>
        {ClientDetail.NavPath && <ClientDetail.NavPath/>}
    </>

    const path = paths.clientRelations(id)
    const disabled = router.asPath == path

    return <>
        {ClientDetail.NavPath && <ClientDetail.NavPath/>}
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost btn-sm px-2 normal-case font-normal text-lg ${disabled ? "pointer-events-none" : ""}`}>
                    Relations
                </button>
            </Link>
        </li>
    </>
})

export default ClientRelations
