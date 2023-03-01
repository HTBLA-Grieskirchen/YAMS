import Head from "next/head";
import {NextLayoutPage} from "../../types/layout";
import TreatmentAppointmentLayout from "./_layout";
import {useStore} from "../../stores";
import TreatmentAppointmentTableHeader from "../../components/treatmentAppointment/TreatmentAppointmentTableHeader";
import TreatmentAppointmentAddItem from "../../components/treatmentAppointment/TreatmentAppointmentAddItem";
import TreatmentAppointmentListEntry from "../../components/treatmentAppointment/TreatmentAppointmentListEntry";
import {observer} from "mobx-react";

const TreatmentAppointments: NextLayoutPage = observer(() => {
    const store = useStore()
    const treatmentAppointments = store.treatmentAppointmentStore.treatmentAppointments

    return <>
        <Head>
            <title>YAMS - TreatmentAppointment</title>
        </Head>

        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <TreatmentAppointmentTableHeader/>
                <div className="table-row-group w-full bg-white divide-y-2 ">
                    {<TreatmentAppointmentAddItem/>}
                    {treatmentAppointments.length > 0 ?
                        treatmentAppointments.map((treatmentAppointment) =>
                            <TreatmentAppointmentListEntry key={treatmentAppointment.record.join()} treatmentAppointment={treatmentAppointment}/>) :
                        <tr>
                            <td colSpan={10} className="py-1">
                                <div className="w-full flex">
                                    <p className="mx-auto text-gray-600 whitespace-nowrap font-normal">Start to add
                                        treatmentAppointments</p>
                                </div>
                            </td>
                        </tr>}
                </div>
            </div>
        </main>
    </>
})

TreatmentAppointments.Layout = TreatmentAppointmentLayout

export default TreatmentAppointments