import {observer} from "mobx-react";
import Head from "next/head";
import ClientItem from "../../components/client/ClientItem";
import {NextLayoutPage} from "../../types/layout";
//import ClientLayout from "./_layout";
import {useStore} from "../../stores";
import ClientFileItem from "../../components/clientfile/ClientFileItem";
import client from "../client";
import React, {useEffect} from "react";
import {query} from "../../libs/database";
import {useRouter} from "next/router";
import {toJS} from "mobx";
import paths from "../../util/paths";

const ClientFileOverview: NextLayoutPage = observer(() => {
    const store = useStore()
    const clients = store.clientStore.clients
    const clientfiles=store.clientFileStore.clientFiles
    var selectedClientFile=clientfiles[0]
    const router = useRouter()
    useEffect(()=>{console.log(router.query.client_id)})

    clientfiles.forEach(function(client_file){if(client_file.client.record.id==router.query.client_id){selectedClientFile=client_file;console.log(selectedClientFile)};})


    useEffect(() => {
        store.addressStore.countries.forEach((country) => console.warn(country.record.id))
        query("CREATE city SET plz = '1234599', name='lustigSama', country = type::thing('country', 'country:iyweahfrlj3chnxirc5b')")
        query("SELECT * FROM city").then((result) => result.forEach((item) => console.log(item)))
    }, [])

    //useEffect(() => {
     //   query("Select * From client_file").then((result) => result.forEach((item) => console.log(item)))})

    const treatment_appointments=store.treatmentAppointmentStore.treatmentAppointments
    useEffect(() => {
        console.log(treatment_appointments[0])})

    const client_files=store.clientFileStore.clientFiles
    useEffect(() => { client_files[0]!=undefined&&
        console.log(client_files[0].treatment_appointments[0])})

    return (
        <>
            <Head>
                <title>YAMS - Clients</title>
            </Head>

            <main className="flex flex-col min-w-fit w-[52%] m-5 p-3 rounded-lg bg-gray-200 shadow">
                <div className="flex flex-col pt-3">
                    {/*Jetzt Klientenakte zusammensetzten (zuerst mit Testdaten), model klasse muss fertig gemacht werden, componente muss fertig gemacht werden*/}

                        <div className="divide-gray-400 divide-y">
                            {selectedClientFile!=undefined&&selectedClientFile.treatment_appointments.map((treatment_appointment) =>
                                <div key={treatment_appointment.record.join()} className="p-2">
                                    <ClientFileItem treatment_appointment={treatment_appointment} refresher={store.clientFileStore.refresh}/>
                                </div>
                            )}

                        </div>
                    <button onClick={e=> {router.push(
                        { pathname: paths.treatment_appointment, query: { clientFileID:selectedClientFile.record.id } },
                        "client_file"
                    )}}>Treatment_Appointment</button>
                </div>
            </main>

        </>
    )
})

//ClientFileOverview.Layout = ClientLayout

export default ClientFileOverview