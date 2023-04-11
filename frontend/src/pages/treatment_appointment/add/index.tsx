import Head from 'next/head'
import ClientRegisterForm from "../../../components/client/ClientRegisterForm";
import {NextLayoutPage} from "../../../types/layout";
import ClientLayout from "../_layout";
import TreatmentAppointmentADD from "../../../components/treatment_appointment/TreatmentAppointmentADD";

const RegisterClient: NextLayoutPage = () => {
    return (
        <div className="flex flex-col w-11/12 m-5 p-3 rounded-lg bg-gray-200 shadow">
            <Head>
                <title>YAMS - New Client</title>
            </Head>

            <main className="">
                <TreatmentAppointmentADD onFinish={(e: any) => {
                }}/>
            </main>
        </div>
    )
}


export default RegisterClient