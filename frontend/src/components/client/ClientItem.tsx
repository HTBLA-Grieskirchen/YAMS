import {observer} from "mobx-react";
import Client from "../../model/client"
import {LiveRefresher, query, useLive} from "../../libs/database";
import React, {useEffect, useState} from "react";
import {Result} from "surrealdb.js";
import AnimalList from "../animal/AnimalList";
import paths from "../../util/paths";
import {useRouter} from "next/router";
import client from "../../pages/client";
import Client_file from "../../pages/client_file";
import ClientFile from "../../model/clientfile";

const ClientItem = observer(({client, refresher}: { client: Client, refresher: LiveRefresher }) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [showDetail, setShowDetail] = useState(false)
    const router = useRouter()
    const dateOfBirth = new Date(1990, 4, 7)
    const date: Date = new Date(2017, 4, 4, 17, 23, 42, 11);
    const [entries, refreshEntries] = useLive("SELECT content FROM entry", undefined)
    const entriesExtracted = entries.response !== undefined && entries.response.length > 0 ? entries.response[0].result : []



    const client_files:ClientFile[]=[]

    const cfileDate=new Date()
    const behandlungsArt='test'
    const anmerkung=''
    const dauer=1
    const kosten=1

    //hier weiter machen
   // const [clientFileRaw] = useLive("SELECT treatment.*.* FROM type::thing($table, $id)", {
    //    table: "client_file",
   //     id: client.record.id
   // })client,



    //useEffect(() => {
        //query("Select * From treatment").then((result) => result.forEach((item) => console.log(item)))})

    //Ich will eine Liste aller treatments von einem bestimmten clienten


    const deleteSubmit = async () => {
        setDeleteSubmitted(true)

        const result = await deleteClient(client)

        await refresher()
        setDeleteSubmitted(false)
    }

    return (
        <div className="flex flex-col w-fit">

            {entries.loading && <p>Data is currently loading</p>}
            <div className="flex flex-row space-x-4 items-center">
                <div className="flex flex-row space-x-2">
                    <div className="flex flex-col place-content-center mr-2">
                        {client.consent ?
                            <i className="fa-solid fa-circle-check text-green-600"/>
                            :
                            <i className="fa-solid fa-circle-exclamation text-red-600"/>}
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Last Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.lastName}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            First Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.firstName}
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
                    <button type="button" onClick={e => {
                        //router.push(paths.client_file);
                        router.push(
                            { pathname: paths.client_file, query: { client_id: client.record.id } },
                            "client_file"
                        );
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-black-700 text-black-600 w-8 h-8 transition-all">
                        <i className=" fa-solid fa-address-card"></i>
                    </button>
                </div>
            </div>
            {showDetail ?
                <div className="bg-white rounded-xl p-2 w-[42rem] mt-2">
                    <p className="text-xs text-gray-400">
                        FURTHER INFORMATION:
                    </p>
                    <div className="flex flex-col mb-1 w-fit">
                        <a className="font-medium text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate underline decoration-blue-700 text-blue-700 hover:no-underline hover:text-blue-600"
                           href={"mailto:" + client.email}>
                            {client.email}
                        </a>
                    </div>
                    <div className="flex flex-col mb-1">
                        <p className="text-base text-gray-600 min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
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
                            <p className="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-green-600 bg-green-200 uppercase last:mr-0 mr-1">
                                <i className="fa-regular fa-circle-check mr-2"/>
                                Consent given
                            </p>
                            :
                            <p className="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-red-600 bg-red-200 uppercase last:mr-0 mr-1">
                                <i className="fa-regular fa-circle-xmark mr-2"/>
                                Consent not given
                            </p>
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
                :
                <div></div>
            }

        </div>

    )
})

export async function deleteClient(client: Client): Promise<Result<any>> {
    const response = await query("DELETE type::thing($clientTable, $clientID)", {
        clientTable: Client.TABLE,
        clientID: client.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export default ClientItem