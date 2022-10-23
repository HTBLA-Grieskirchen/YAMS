import store from "./index";
import {action, autorun, makeAutoObservable, observable} from "mobx";
import {NotificationInfo} from "../libs/notification";

export default class NotificationStore {
    private root: typeof store
    private notifications: NotificationEntry[]

    constructor(root: typeof store) {
        this.notifications = observable([])

        this.root = root
        makeAutoObservable(this, {
            removeNotification: action
        })
    }

    addNotification(notification: NotificationInfo) {
        const interval = setInterval(() => {
            notification.tick(25)
        }, 25)

        this.notifications.push({
            info: notification,
            interval: interval
        })

        autorun(() => {
            if (notification.remaining < 0) {
                this.removeNotification(notification)
            }
        })
    }

    removeNotification(notification: NotificationInfo) {
        this.notifications.splice(
            this.notifications.findIndex(
                (entry) => entry.info === notification),
            1).forEach((entry) => clearInterval(entry.interval))
    }

    currentNavigations(): NotificationInfo[] {
        return this.notifications.map((entry) => entry.info)
    }
}

type NotificationEntry = {
    info: NotificationInfo,
    interval: NodeJS.Timer
}
