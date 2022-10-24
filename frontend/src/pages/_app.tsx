import '../styles/globals.css'
import "@fortawesome/fontawesome-free/css/all.css"
import type {AppProps} from 'next/app'
import {StoreProvider} from "../stores";
import Layout from "./_layout"
import {NextLayoutPage} from "../types/layout";
import {ReactElement} from "react";

type AppLayoutProps = AppProps & {
    Component: NextLayoutPage
}

export default function MyApp({Component, pageProps}: AppLayoutProps) {
    const PageLayout = Component.Layout ?? (({children}: { children: ReactElement }) => children)

    return <StoreProvider>
        <Layout>
            <PageLayout>
                <Component {...pageProps} />
            </PageLayout>
        </Layout>
    </StoreProvider>
}
