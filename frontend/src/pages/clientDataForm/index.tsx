import type {NextPage} from 'next'
import Head from 'next/head'
import {StoreProvider} from "../../stores";
import FormPage from "./FormPage";

const Home: NextPage = () => {
    return (
        <div className="">
            <Head>
                <title>YAMS</title>
                <meta name="description"
                      content="Yet Another Management System - Management for Clients, Animals and Bills"/>

                <link rel="shortcut icon" href="/favicon.ico"/>
                <link rel="icon" href="/favicon.ico"/>
            </Head>

            <main className="">
                <StoreProvider>
                    <FormPage/>
                </StoreProvider>
            </main>
        </div>
    )
}

export default Home