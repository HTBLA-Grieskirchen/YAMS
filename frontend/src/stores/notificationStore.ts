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
        let interval: NodeJS.Timer | undefined = undefined
        if (notification.duration !== undefined) {
            interval = setInterval(() => {
                notification.tick(25)
            }, 25)

            autorun(() => {
                if (notification.passed >= notification.duration!!) {
                    this.removeNotification(notification)
                }
            })
        }

        this.notifications.push({
            info: notification,
            interval
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
    interval: NodeJS.Timer | undefined
}
