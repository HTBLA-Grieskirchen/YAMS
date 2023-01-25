import React, {ReactElement} from "react";
import Head from "next/head";
import {observer} from "mobx-react";
import Notifications from "../components/Notifications";
import Modals from "../components/Modals";
import MainMenu from "../components/layout/MainMenu";
import paths from "../util/paths";
import {AppLayoutProps} from "./_app";
import MainNavbar from "../components/layout/MainNavbar";

const Layout = observer((
    {children, Page}:
        { children: ReactElement, Page: AppLayoutProps["Component"] }
) => {
    const PageLayout = Page.Layout ?? (({children}: { children: ReactElement }) => children)
    const NavbarPath = Page.NavPath ?? (() => <></>)
    const NavbarMenu = Page.NavMenu ?? (() => <></>)

    return <>
        <Head>
            <title>YAMS</title>
            <meta name="description"
                  content="Yet Another Management System - Management for Clients, Animals and Bills"/>

            <link rel="shortcut icon" href="/favicon.ico"/>
            <link rel="icon" href="/favicon.ico"/>
        </Head>

        <MainMenu entries={itemsMainMenu}>
            <MainNavbar NavbarPath={NavbarPath} NavbarMenu={NavbarMenu}>
                <PageLayout>
                    {children}
                </PageLayout>
            </MainNavbar>
        </MainMenu>

        <Modals/>
        <Notifications/>
    </>
})

const itemsMainMenu = {
    "Management": {
        "Client": {
            href: paths.clients
        },
        "Address": {
            href: paths.addresses
        }
    }
}

export default Layout
