import type {NextPage} from 'next'
import Head from 'next/head'
import ClientRegisterForm from "../../../components/client/ClientRegisterForm";

const Home: NextPage = () => {
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

export default Home