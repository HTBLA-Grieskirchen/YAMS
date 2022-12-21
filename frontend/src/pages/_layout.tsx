import {ReactElement} from "react";
import Head from "next/head";
import {observer} from "mobx-react";
import Notifications from "../components/Notifications";
import DialogOverlay from "../components/DialogOverlay";
import createLeftMenuLayout from "../components/layout/LeftMenuLayout";
import paths from "../util/paths";

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
        <DialogOverlay/>
        <Notifications/>
    </>
})

const MainMenu = createLeftMenuLayout({
    "Client": {
        href: paths.home, // TODO: Replace with client once available
        icon: "fa-person"
    },
    "Locations": {
        href: paths.addresses,
        icon: "fa-map-location-dot",
        recursive: true
    }
})

export default Layout
