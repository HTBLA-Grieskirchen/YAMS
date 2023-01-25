import '../styles/globals.css'
import "@fortawesome/fontawesome-free/css/all.css"
import type {AppProps} from 'next/app'
import {StoreProvider} from "../stores";
import Layout from "./_layout"
import {LayoutPage, NavigationPage} from "../types/layout";

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
