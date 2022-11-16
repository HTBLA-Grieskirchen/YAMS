import type {NextPage} from 'next'
import Head from 'next/head'
import ClientRegisterEditingForm from "../../../components/ClientRegisterEditingForm";

const Home: NextPage = () => {
    return (
        <div className="">
            <Head>
                <title>YAMS - Edit Client</title>
            </Head>

            <main className="">
                <ClientRegisterEditingForm/>
            </main>
        </div>
    )
}
export default Home

