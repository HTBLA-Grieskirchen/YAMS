import {observer} from "mobx-react";
import Client from "../../model/client"
import {LiveRefresher, query} from "../../libs/database";
import React, {useState} from "react";
import AnimalList from "../animal/AnimalList";
import {Result} from "surrealdb.js";

const ClientItem = observer(({client, refresher}: { client: Client, refresher: LiveRefresher }) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [showDetail, setShowDetail] = useState(false)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)

        const result = await deleteClient(client)

        await refresher()
        setDeleteSubmitted(false)
    }

    return (
        <div className="flex flex-col w-fit">
            <div className="flex flex-row space-x-4 items-center">
                <div className="flex flex-row space-x-2">
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Last Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.last_name}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            First Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.first_name}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Birth Date
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.birthdate.toLocaleDateString()}
                        </p>
                    </div>

                </div>
                <div className="flex flex-row h-full items-center space-x-1">
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
                    <button type="button" onClick={e => {
                        deleteSubmit()
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-red-700 text-red-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-remove"/>
                    </button>
                </div>
            </div>
            {showDetail ?
                <div className="bg-white rounded-xl p-2 w-fit mt-2">
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            E-Mail
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.email}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Consent
                        </label>
                        {client.consent ?
                            <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                                Present
                            </p>
                            :
                            <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                                Not Present
                            </p>
                        }
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Mobile Number
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.mobile_number}
                        </p>
                    </div>
                    {deleteSubmitted ?
                        <></>
                        :
                        <AnimalList client={client}/>
                    }
                </div>
                :
                <div></div>
            }
        </div>
    )
})

export async function deleteClient(client: Client): Promise<Result<any>> {
    const response = await query("DELETE type::thing($landTable, $landID)", {
        landTable: Client.TABLE_NAME,
        landID: client.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export default ClientItem