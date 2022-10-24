import {ReactElement, useEffect, useRef} from "react";
import Head from "next/head";
import {useStore} from "../stores";
import NotificationComponent from "../components/NotificationComponent";
import {observer} from "mobx-react";

const Layout = observer(({children}: { children: ReactElement }) => {
    const store = useStore()
    const notifications = useRef<HTMLDivElement>(null)

    // Scroll down to the newest notification if added
    useEffect(() => {
        if (notifications.current !== null) {
            notifications.current.scrollTop = notifications.current.scrollHeight - notifications.current.clientHeight
        }
    }, [notifications.current?.scrollHeight])


    return <>
        <Head>
            <title>YAMS</title>
            <meta name="description"
                  content="Yet Another Management System - Management for Clients, Animals and Bills"/>

            <link rel="shortcut icon" href="/favicon.ico"/>
            <link rel="icon" href="/favicon.ico"/>
        </Head>
        {children}
        {/*Notifications*/}
        <div ref={notifications}
             className="fixed bottom-4 right-4 space-y-2 pl-16 pt-8 max-w-full max-h-full overflow-y-auto overscroll-auto hover:overscroll-contain">
            {store.notificationStore.currentNavigations().map((notification, index) =>
                <NotificationComponent key={index} notification={notification}/>)}
        </div>
    </>
})

export default Layout
