import {observer} from "mobx-react";
import {NotificationInfo, NotificationType} from "../libs/notification";
import {useStore} from "../stores";
import {useState} from "react";
import {isPromise} from "../util/types";

const NotificationComponent = observer(({notification}: { notification: NotificationInfo }) => {
    const store = useStore()

    const percent = notification.remaining * 100 / notification.duration
    const circumference = 12 * 2 * Math.PI

    const colors = getNotificationColors(notification.type)

    const ActionButton = (
        {label, action}: { label: string, action: () => Promise<boolean> | boolean }
    ) => {
        const [clicked, setClicked] = useState(false)
        const handleClick = async () => {
            const result = action()
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

        return <button onClick={e => {
            handleClick()
        }} disabled={clicked}
                       className={"inline-block rounded-md px-1 py-0 text-sm shadow-md " +
                           "hover:shadow-lg transition " +
                           "disabled:cursor-not-allowed disabled:opacity-75 "
                           + " " + " " + colors.buttonBackground
                           + " " + colors.buttonText
                           + " enabled:" + colors.buttonHover}>
            {label}
        </button>
    }

    return <div
        className={"relative rounded-lg border border-4 w-full h-fit max-h-20 " + colors.background + " " + colors.border}>
        {notification.title && <p className={"static truncate text-lg pr-7 p-1 pt-0 " + colors.title}>
            {notification.title}
        </p>}
        <button onClick={e => store.notificationStore.removeNotification(notification)}
                className="absolute inline-flex items-center justify-center overflow-hidden rounded-full top-0 right-0 text-sm text-gray-600 hover:text-gray-800 transition-all">
            <svg className="w-6 h-6 items-center justify-center" viewBox="0 0 36 36">
                <circle
                    className={colors.circleBackground}
                    strokeWidth={3}
                    stroke="currentColor"
                    fill="transparent"
                    r="12"
                    cx="18"
                    cy="18"
                />
                <circle
                    className={colors.circleForeground}
                    strokeWidth={3}
                    strokeDasharray={circumference}
                    strokeDashoffset={circumference - percent / 100 * circumference}
                    strokeLinecap="round"
                    stroke="currentColor"
                    fill="transparent"
                    r="12"
                    cx="18"
                    cy="18"
                />
            </svg>
            <div className="absolute align-text-bottom">
                <i className="fa-solid fa-xmark"/>
            </div>
        </button>
        <div
            className={"flex flex-row m-1 items-center place-content-between" + (notification.title ? " mt-0" : " mr-8")}>
            <p className={"leading-none line-clamp-2 break-words pr-4 " + colors.message}>
                {notification.message}
            </p>
            {notification.actions &&
                <div className="flex flex-row space-x-2">
                    {Object.entries(notification.actions).map(([label, action], index) =>
                        ActionButton({label: label, action: action}))}
                </div>
            }
        </div>
    </div>
})

type NotificationColors = {
    buttonHover: string;
    buttonBackground: string,
    buttonText: string,
    background: string,
    border: string,
    title: string
    message: string,
    circleBackground: string,
    circleForeground: string
}

function getNotificationColors(type: NotificationType): NotificationColors {
    switch (type) {
        case NotificationType.Info:
            return {
                buttonHover: "hover:bg-gray-700",
                background: "bg-gray-100",
                border: "border-gray-400",
                buttonBackground: "bg-gray-500",
                buttonText: "text-white",
                circleBackground: "text-gray-300",
                circleForeground: "text-gray-500",
                message: "text-gray-800",
                title: "text-gray-900"
            }
        case NotificationType.Success:
            return {
                buttonHover: "hover:bg-green-700",
                background: "bg-green-200",
                border: "border-green-400",
                buttonBackground: "bg-green-500",
                buttonText: "text-white",
                circleBackground: "text-green-300",
                circleForeground: "text-green-500",
                message: "text-green-800",
                title: "text-green-900"

            }
        case NotificationType.Warn:
            return {
                buttonHover: "hover:bg-yellow-700",
                background: "bg-yellow-200",
                border: "border-yellow-400",
                buttonBackground: "bg-yellow-500",
                buttonText: "text-black hover:text-white",
                circleBackground: "text-yellow-300",
                circleForeground: "text-yellow-500",
                message: "text-yellow-800",
                title: "text-yellow-900"

            }
        case NotificationType.Error:
            return {
                buttonHover: "hover:bg-red-700",
                background: "bg-red-200",
                border: "border-red-400",
                buttonBackground: "bg-red-500",
                buttonText: "text-white",
                circleBackground: "text-red-300",
                circleForeground: "text-red-500",
                message: "text-red-800",
                title: "text-red-900"

            }
        default:
            throw new Error("May never reach here")
    }
}

export default NotificationComponent
