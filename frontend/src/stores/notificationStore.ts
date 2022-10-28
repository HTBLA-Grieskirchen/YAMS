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
        let disposeCheck: (() => void) | undefined = undefined
        if (notification.duration !== undefined) {
            interval = setInterval(() => {
                notification.tick(25)
            }, 25)

            disposeCheck = autorun(() => {
                if (notification.passed >= notification.duration!!) {
                    this.removeNotification(notification)
                }
            })
        }

        this.notifications.push({
            info: notification,
            clean: disposeCheck,
            interval
        })
    }

    removeNotification(notification: NotificationInfo) {
        this.notifications.splice(
            this.notifications.findIndex(
                (entry) => entry.info === notification),
            1).forEach((entry) => {
            if (entry.clean) {
                entry.clean()
            }
            clearInterval(entry.interval)
        })
    }

    currentNavigations(): NotificationInfo[] {
        return this.notifications.map((entry) => entry.info)
    }
}

type NotificationEntry = {
    info: NotificationInfo,
    clean: (() => void) | undefined,
    interval: NodeJS.Timer | undefined
}
