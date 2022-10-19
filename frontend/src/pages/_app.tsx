import '../styles/globals.css'
import type {AppProps} from 'next/app'
import {StoreProvider} from "../stores";
import {Layout} from "./_layout"

export default function MyApp({Component, pageProps}: AppProps) {
    return <StoreProvider>
        <Layout>
            <Component {...pageProps} />
        </Layout>
    </StoreProvider>
}
