import {observer} from "mobx-react";
import {useStore} from "../stores";
import {CSSProperties, useEffect, useState} from "react";
import {NotificationBehaviour, NotificationInfo, NotificationType} from "../libs/notification";
import {isPromise} from "../util/types";
import {autorun} from "mobx";
import { Card, CardBody, Button, CircularProgress } from "@heroui/react";
import { Bell, Info, CheckCircle2, AlertTriangle, XCircle, X } from "lucide-react";

const Notifications = observer(() => {
    const store = useStore()

    return <div className="fixed bottom-4 right-4 z-[100] flex flex-col gap-2 max-w-md w-full">
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
    const {Icon, color} = notificationTypeValues(notification)

    return (
        <Card shadow="lg" className="border-none bg-background/80 backdrop-blur-md">
            <CardBody className="p-3">
                <div className="flex gap-3">
                    <div className={`p-2 rounded-full bg-${color}/20 text-${color}`}>
                        <Icon size={20} />
                    </div>
                    <div className="flex-1 min-w-0">
                        {notification.title && <h4 className="text-sm font-bold truncate">{notification.title}</h4>}
                        <p className="text-xs text-foreground-500 line-clamp-2">{notification.message}</p>
                        
                        {notification.actions && (
                            <div className="flex flex-wrap gap-2 mt-2">
                                {Object.entries(notification.actions).map(([label, action]) => (
                                    <ActionButton key={label} notification={notification} label={label} behaviour={action}/>
                                ))}
                            </div>
                        )}
                    </div>
                    <div className="flex flex-col items-center gap-1">
                        <Button
                            isIconOnly
                            variant="light"
                            size="sm"
                            onClick={() => store.notificationStore.removeNotification(notification)}
                        >
                            {notification.duration ? (
                                <CircularProgress
                                    aria-label="Notification timeout"
                                    size="sm"
                                    value={100 - 100 * notification.msPassed / (notification.duration ?? Infinity)}
                                    color={color as any}
                                    showValueLabel={false}
                                    strokeWidth={4}
                                    classNames={{
                                        svg: "w-6 h-6",
                                    }}
                                >
                                    <X size={14} />
                                </CircularProgress>
                            ) : (
                                <X size={18} />
                            )}
                        </Button>
                    </div>
                </div>
            </CardBody>
        </Card>
    )
})

const ActionButton = observer((
    {notification, behaviour, label}:
        { notification: NotificationInfo, behaviour: NotificationBehaviour, label: string }
) => {
    const store = useStore()
    const [clicked, setClicked] = useState(false)
    const [disabled, setDisabled] = useState(false)

    const color = buttonColor(notification, behaviour)

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
    }, [behaviour])

    return (
        <Button 
            size="sm" 
            variant={behaviour.type === "ghost" ? "light" : "flat"} 
            color={color as any}
            isLoading={clicked}
            isDisabled={disabled}
            onClick={handleClick}
        >
            {label}
        </Button>
    )
})

function notificationTypeValues(notification: NotificationInfo) {
    switch (notification.type) {
        case NotificationType.Neutral:
            return { Icon: Bell, color: "default" }
        case NotificationType.Info:
            return { Icon: Info, color: "primary" }
        case NotificationType.Success:
            return { Icon: CheckCircle2, color: "success" }
        case NotificationType.Warn:
            return { Icon: AlertTriangle, color: "warning" }
        case NotificationType.Error:
            return { Icon: XCircle, color: "danger" }
        default:
            return { Icon: Bell, color: "default" }
    }
}

function buttonColor(notification: NotificationInfo, behaviour: NotificationBehaviour) {
    if (behaviour.type === "ghost") return "default"
    if (behaviour.type === "neutral") return "default"

    switch (notification.type) {
        case NotificationType.Neutral: return "primary"
        case NotificationType.Info: return "primary"
        case NotificationType.Success: return "success"
        case NotificationType.Warn: return "warning"
        case NotificationType.Error: return "danger"
        default: return "default"
    }
}

export default Notifications
