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
                {/*<ClientRegisterEditingForm client={}/>*/}
            </main>
        </div>
    )
}
export default Home

