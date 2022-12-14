import type {NextPage} from 'next'
import Head from 'next/head'
import ClientRegisterEditingForm from "../../../components/ClientRegisterEditingForm";
import Client from "../../../model/client";
import Address from "../../../model/address";

const Home: NextPage = () => {
    return (
        <div className="">
            <Head>
                <title>YAMS - Edit Client</title>
            </Head>

            <main className="">
                <ClientRegisterEditingForm client={new Client("Huber", "Fritz", new Date("01.11.2000"), "huber@gmail.com", "0660/123456", false, "01", new Address("Musterstrasse", "13", "4000", "Alkoven")  )}/>
            </main>
        </div>
    )
}
export default Home

