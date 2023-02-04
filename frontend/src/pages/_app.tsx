import '../styles/globals.css'
import "@fortawesome/fontawesome-free/css/all.css"
import type { AppProps } from 'next/app'
import { StoreProvider } from "../stores"
import { LayoutPage, NavigationPage } from "../types/layout";
import { observer } from "mobx-react";
import React, { ReactElement } from "react";
import Head from "next/head";
import MainMenu from "../components/layout/MainMenu";
import MainNavbar from "../components/layout/MainNavbar";
import Modals from "../components/Modals";
import Notifications from "../components/Notifications";
import paths from "../util/paths";

export type AppLayoutProps = AppProps & {
    Component: LayoutPage & NavigationPage
}

export default function MyApp({Component, pageProps}: AppLayoutProps) {

    return <StoreProvider>
        <Layout Page={Component}>
            <Component {...pageProps} />
        </Layout>
    </StoreProvider>
}

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

