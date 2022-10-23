import {makeAutoObservable} from "mobx";
import store from "../stores";

class NotificationInfo {
    type: NotificationType
    title?: string
    message: string
    actions?: NotificationActions
    remaining: number
    duration: number

    constructor(type: NotificationType, title: string | undefined, message: string, duration: number, actions?: NotificationActions) {
        this.type = type
        this.title = title
        this.message = message
        this.actions = actions
        this.remaining = duration
        this.duration = duration

        makeAutoObservable(this, {
            actions: false
        })
    }

    tick(elapsed: number) {
        this.remaining -= elapsed
    }
}

export enum NotificationType {
    Info,
    Success,
    Warn,
    Error
}

type NotificationActions = {
    [key: string]: () => boolean
}

type NotificationInfoType = NotificationInfo
export type {NotificationInfoType as NotificationInfo}

export type NotificationContent = {
    title?: string
    message: string
}

// TODO: Add possibility to also display notification on host system if in Tauri
const notification = {
    info(content: NotificationContent, duration: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Info, content.title, content.message, duration * 1000, actions)
        )
    },
    warn(content: NotificationContent, duration: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Warn, content.title, content.message, duration * 1000, actions)
        )
    },
    success(content: NotificationContent, duration: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Success, content.title, content.message, duration * 1000, actions)
        )
    },
    error(content: NotificationContent, duration: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Error, content.title, content.message, duration * 1000, actions)
        )
    }
}
export default notification