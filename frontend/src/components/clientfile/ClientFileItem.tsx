import {observer} from "mobx-react";
import Client from "../../model/client"
import {LiveRefresher, query} from "../../libs/database";
import React, {useState} from "react";
import {Result} from "surrealdb.js";
import AnimalList from "../animal/AnimalList";
import {useRouter} from "next/router";
import {useEffect} from "react";
import paths from "../../util/paths";
import {useStore} from "../../stores";
import Treatment_Appointment from "../../model/treatment_appointment";

const ClientFileItem = observer(({treatment_appointment, refresher}: { treatment_appointment: Treatment_Appointment, refresher: LiveRefresher }) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [showDetail, setShowDetail] = useState(false)
    const store=useStore()
    const router = useRouter();
    var selectedClient=null

    useEffect(() => {
        console.log(router.query.client_id); // Alerts ClientID
    }, [router.query]);

    const clients=store.clientStore.clients

    useEffect(()=>{clients.forEach(function (value){
        if(value.record.id==router.query.client_id){selectedClient=value}
    })})


    useEffect(() => {
        console.log(router.query.client_id); // Alerts ClientID
    }, [router.query]);

    /*const deleteSubmit = async () => {
        setDeleteSubmitted(true)

        const result = await delete(treatment_appointment)

        await refresher()
        setDeleteSubmitted(false)
    }*/



    return (
        <div className="flex flex-col w-fit">
            <div className="flex flex-row space-x-4 items-center">
                <div className="flex flex-row space-x-2">

                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Datum
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {treatment_appointment.date.toString()}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Behandlugsart
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {treatment_appointment.treatment.type.description}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Anmerkung
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {treatment_appointment.extra}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Dauer
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {treatment_appointment.treatment.duration.toString()}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Kosten
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {treatment_appointment.cost.toString()}
                        </p>
                    </div>

                </div>

            </div>


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

export default ClientFileItem