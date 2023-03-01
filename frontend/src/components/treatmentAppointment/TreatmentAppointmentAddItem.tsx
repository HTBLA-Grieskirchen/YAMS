import {observer} from "mobx-react";
import {useState} from "react";
import TreatmentAppointmentEditItem from "./TreatmentAppointmentEditItem";
import {Record} from "../../model/surreal";
import Treatment from "../../model/treatment";
import TreatmentAppointment from "../../model/treatmentAppointment";
import Treatment_type from "../../model/treatment_type";

const TreatmentAppointmentAddItem = observer(() => {
    const [adding, setAdding] = useState(false)

    return <>
        {adding ?
            <TreatmentAppointmentEditItem treatmentAppointment={
                new TreatmentAppointment(new Record(TreatmentAppointment.TABLE, "").join(),
                    new Treatment(
                        new Record(Treatment.TABLE, "").join(),
                        new Treatment_type(new Record(Treatment_type.TABLE, "").join(), ""),
                        "", 0, "", false),
                    0,
                    Date.prototype,
                    ""
                )
            } onCancel={() => setAdding(false)} onConfirm={(s) => {
                if (s) {
                    setAdding(false)
                }
            }}/> :
            <div className="table-row">
                <div className="table-cell py-2 px-4">
                    <button
                        className="flex w-6 h-6 bg-gray-300 shadow rounded hover:bg-gray-400/75 hover:shadow-lg transition"
                        onClick={e => setAdding(true)}>
                        <i className="fa-solid fa-add m-auto"/>
                    </button>
                </div>
            </div>
        }
    </>
})

export default TreatmentAppointmentAddItem