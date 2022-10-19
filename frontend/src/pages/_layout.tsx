import {ReactNode} from "react";
import Head from "next/head";

const Layout = ({children}: { children: ReactNode }) => {
    return <>
        <Head>
            <title>YAMS</title>
            <meta name="description"
                  content="Yet Another Management System - Management for Clients, Animals and Bills"/>

            <link rel="shortcut icon" href="/favicon.ico"/>
            <link rel="icon" href="/favicon.ico"/>
        </Head>
        {children}
    </>
}

export {Layout}
