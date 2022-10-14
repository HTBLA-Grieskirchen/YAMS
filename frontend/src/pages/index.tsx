import type {NextPage} from 'next'
import Head from 'next/head'
import styles from '../styles/Home.module.css'
import MyApp from "./_app";
import {StoreProvider} from "../stores";

const Home: NextPage = () => {
  return (
      <div className={styles.container}>
          <Head>
              <title>YAMS</title>
              <meta name="description"
                    content="Yet Another Management System - Management for Clients, Animals and Bills"/>

              <link rel="shortcut icon" href="/favicon.ico"/>
              <link rel="icon" href="/favicon.ico"/>
          </Head>

          <main className={styles.main}>
              <StoreProvider>
                  <MyApp/>
              </StoreProvider>
          </main>
      </div>
  )
}

export default Home
