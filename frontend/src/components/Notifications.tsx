import {observer} from "mobx-react";
import {useStore} from "../stores";
import {CSSProperties, useEffect, useState} from "react";
import {NotificationBehaviour, NotificationInfo, NotificationType} from "../libs/notification";
import {isPromise} from "../util/types";
import {autorun} from "mobx";

const Notifications = observer(() => {
    const store = useStore()

    return <div className="toast">
        {store.notificationStore.currentNotifications().map((notification) =>
            <Notification key={notification.uuid} notification={notification}/>)}
    </div>
})

const Notification = observer((
    {notification}: {
        notification: NotificationInfo
    }
) => {
    const store = useStore()
    const {icon, iconColor, alertColor} = notificationTypeValues(notification)

    return <div className={`alert ${alertColor} shadow-lg max-w-prose`}>
        <div>
            {typeof icon === "string" ? <i className={`${icon} ${iconColor} text-xl`}/> : <>{icon}</>}
            <div>
                {notification.title && <h3 className="font-bold">{notification.title}</h3>}
                <div className="text-xs">{notification.message}</div>
            </div>
        </div>
        <div className="flex-none flex-col place-content-between place-self-start">
            <button
                onClick={e => store.notificationStore.removeNotification(notification)}
                className={`btn btn-circle btn-ghost btn-xs self-end ${notification.duration ? "radial-progress" : ""}`}
                style={{
                    "--value": 100 - 100 * notification.msPassed / (notification.duration ?? Infinity),
                    "--size": "1.5rem",
                    "--thickness": "0.2rem"
                } as CSSProperties}>
                <i className="fa-solid fa-xmark text-lg"/>
            </button>
            <div className="flex flex-row gap-2">
                {notification.actions && Object.entries(notification.actions).map(([label, action]) => {
                    return <ActionButton key={label} notification={notification} label={label} behaviour={action}/>
                })}
            </div>
        </div>
    </div>
})

const ActionButton = observer((
    {notification, behaviour, label}:
        { notification: NotificationInfo, behaviour: NotificationBehaviour, label: string }
) => {
    const store = useStore()
    const [clicked, setClicked] = useState(false)
    const [disabled, setDisabled] = useState(false)

    const {btnColorType} = buttonTypeValues(notification, behaviour)

    const handleClick = async () => {
        const result = behaviour.action()
        if (isPromise(result)) {
            setClicked(true)
            const remove = await result
            setClicked(false)
            if (remove) {
                store.notificationStore.removeNotification(notification)
            }
        } else if (result) {
            store.notificationStore.removeNotification(notification)
        }
    }

    useEffect(() => {
        const dispose = autorun(() => {
            if (behaviour.disabled) {
                setDisabled(behaviour.disabled())
            } else {
                setDisabled(false)
            }
        })

        return () => dispose()
    }, [])

    return <button disabled={clicked || disabled} onClick={e => {
        handleClick().then()
    }} className={`btn btn-xs ${btnColorType} disabled:cursor-not-allowed`}>
        {label}
    </button>
})

function notificationTypeValues(notification: NotificationInfo) {
    switch (notification.type) {
        case NotificationType.Neutral:
            return {
                icon: "fa-regular fa-bell",
                iconColor: "text-info",
                alertColor: ""
            }
        case NotificationType.Info:
            return {
                icon: <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="2 2 20 20"
                           className="stroke-current flex-shrink-0 w-5 h-5">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                          d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>,
                iconColor: "",
                alertColor: "alert-info"
            }
        case NotificationType.Success:
            return {
                icon: "fa-regular fa-circle-check",
                iconColor: "",
                alertColor: "alert-success"
            }
        case NotificationType.Warn:
            return {
                icon: <svg xmlns="http://www.w3.org/2000/svg" className="stroke-current flex-shrink-0 h-5 w-5"
                           fill="none" viewBox="2 2 20 20">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                          d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                </svg>,
                iconColor: "",
                alertColor: "alert-warning"
            }
        case NotificationType.Error:
            return {
                icon: "fa-regular fa-circle-xmark",
                iconColor: "",
                alertColor: "alert-error"
            }
        default:
            throw new Error("May never reach here")
    }
}

function buttonTypeValues(notification: NotificationInfo, behaviour: NotificationBehaviour) {
    if (behaviour.type === "ghost") {
        return {
            btnColorType: "btn-ghost"
        }
    }

    if (behaviour.type === "neutral") {
        return {
            btnColorType: ""
        }
    }

    switch (notification.type) {
        case NotificationType.Neutral:
            return {
                btnColorType: "btn-primary"
            }
        case NotificationType.Info:
            return {
                btnColorType: ""
            }
        case NotificationType.Success:
            return {
                btnColorType: ""
            }
        case NotificationType.Warn:
            return {
                btnColorType: ""
            }
        case NotificationType.Error:
            return {
                btnColorType: ""
            }
        default:
            throw new Error("May never reach here")
    }
}

export default Notifications
