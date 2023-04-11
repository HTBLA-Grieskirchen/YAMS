import React, {useEffect, useState} from "react"
import {query} from "../../libs/database";
import Select from "react-select";
import {observer} from "mobx-react";
import {Result} from "surrealdb.js";
import Address from "../../model/address";
import {useRouter} from "next/router";
import paths from "../../util/paths";
import {useStore} from "../../stores";
import {beforeEach} from "vitest";
import Treatment from "../../model/treatment";

const AddTreatmentAppointmentForm = observer(({onFinish}: { onFinish: (result: Result<any> | null) => void }) => {
    const router = useRouter()
    const store = useStore()
    const list:string[]=["test"]



    const [anmerkung, setAnmerkung] = useState('')
    const [kosten, setKosten] = useState(0)
    const [date, setDate] = useState('2000-01-01')
    const [treatment, setTreatment] = useState<Treatment | null>(null)


    const options=store.treatmentStore.treatments
    var test=[{value:options[0],label:"test"}]
    useEffect(()=>{options.forEach(function (value){
        test.push({value:value,label:value.description}); console.log(value)
    })})
    useEffect(()=>{console.log(router.query.clientFileID)})

    const handleADDPress=async () =>{
        var response = await query("Insert into treatment_appointment (cost,date,extra,treatment) Values ($cost,$date,$extra,type::thing($table,$id))", {
            table:"treatment",
            //hier id ändern
            id:treatment?.record.id,
            date:date,
            cost:kosten,
            extra:anmerkung
        });




        query("Update type::thing('client_file',$clientFILEID) Set treatment += [type::thing('treatment_appointment',$treatmentAID)]",
            {treatmentAID:response[0].result[0].id,
                clientFILEID:router.query.clientFileID});

        console.log('hallo')}


    //TODO: improve validation | onSubmit deletes all inputs
    return (
        <div>
            <div>
                <div>

                    <label>
                        <span className="text-gray-700">Anmerkung</span>
                    </label>
                    <input
                        type="text"
                        className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                        placeholder="Anmerkung"
                        onChange={e => setAnmerkung(e.target.value)}
                    />
                </div>
                <br></br>

                        <div>
                            <label>
                                <span className="text-gray-700">Kosten</span>
                            </label>
                            <input
                        type="text"
                        className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                        placeholder="Kosten"
                        onChange={e => setKosten(parseFloat(e.target.value))}
                            />
                        </div>
                <br></br>

                <div>
                    <label>
                        <span className="text-gray-700">Datum</span>
                    </label>
                    <input
                        type="date"
                        className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                        onChange={e => setDate(e.target.value)}

                    />
                </div>
                <br></br>

                <div
                    className="text-lg p-1 rounded-lg border-2 border-transparent outline-none focus:border-blue-600 bg-white">
                    <label htmlFor="cboAddress" className="text-gray-700">
                        Behandlung auswählen
                    </label>
                    <Select className="w-96 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                            classNamePrefix="select"
                            options={test}
                            name="treatments"
                            onChange={e => e!=null&&setTreatment(e.value)}/>

                </div>
                <br></br>
                <button onClick={event =>handleADDPress()}
                >Speichern</button>
                <br></br>
                <button onClick={e=> {router.push(
                { pathname: paths.client_file, query: { name: "Someone" } },
                "client_file"
            );console.log(router.query.clientFileID)}}>Zurück</button>
                    </div>




                </div>





    )
})

export default AddTreatmentAppointmentForm