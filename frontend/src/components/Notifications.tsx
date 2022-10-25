import {observer} from "mobx-react";
import NotificationComponent from "./NotificationComponent";
import {useStore} from "../stores";
import {useEffect, useRef} from "react";

const Notifications = observer(() => {
    const store = useStore()

    const notifications = useRef<HTMLDivElement>(null)

    // Scroll down to the newest notification if added
    useEffect(() => {
        if (notifications.current !== null) {
            notifications.current.scrollTop = notifications.current.scrollHeight - notifications.current.clientHeight
        }
    }, [notifications.current?.scrollHeight])

    return <div ref={notifications}
                className="fixed bottom-0 right-2 space-y-2 pl-16 pt-8 pb-4 pr-2 max-w-full max-h-full overflow-y-auto overscroll-auto hover:overscroll-contain">
        {store.notificationStore.currentNavigations().map((notification, index) =>
            <NotificationComponent key={index} notification={notification}/>)}
    </div>
})

export default Notifications
