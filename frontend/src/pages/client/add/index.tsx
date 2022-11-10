import type {NextPage} from 'next'
import Head from 'next/head'
import ClientRegisterForm from "../../../components/ClientRegisterForm";

const Home: NextPage = () => {
    return (
        <div className="">
            <Head>
                <title>YAMS - New Client</title>
            </Head>

            <main className="">
                <ClientRegisterForm/>
            </main>
        </div>
    )
}

export default Home