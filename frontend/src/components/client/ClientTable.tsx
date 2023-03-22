import {observer} from "mobx-react";
import Client from "../../model/client";
import React, {useState} from "react";
import {deleteClient} from "./ClientItem";
import {LiveRefresher} from "../../libs/database";
import Link from "next/link";
import paths from "../../util/paths";
import AnimalList from "../animal/AnimalList";

export const ClientTableHeader = observer(() => {
    return <tr>
        <th></th>
        <th>Last Name</th>
        <th>First Name</th>
        <th>Birth Date</th>
        <th></th>
        <th></th>
        <th></th>
    </tr>
})

export const ClientTableRow = observer((
    {client, refresher}: { client: Client, refresher: LiveRefresher }
) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [showDetail, setShowDetail] = useState(false)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)

        const result = await deleteClient(client)

        await refresher()
        setDeleteSubmitted(false)
    }

    return <>
        <tr>
            <td>
                {client.consent ?
                    <i className="fa-solid fa-circle-check text-green-600"/>
                    :
                    <i className="fa-solid fa-circle-exclamation text-red-600"/>}
            </td>
            <td>{client.lastName}</td>
            <td>{client.firstName}</td>
            <td>{client.birthdate.toLocaleDateString()}</td>
            <td>
                <button
                    type="button"
                    onClick={() => setShowDetail(!showDetail)}
                    className="align-text-bottom text-2xl hover:text-3xl hover:text-gray-600 text-gray-400 w-8 h-8 transition-all">
                    {showDetail ?
                        <i className="fa-solid fa-caret-up"/>
                        :
                        <i className="fa-solid fa-caret-down"/>
                    }
                </button>
            </td>
            <td>
                <button type="button" onClick={e => {
                    deleteSubmit()
                }} disabled={deleteSubmitted}
                        className="align-text-bottom text-3xl hover:text-4xl hover:text-red-700 text-red-600 w-8 h-8 transition-all">
                    <i className="fa-solid fa-remove"/>
                </button>
            </td>
            <td>
                <div className="dropdown dropdown-end">
                    <label tabIndex={0} className="btn btn-ghost btn-square m-1">
                        <i className="fa-solid fa-ellipsis-vertical"/>
                    </label>
                    <ul tabIndex={0} className="dropdown-content menu p-2 shadow-lg bg-base-200 rounded-btn w-52">
                        <li>
                            <Link href={paths.client(client.record.join())}>
                                <a>View File</a>
                            </Link>
                        </li>
                        <li>
                            <Link href={paths.clientRelations(client.record.join())}>
                                <a>Relations</a>
                            </Link>
                        </li>
                    </ul>
                </div>
            </td>
        </tr>

        {showDetail ?
            <tr>
                <td colSpan={100}>
                    <div className="bg-base-300 font-normal rounded-box p-2 w-[42rem] ">
                        <p className="font-bold text-xs text-gray-400">
                            FURTHER INFORMATION:
                        </p>
                        <div className="flex flex-col mb-1 w-fit">
                            <p>
                                <i className="fa-solid fa-paper-plane mr-1"/>
                                <a className="font-normal text-base min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate underline decoration-blue-700 text-blue-700 hover:no-underline hover:text-blue-600"
                                   href={"mailto:" + client.email}>
                                    {client.email}
                                </a>
                            </p>
                        </div>
                        <div className="flex flex-col mb-1">
                            <p className="text-base min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                                <i className="fa-solid fa-phone mr-1 text-[0.9rem]"/>
                                {client.mobileNumber.at(0) == '+' ?
                                    <>
                                        {client.mobileNumber.substring(0, 3) + " " + client.mobileNumber.substring(3, 6) + " " + client.mobileNumber.substring(6, 13)}
                                    </>
                                    :
                                    <>

                                        {client.mobileNumber.substring(0, 4) + " " + client.mobileNumber.substring(4, client.mobileNumber.length)}
                                    </>
                                }
                            </p>
                        </div>
                        <div className="flex flex-col w-fit mb-4">
                            {client.consent ?
                                <div className="badge badge-success gap-1 mt-1">
                                    <i className="fa-solid fa-check"/>
                                    Consent given
                                </div>
                                :
                                <div className="badge badge-error gap-1 mt-1">
                                    <i className="fa-solid fa-x"/>
                                    No consent
                                </div>
                            }
                        </div>
                        <p className="text-xs text-gray-400 mb-1">
                            REGISTERED ANIMALS:
                        </p>
                        {deleteSubmitted ?
                            <></>
                            :
                            <AnimalList client={client}/>
                        }
                    </div>
                </td>
            </tr>
            :
            <></>
        }
    </>
})