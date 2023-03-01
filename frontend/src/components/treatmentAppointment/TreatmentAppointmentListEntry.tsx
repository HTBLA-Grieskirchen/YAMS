import {observer} from "mobx-react";
import TreatmentAppointmentDetailItem from "./TreatmentAppointmentDetailItem";
import {useState} from "react";
import TreatmentAppointmentEditItem from "./TreatmentAppointmentEditItem";
import TreatmentAppointmentUsageTabs from "./TreatmentAppointmentUsageTabs";
import TreatmentAppointment from "../../model/treatmentAppointment";

const TreatmentAppointmentListEntry = observer((
    {treatmentAppointment}:
        { treatmentAppointment: TreatmentAppointment }
) => {
    const [editing, setEditing] = useState(false)

    return <>
        {editing ?
            <TreatmentAppointmentEditItem treatmentAppointment={treatmentAppointment} onCancel={() => setEditing(false)} onConfirm={(s) => {
                if (s) setEditing(false)
            }} noBottomPadding={true}/> :
            <TreatmentAppointmentDetailItem treatmentAppointment={treatmentAppointment} onEdit={() => setEditing(true)} noBottomPadding={true}/>
        }
        <div className="table-row">
            <td colSpan={10} className="px-4 pt-1 pb-2">
                <TreatmentAppointmentUsageTabs treatmentAppointment={treatmentAppointment}/>
            </td>
        </div>
    </>
})

export default TreatmentAppointmentListEntry
