import {NavigationPage} from "../../../../types/layout";
import {observer, useLocalObservable} from "mobx-react";
import React, {useState} from "react";
import {useRouter} from "next/router";
import {useStore} from "../../../../stores";
import Client from "../../../../model/client";
import Link from "next/link";
import paths from "../../../../util/paths";
import ClientDetail from "../index";
import dialog from "../../../../libs/dialog";
import {ValidatableComboBox} from "../../../../components/form/combobox";
import {ValidatableFieldData} from "../../../../libs/field/validatable";
import {Combobox} from "@headlessui/react";
import notification from "../../../../libs/notification";
import {relateClients} from "../../../../libs/database/clientRelation";
import Head from "next/head";

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

    return <main className="p-6">
        <Head>
            <title>YAMS - Relations Client</title>
        </Head>

        <p>
            {relations && relations.map((relation) => <li key={relation.record.join()}>
                --{relation.relationType}-{`>`} {relation.dest.firstName} {relation.dest.lastName}
            </li>)}
        </p>
    </main>
})

ClientRelations.NavMenu = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let client: Client | undefined

    if (typeof id !== "string" || (client = store.clientStore.indexedClients.get(id)) === undefined) return <></>

    return <>
        {store.clientStore.clients.length > 1 && <li>
            <a onClick={e => dialog((close) => <AddRelationDialog close={close} fromClient={client!}/>)}>
                <i className="fa-solid fa-plus"/>Add
            </a>
        </li>}
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
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    Relations
                </button>
            </Link>
        </li>
    </>
})


const AddRelationDialog = observer((
    {close, fromClient}:
        { close: () => void, fromClient: Client }
) => {
    const [submitted, setSubmitted] = useState(false)

    const store = useStore()
    const relations = store.clientStore.relationStore
    const potentialTargetClients = store.clientStore.clients.filter((potential) => potential != fromClient)
    const potentialRelationTypes = Array.from(new Set(relations.relations.map((relation) => relation.relationType)).values())

    console.log(potentialRelationTypes)

    const typesFromTarget = () => {
        const outgoingRelations = relations.indexedByFrom.get(fromClient.record.join())
        return outgoingRelations === undefined ? [] :
            outgoingRelations.filter((relation) =>
                relation.dest == targetClient?.value
            ).map((relation) => relation.relationType)
    }

    const targetClient = useLocalObservable(() =>
        new ValidatableFieldData<Client | null>(null, (value) => {
            if (value != null) return null

            return "You have to select a client"
        }))
    const [targetClientQuery, setTargetClientQuery] = useState("")

    const relationType = useLocalObservable(() =>
        new ValidatableFieldData("", (value) => {
            if (value.trim().length < 1) {
                return "Type must be specified"
            } else if (typesFromTarget().some((type) => type.toLowerCase().trim() == value.toLowerCase().trim())) {
                return "This relationship already exists"
            }

            return null
        }))


    const filteredTargetClients = targetClientQuery.trim().length < 1 ? potentialTargetClients :
        potentialTargetClients.filter((client) => {
            return targetClientQuery.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
                const trimmedSegment = segment.trim().toLowerCase()
                if (trimmedSegment.length < 0) return true

                return client.firstName.toLowerCase().includes(trimmedSegment) ||
                    client.lastName.toLowerCase().includes(trimmedSegment) ||
                    client.email.toLowerCase().includes(trimmedSegment) ||
                    client.mobileNumber.toLowerCase().includes(trimmedSegment)
            })
        })
    const filteredRelationTypes = relationType.value.trim().length < 1 ? potentialRelationTypes :
        potentialRelationTypes.filter((relation) => {
            return relationType.value.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
                const trimmedSegment = segment.trim().toLowerCase()
                if (trimmedSegment.length < 0) return true

                return relation.toLowerCase().includes(trimmedSegment)
            })
        })


    const addRelation = async () => {
        setSubmitted(true)

        const result = await relateClients(fromClient.record, targetClient.value!.record, relationType.value)
        if (result.error) {
            notification.error({
                title: "Relation cannot be saved!",
                message: `"${result.error.message}". Do you want to try again?`
            }, 15, {
                "Retry": {
                    action: async () => {
                        await addRelation()
                        return true
                    },
                    disabled: () => submitted
                }
            })
        } else {
            await store.addressStore.refresh()
            close()
        }

        setSubmitted(false)
    }

    return <form className="modal-box max-w-none w-fit overflow-visible" onSubmit={e => {
        e.preventDefault()
        addRelation().then()
    }}>
        <h3 className="font-bold text-lg">
            Add new relation...
        </h3>
        <div className="flex grow space-x-4 py-4">
            <ValidatableComboBox data={targetClient} label="Other client"
                                 placeholder="Max Mustermann, 01.02.2003"
                                 className="max-w-md basis-7/12"
                                 mapDisplayValue={(value: typeof targetClient.value) => {
                                     return value == null || !(value instanceof Client) ? "" :
                                         `${value.firstName} ${value.lastName}, ${value.birthdate.toLocaleDateString()}`
                                 }} setQuery={setTargetClientQuery}>
                {filteredTargetClients.map((item) =>
                    <Combobox.Option className={`${item == targetClient.value ? "active" : ""}`}
                                     key={item.record.join()} value={item}>
                        <a className="flex items-center">
                            <div className={`badge badge-xs ${item.consent ? "badge-success" : "badge-error"}`}></div>
                            <div className="flex flex-col">
                                <p className="font-medium">{`${item.firstName} ${item.lastName}, ${item.birthdate.toLocaleDateString()}`}</p>
                                <p>{item.mobileNumber}, <span className="text-primary">{item.email}</span></p>
                            </div>
                        </a>
                    </Combobox.Option>
                )}
            </ValidatableComboBox>

            <ValidatableComboBox data={relationType} label="Relation type"
                                 placeholder="Legal guardian" disabled={targetClient.value == null}
                                 className="max-w-sm basis-5/12"
                                 mapDisplayValue={(value: typeof relationType.value) => {
                                     return value
                                 }} setQuery={relationType.setValue}>
                {filteredRelationTypes.map((item) =>
                    <Combobox.Option className={`${item == relationType.value.trim() ? "active" : ""}`}
                                     key={item} value={item}>
                        <a>{item}</a>
                    </Combobox.Option>
                )}
            </ValidatableComboBox>
        </div>
        {!!typesFromTarget().length &&
            <div className="prose w-full">
                <p className="text-base-content mb-0">
                    Relations with {targetClient.value!.firstName} {targetClient!.value!.lastName}
                </p>
                <ul className="mt-0">
                    {typesFromTarget().map((type) => <li key={type} className="my-0">
                        {type}
                    </li>)}
                </ul>
            </div>}
        <div className="modal-action">
            <button className="btn btn-error" type="button" onClick={e => close()}>
                Cancel
            </button>
            <button className={`btn btn-success ${submitted ? "loading" : ""}`} type="submit"
                    disabled={[targetClient, relationType].some((value) => !value.valid) || submitted}>
                Relate!
            </button>
        </div>
    </form>
})

export default ClientRelations
