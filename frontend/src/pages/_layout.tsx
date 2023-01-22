import React, {ReactElement} from "react";
import Head from "next/head";
import {observer} from "mobx-react";
import Notifications from "../components/Notifications";
import Modals from "../components/Modals";
import MainMenu from "../components/layout/MainMenu";

const Layout = observer(({children}: { children: ReactElement }) => {
    return <>
        <Head>
            <title>YAMS</title>
            <meta name="description"
                  content="Yet Another Management System - Management for Clients, Animals and Bills"/>

            <link rel="shortcut icon" href="/favicon.ico"/>
            <link rel="icon" href="/favicon.ico"/>
        </Head>

        <MainMenu>
            {children}
        </MainMenu>

        <Modals/>
        <Notifications/>
    </>
})

export default Layout
