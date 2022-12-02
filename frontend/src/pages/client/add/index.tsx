import Head from 'next/head'
import ClientRegisterForm from "../../../components/client/ClientRegisterForm";
import {NextLayoutPage} from "../../../types/layout";
import ClientLayout from "../_layout";

const RegisterClient: NextLayoutPage = () => {
    return (
        <div className="">
            <Head>
                <title>YAMS - New Client</title>
            </Head>

            <main className="">
                <ClientRegisterForm onFinish={(e: any) => {
                }}/>
            </main>
        </div>
    )
}

RegisterClient.Layout = ClientLayout
export default RegisterClient