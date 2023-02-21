import { NavigationPage } from "../../../../types/layout";
import { observer, useLocalObservable } from "mobx-react";
import React, { useState } from "react";
import { useRouter } from "next/router";
import { useStore } from "../../../../stores";
import Client from "../../../../model/client";
import Link from "next/link";
import paths from "../../../../util/paths";
import ClientDetail from "../index";
import dialog from "../../../../libs/dialog";
import { ValidatableComboBox } from "../../../../components/form/combobox";
import { ValidatableFieldData } from "../../../../libs/field/validatable";
import { Combobox } from "@headlessui/react";
import notification from "../../../../libs/notification";
import { deleteClientRelation, relateClients, updateClientRelation } from "../../../../libs/database/clientRelation";
import Head from "next/head";
import { query } from "../../../../libs/database";
import ClientRelation from "../../../../model/relation";
import { groupBy } from "../../../../util/helpers";
import { runInAction } from "mobx";

const ClientRelations: NavigationPage = observer(() => {
    const router = useRouter()
    const store = useStore()

    const {id} = router.query
    let client: Client | undefined

    if (typeof id !== "string" || (client = store.clientStore.indexedClients.get(id)) === undefined)
        return <div className="p-6 overflow-y-auto h-full w-full flex flex-col items-center justify-center space-y-4">
            <i className="text-9xl text-error fa-solid fa-user-slash"/>
            <p className="text-6xl font-bold text-error">Unknown Client</p>
        </div>

    const filter = useLocalObservable(() => ({
        value: "",
        setValue(filter: string) {
            this.value = filter
        }
    }))

    const relations = store.clientStore.relationStore.indexedByFrom.get(client.record.join()) ?? []
    const relationsFiltered = relations.filter((relation) => clientSearched(relation.dest, filter.value))

    return <>
        <Head>
            <title>YAMS - Relations Client</title>
        </Head>

        <main className="p-6 overflow-y-auto h-full">
            <div className="card card-compact bg-base-100 shadow max-h-full">
                <div className="card-body overflow-visible overflow-x-auto">
                    <div className="flex flex-col lg:flex-row justify-between space-y-2 lg:space-y-0">
                        <h2 className="card-title">{`${client.firstName} ${client.lastName}'s Relations`}</h2>
                        <div className="form-control">
                            <div className="input-group input-group-sm">
                                <input type="text" placeholder="Search term..."
                                       className="input input-bordered input-sm input-primary"
                                       value={filter.value} onChange={e => filter.setValue(e.target.value)}/>
                                <span className="px-2">
                                    <i className="fa-solid fa-magnifying-glass"/>
                                </span>
                            </div>
                        </div>
                    </div>

                    <div className="divider my-0"/>

                    <div className="overflow-y-auto relative">
                        {!relations.length ?
                            <div className="alert alert-info">
                                <div className="text-xl">
                                    <i className="fa-solid fa-warning flex-shrink-0 text-2xl mr-2"/>
                                    This client has no relations.
                                </div>
                            </div> : !relationsFiltered.length ?
                                <div className="alert alert-info">
                                    <div className="text-xl">
                                        <i className="fa-solid fa-warning flex-shrink-0 text-2xl mr-2"/>
                                        This client has no relations with other filtered clients.
                                    </div>
                                </div> :
                                <div className="flex flex-col space-y-4">
                                    {Object.entries(groupBy(relationsFiltered, (relation) => relation.dest.record.join()))
                                        .map(([group, values]) => <RelationsPerClient key={group} relations={values}/>)}
                                </div>}
                    </div>
                </div>
            </div>
        </main>
    </>
})
export default ClientRelations

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

    const bidirectional = useLocalObservable(() => ({
        value: false,
        setValue(value: boolean) {
            this.value = value
        },
        get isPossible() {
            const incomingRelations = relations.indexedByDest.get(fromClient.record.join())
            return (incomingRelations === undefined ? [] :
                incomingRelations.filter((relation) =>
                    relation.from == targetClient?.value
                )).every((relation) => relation.relationType.toLowerCase().trim() != relationType.value.toLowerCase().trim())
        }
    }))


    const filteredTargetClients = targetClientQuery.trim().length < 1 ? potentialTargetClients :
        potentialTargetClients.filter((client) => {
            return clientSearched(client, targetClientQuery)
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

        const erroredResult = async () => {
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

                await query("CANCEL TRANSACTION")
                setSubmitted(false)
                return true
            }

            return false
        }

        await query("BEGIN TRANSACTION")

        let result = await relateClients(fromClient.record, targetClient.value!.record, relationType.value)
        if (await erroredResult()) return

        if (bidirectional.value) {
            result = await relateClients(targetClient.value!.record, fromClient.record, relationType.value)
            if (await erroredResult()) return
        }

        await query("COMMIT TRANSACTION")
        setSubmitted(false)
        close()
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

            <div className="form-control grow mt-8">
                <label className={`label ${bidirectional.isPossible ? "cursor-pointer" : "cursor-help"} w-full`}>
                    <div className="label-text text-left tooltip tooltip-accent tooltip-top"
                         data-tip="The relation will be created in both directions">
                        <span
                            className={`${!bidirectional.isPossible ? "opacity-50" : ""}`}>Bidirectional connection</span>
                    </div>
                    <input type="checkbox" checked={bidirectional.value}
                           disabled={!bidirectional.isPossible}
                           onChange={e => bidirectional.setValue(e.target.checked)}
                           className="checkbox checkbox-primary"/>
                </label>
            </div>
        </div>
        {!!typesFromTarget().length &&
            <div className="prose w-full">
                <p className="text-base-content mb-0">
                    Relations to {targetClient.value!.firstName} {targetClient!.value!.lastName}
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

const RelationsPerClient = observer((
    {relations}:
        { relations: ClientRelation[] }
) => {
    const destClient = relations[0].dest

    return <div className="flex flex-col space-y-2">
        <div className="sticky top-0 w-full bg-base-200 rounded-btn p-2 flex flex-row items-center space-x-2">
            <div className="tooltip tooltip-accent tooltip-right w-fit cursor-help my-auto"
                 data-tip={`View ${destClient.firstName} ${destClient.lastName} in detail`}>
                <Link href={paths.client(destClient.record.join())}>
                    <button className="btn btn-sm px-4"><i className="fa-solid fa-external-link"/></button>
                </Link>
            </div>
            <div className="flex flex-col">
                <p className="font-medium text-lg">{destClient.firstName} {destClient.lastName}, {destClient.birthdate.toLocaleDateString()}</p>
                <p>{destClient.mobileNumber}, <span className="text-primary">{destClient.email}</span></p>
            </div>
        </div>
        <div className="grid grid-cols-3 px-2 justify-items-center gap-1">
            {relations.map((relation) => <Relation key={relation.record.join()} relation={relation}/>)}
        </div>
    </div>
})

const Relation = observer((
    {relation}:
        { relation: ClientRelation }
) => {
    const store = useStore()
    const relations = store.clientStore.relationStore
    const mutability = useLocalObservable(() => ({
        deleteSubmitted: false,
        changeSubmitted: false,
        changeValue: new ValidatableFieldData(relation.relationType, (value) => {
            const outgoingRelations = relations.indexedByFrom.get(relation.from.record.join())
            const otherTypes = outgoingRelations === undefined ? [] :
                outgoingRelations.filter((outgoingRelation) =>
                    outgoingRelation.dest == relation.dest
                ).map((relation) => relation.relationType)

            if (value.trim().length < 1) {
                return "Type must be specified"
            } else if (otherTypes.some((type) => type.toLowerCase().trim() == value.toLowerCase().trim())) {
                return "This relationship already exists"
            }

            return null
        })
    }))

    const submitDelete = async () => {
        runInAction(() => mutability.deleteSubmitted = true)

        const result = await deleteClientRelation(relation.record)
        if (result.error) {
            notification.error({
                title: "Relation could not be deleted!",
                message: `"${result.error.message}". Do you want to try again?`
            }, 10, {
                "Retry": {
                    action: async () => {
                        await submitDelete()
                        return true
                    },
                    disabled: () => mutability.deleteSubmitted
                }
            })
        } else {
            await store.clientStore.relationStore.refresh()
        }

        runInAction(() => mutability.deleteSubmitted = false)
    }

    const submitChange = async () => {
        runInAction(() => mutability.changeSubmitted = true)

        const result = await updateClientRelation(relation, mutability.changeValue.value)
        if (result.error) {
            notification.error({
                title: "Relation could not be updated!",
                message: `"${result.error.message}". Do you want to try again?`
            }, 10, {
                "Retry": {
                    action: async () => {
                        await submitChange()
                        return true
                    },
                    disabled: () => mutability.changeSubmitted
                }
            })
        } else {
            await store.clientStore.relationStore.refresh()
        }

        runInAction(() => mutability.changeSubmitted = false)
    }

    const DeleteDialog = observer((
        {close}:
            { close: (all?: boolean) => void }
    ) => {
        return <div className="modal-box">
            <h3 className="font-bold text-lg">
                <i className="fa-solid fa-exclamation-triangle text-warning mr-1"/>
                This action will delete the relation!
            </h3>
            <p className="py-4">{`Are you sure you want to delete relation "${relation.relationType}" with ${relation.dest.firstName} ${relation.dest.lastName}?`}</p>
            <div className="modal-action">
                <button className="btn" onClick={e => close()}>Abort</button>
                <button className="btn btn-primary" onClick={e => {
                    close()
                    submitDelete().then()
                }}>
                    Delete
                </button>
            </div>
        </div>
    })

    const ChangeDialog = observer((
        {close}:
            { close: (all?: boolean) => void }
    ) => {
        const potentialRelationTypes = Array.from(new Set(relations.relations.map((relation) => relation.relationType)).values())
        const filteredRelationTypes = mutability.changeValue.value.trim().length < 1 ? potentialRelationTypes :
            potentialRelationTypes.filter((relation) => {
                return mutability.changeValue.value.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
                    const trimmedSegment = segment.trim().toLowerCase()
                    if (trimmedSegment.length < 0) return true

                    return relation.toLowerCase().includes(trimmedSegment)
                })
            })

        return <form className="modal-box overflow-visible" onSubmit={e => {
            e.preventDefault()
            close()
            submitChange().then()
        }}>
            <h3 className="font-bold text-lg">
                Editing relation type...
            </h3>
            <ValidatableComboBox data={mutability.changeValue} label="Relation type"
                                 placeholder={relation.relationType}
                                 className="max-w-sm" mapDisplayValue={(value) => value}
                                 setQuery={mutability.changeValue.setValue}>
                {filteredRelationTypes.map((item) =>
                    <Combobox.Option className={`${item == mutability.changeValue.value.trim() ? "active" : ""}`}
                                     key={item} value={item}>
                        <a>{item}</a>
                    </Combobox.Option>
                )}
            </ValidatableComboBox>
            <div className="modal-action">
                <button className="btn" onClick={e => close()}>Abort</button>
                <button className="btn btn-primary" onClick={e => {
                    close()
                    submitChange().then()
                }} disabled={!mutability.changeValue.valid}>
                    Change
                </button>
            </div>
        </form>
    })

    return <div className="flex flex-row">
        <p className="self-center text-md mr-2">{relation.relationType}</p>
        <button className={`btn btn-square btn-xs btn-ghost text-error ${mutability.deleteSubmitted ? "loading" : ""}`}
                onClick={e => dialog((close) => <DeleteDialog close={close}/>)}
                disabled={mutability.deleteSubmitted || mutability.changeSubmitted}>
            {!mutability.deleteSubmitted && <i className="fa-solid fa-trash-can"/>}
        </button>
        <button
            className={`btn btn-square btn-xs btn-ghost text-secondary ${mutability.changeSubmitted ? "loading" : ""}`}
            onClick={e => dialog((close) => <ChangeDialog close={close}/>)}
            disabled={mutability.deleteSubmitted || mutability.changeSubmitted}>
            {!mutability.changeSubmitted && <i className="fa-solid fa-edit"/>}
        </button>
    </div>
})

function clientSearched(client: Client, query: string) {
    return query.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
        const trimmedSegment = segment.trim().toLowerCase()
        if (trimmedSegment.length < 0) return true

        return client.firstName.toLowerCase().includes(trimmedSegment) ||
            client.lastName.toLowerCase().includes(trimmedSegment) ||
            client.email.toLowerCase().includes(trimmedSegment) ||
            client.mobileNumber.toLowerCase().includes(trimmedSegment)
    })
}
