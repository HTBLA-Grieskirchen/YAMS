import {observer, useLocalObservable} from "mobx-react";
import {categories} from "../../pages/purchases";
import Purchase from "../../model/purchase";
import {ReactNode, useState} from "react";
import {useStore} from "../../stores";
import Client from "../../model/client";
import {MD5} from "object-hash";
import dialog from "../../libs/dialog";
import notification from "../../libs/notification";
import {patchPurchasesDynamic} from "../../libs/database/purchase";
import {runInAction} from "mobx";
import Link from "next/link";
import paths from "../../util/paths";

export const PurchaseTableHeader = observer((
    {selectedCategories}:
        { selectedCategories: Map<string, boolean> }
) => {
    return <tr>
        {Object.entries(categories).map(([field, {humanReadable}]) => {
            return <th key={field}>{humanReadable}</th>
        })}
        <th></th>
    </tr>
})

const initialEditState = {
    editing: false,
    submitted: false
}
type EditState = typeof initialEditState

export const PurchaseTableRow = observer((
    {selectedCategories, facingValues, backingPurchases}:
        { selectedCategories: Map<string, boolean>, facingValues: { [key: string]: string }, backingPurchases: Purchase[] }
) => {
    const store = useStore()

    const editState = useLocalObservable(() => initialEditState)
    const [expanded, setExpanded] = useState(false)

    const usingClients = store.clientStore.clients.filter((client) => backingPurchases.find((purchase) => client.purchase == purchase))
    const usingEvents: any[] = [] // TODO: Implement once events are available
    const clientsCount = usingClients.length
    const eventsCount = usingEvents.length
    const usages = clientsCount + eventsCount

    return <>
        <tr onKeyDown={e => {
            if (e.key == "Escape") {
                runInAction(() => editState.editing = false)
            }
        }}>
            {editState.editing ?
                <PurchaseEditTableRowContent selectedCategories={selectedCategories} facingValues={facingValues}
                                            editState={editState} usageAmount={usages}/> :
                <PurchaseViewTableRowContent selectedCategories={selectedCategories} facingValues={facingValues}
                                            editState={editState} usageAmount={usages}/>
            }
        </tr>

        <tr>
            <td colSpan={100}>
                <div className={`collapse ${expanded ? "collapse-open" : "collapse-close"}`}>
                    <div className={`collapse-title text-xl font-normal p-0 ${expanded ? "-mb-2" : "-mb-6"}`}>
                        <button className={`btn btn-circle btn-ghost btn-sm mr-2 ${usages <= 0 && "invisible"}`}
                                onClick={e => setExpanded(!expanded)}>
                            <i className={`fa-solid fa-chevron-down transition-transform ${expanded ? "rotate-180" : "rotate-0"}`}/>
                        </button>
                        <span className="font-bold">{usages}</span> Usages
                    </div>
                    <div className={`collapse-content ${expanded && "-mb-2"}`}>
                        <PurchaseUsageInfo clients={usingClients} events={usingEvents}/>
                    </div>
                </div>
            </td>
        </tr>
    </>
})

const PurchaseViewTableRowContent = observer((
    {selectedCategories, facingValues, editState, usageAmount}:
        { selectedCategories: Map<string, boolean>, facingValues: { [key: string]: string }, editState: EditState, usageAmount: number }
) => {
    return <>
        {Object.keys(categories).map((field) => {
            return <NoBorderTD key={field}>
                {selectedCategories.get(field) ?
                    <>{(facingValues as any)[field]}</> :
                    <div className="badge px-4">-</div>
                }
            </NoBorderTD>
        })}
        <NoBorderTD>
            <div className="relative flex place-content-end">
                <div className="invisible">
                    <EditButtons/>
                </div>
                <EditButton entries={usageAmount} onClick={() => runInAction(() => editState.editing = true)}/>
            </div>
        </NoBorderTD>
    </>
})

const PurchaseEditTableRowContent = observer((
    {selectedCategories, facingValues, editState, usageAmount}:
        { selectedCategories: Map<string, boolean>, facingValues: { [key: string]: string }, editState: EditState, usageAmount: number }
) => {
    const store = useStore()
    const id = MD5(facingValues)

    const editValue = useLocalObservable(() => ({...facingValues}))
    const errors = useLocalObservable(() => {
        const map = new Map<string, string | null>()

        for (const category of Object.keys(categories)) {
            map.set(category, null)
        }

        return map
    })

    function askSubmit() {
        if (usageAmount > 1) {
            dialog((close) => <div className="modal-box">
                <h3 className="font-bold text-lg">
                    <i className="fa-solid fa-exclamation-triangle text-warning mr-1"/>
                    This action will change {usageAmount} entries!
                </h3>
                <p className="py-4">Are you sure you want to update this many items? All purchases in this category will
                    be changed.</p>
                <div className="modal-action">
                    <button className="btn btn-success" onClick={e => {
                        close()
                        submit()
                    }}>{"I'm sure!"}
                    </button>
                    <button className="btn btn-error" onClick={e => close()}>Cancel</button>
                </div>
            </div>)
        } else {
            submit()
        }
    }

    async function submit() {
        runInAction(() => editState.submitted = true)

        const prev = Object.fromEntries(Object.entries(facingValues).map(([field, value]) => [categories[field].databaseField, value]))
        const next = Object.fromEntries(Object.entries(editValue).map(([field, value]) => [categories[field].databaseField, value]))
        const result = await patchPurchasesDynamic(prev, next)
        if (result.error) {
            notification.error({
                title: "Purchase cannot be updated!",
                message: `"${result.error.message}". Do you want to try again?`
            }, 10, {
                "Retry": {
                    action: async () => {
                        await submit()
                        return true
                    },
                    disabled: () => editState.submitted
                }
            })
        } else {
            await store.purchaseStore.refresh()
            runInAction(() => editState.editing = false)
        }

        runInAction(() => editState.submitted = false)
    }

    return <>
        {Object.entries(categories).map(([field, {humanReadable, validation}]) => {
            return <NoBorderTD key={field}>
                {selectedCategories.get(field) ?
                    <div className="form-control w-full max-w-xs">
                        <input type="text" form={id} placeholder={`Type ${humanReadable.toLowerCase()}`}
                               value={(editValue as any)[field]}
                               onChange={e => {
                                   runInAction(() => {
                                       errors.set(field, validation ? validation(e.target.value) : null);
                                       (editValue as any)[field] = e.target.value
                                   })
                               }}
                               className={`input ${errors.get(field) == null ? "input-accent" : "input-error"} input-sm max-w-full`}/>
                        {!!errors.get(field)?.length &&
                            <label className="label">
                                <span className="label-text text-error">* {errors.get(field)}</span>
                            </label>
                        }
                    </div> :
                    <div className="badge px-4">-</div>
                }
            </NoBorderTD>
        })}
        <NoBorderTD>
            <div className="relative">
                <EditButtons id={id} invalid={!Array.from(errors.values()).every((error) => error == null)}
                             onConfirm={() => askSubmit()} onCancel={() => runInAction(() => editState.editing = false)}
                             submitted={editState.submitted}/>
                <EditButton entries={usageAmount} hidden/>
            </div>
        </NoBorderTD>
    </>
})

const EditButtons = observer((
    {onConfirm, onCancel, id, invalid, submitted}:
        { onConfirm?: () => void, onCancel?: () => void, id?: string, invalid?: boolean, submitted?: boolean }
) => {
    return <form id={id} className="flex flex-row space-x-10 place-content-end" onSubmit={e => {
        e.preventDefault()
        onConfirm && onConfirm()
    }}>
        <button className={`btn btn-success btn-sm -mx-4 
        ${invalid ? "cursor-not-allowed" : ""} 
        ${submitted ? "loading" : ""}`} disabled={invalid || submitted}>
            Confirm
        </button>

        <button type="button" className="btn btn-error btn-sm -mx-4" onClick={e => onCancel && onCancel()}
                disabled={submitted}>
            Cancel
        </button>
    </form>
})

const EditButton = observer((
    {onClick, entries, hidden}:
        { onClick?: () => void, entries: number, hidden?: boolean }
) => {
    return <button className={`absolute top-0 right-0 btn btn-sm ${hidden ? "invisible" : ""}`}
                   onClick={e => onClick && onClick()}>
        Edit {!!entries && (entries != 1 ?
        <><span className="font-bold px-1">{entries}</span>entries</> :
        <>entry</>)
    }
    </button>
})

const PurchaseUsageInfo = observer((
    {clients, events}:
        { clients: Client[], events: unknown[] }
) => {
    // TODO: Add support for events once implemented
    type Tabs = "clients" | "events"
    const [selectedTab, setSelectedTab] = useState<Tabs>(!clients.length ? "events" : "clients")

    return <div className="grid">
        <div className="tabs -mb-px">
            <div className={`tab tab-lifted 
                ${!clients.length ? "cursor-no-drop" : ""}
                ${selectedTab == "clients" ? "tab-active" : ""}
                `} onClick={e => !!clients.length && setSelectedTab("clients")}>
                Clients (<span className="font-bold">{clients.length}</span>)
            </div>
            <div className={`tab tab-lifted 
                ${!events.length ? "cursor-no-drop" : ""}
                ${selectedTab == "events" ? "tab-active" : ""}
                `} onClick={e => !!events.length && setSelectedTab("events")}>
                Events (<span className="font-bold">{events.length}</span>)
            </div>
        </div>
        <div className="w-full max-h-72 relative overflow-auto">
            <div className="bg-neutral rounded-b-box rounded-tr-box px-4 py-2">
                {
                    selectedTab == "clients" ? <ClientUsages clients={clients}/> :
                        selectedTab == "events" ? <EventsUsages events={events}/> :
                            <></>
                }
            </div>
        </div>
    </div>
})

const ClientUsages = observer((
    {clients}:
        { clients: Client[] }
) => {
    // TODO: Link to client detail page once implemented (via path)

    return <div className="flex flex-col divide-y divide-base-300 w-full">
        {clients.map((client) => {
            return <div key={client.record.join()} className="w-full flex place-content-between">
                <div className="flex flex-col my-1">
                    <div className="flex flex-row">
                        <p className="text-lg font-medium mb-1 mr-2">{client.firstName} {client.lastName}
                            <span className="font-normal">
                            , {client.birthdate.getFullYear()}
                            </span>
                        </p>
                        {client.consent ? <div className="badge badge-success gap-1 mt-1">
                            <i className="fa-solid fa-check"/>
                            Consent given
                        </div> : <div className="badge badge-error gap-1 mt-1">
                            <i className="fa-solid fa-x"/>
                            No consent
                        </div>}
                    </div>
                    <div className="flex flex-row">
                        <p className="text-md">{client.email}</p>
                        <div className="tooltip tooltip-accent tooltip-right w-fit cursor-help"
                             data-tip={`Message ${client.firstName} ${client.lastName} per E-Mail`}>
                            <a href={`mailto:${client.email}`} className="ml-2 link">
                                <i className="fa-solid fa-paper-plane"/>
                            </a>
                        </div>
                    </div>
                    <div className="flex flex-row">
                        <p className="text-md">{client.mobileNumber}</p>
                        <div className="tooltip tooltip-accent tooltip-right w-fit cursor-help"
                             data-tip={`Call ${client.firstName} ${client.lastName}`}>
                            <a href={`tel:${client.mobileNumber}`} className="ml-2 link">
                                <i className="fa-solid fa-phone"/>
                            </a>
                        </div>
                    </div>
                </div>

                <div className="tooltip tooltip-accent tooltip-left w-fit cursor-help my-auto"
                     data-tip={`View client ${client.firstName} ${client.lastName} in detail`}>
                    <Link href={paths.client(client.record.id)}>
                        <button className="btn btn-md my-2"><i className="fa-solid fa-external-link mr-2"/>Open</button>
                    </Link>
                </div>
            </div>
        })}
    </div>
})

const EventsUsages = observer((
    {events}:
        { events: unknown[] }
) => {
    return <p>
        Unimplemented
    </p>
})


// Helper
const NoBorderTD = ({children}: { children?: ReactNode }) => {
    return <td className={`border-b-0 pb-0 align-top`}>{children}</td>
}
