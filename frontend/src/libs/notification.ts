import {makeAutoObservable} from "mobx";
import store from "../stores";

class NotificationInfo {
    type: NotificationType
    title?: string
    message: string
    actions?: NotificationActions
    passed: number
    readonly duration?: number

    constructor(type: NotificationType, title: string | undefined, message: string, duration?: number, actions?: NotificationActions) {
        this.type = type
        this.title = title
        this.message = message
        this.actions = actions
        this.passed = 0
        this.duration = duration

        makeAutoObservable(this, {
            actions: false
        })
    }

    tick(elapsed: number) {
        this.passed += elapsed
    }
}

export enum NotificationType {
    Info,
    Success,
    Warn,
    Error
}

type NotificationActions = {
    [key: string]: () => Promise<boolean> | boolean
}

type NotificationInfoType = NotificationInfo
export type {NotificationInfoType as NotificationInfo}

/**
 * Leave title empty if no title should be shown in the notification
 */
export type NotificationContent = {
    title?: string
    message: string
}

// TODO: Add possibility to also display notification on host system if in Tauri
const notification = {
    /**
     * This opens an info notification in the lower right corner of the screen. The content and actions can be specified
     * using the parameters down below. The notification can either be closed manually or automatically after the
     * specified amount of time.
     *
     *
     * @param content - The content to be displayed. Must contain a message, but may optionally also contain a title,
     * which will be displayed as title in the notification.
     * @param duration - The duration for which the notification is shown (in seconds) and after which is automatically
     * closed. May be undefined to indicate no automatic closure.
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `function`
     * (action) pairs. The action returns a boolean which indicates whether to close the notification after the
     * action.
     */
    info(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(
                NotificationType.Info,
                content.title,
                content.message,
                (duration ? duration * 1000 : duration),
                actions
            )
        )
    },
    /**
     * This opens a warning in the lower right corner of the screen. The content and actions can be specified
     * using the parameters down below. The notification can either be closed manually or automatically after the
     * specified amount of time.
     *
     *
     * @param content - The content to be displayed. Must contain a message, but may optionally also contain a title,
     * which will be displayed as title in the notification.
     * @param duration - The duration for which the notification is shown (in seconds) and after which is automatically
     * closed. May be undefined to indicate no automatic closure.
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `function`
     * (action) pairs. The action returns a boolean which indicates whether to close the notification after the
     * action.
     */
    warn(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(
                NotificationType.Warn,
                content.title,
                content.message,
                (duration ? duration * 1000 : duration),
                actions
            )
        )
    },
    /**
     * This opens a success notification in the lower right corner of the screen. The content and actions can be
     * specified using the parameters down below. The notification can either be closed manually or automatically after
     * the specified amount of time.
     *
     *
     * @param content - The content to be displayed. Must contain a message, but may optionally also contain a title,
     * which will be displayed as title in the notification.
     * @param duration - The duration for which the notification is shown (in seconds) and after which is automatically
     * closed. May be undefined to indicate no automatic closure.
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `function`
     * (action) pairs. The action returns a boolean which indicates whether to close the notification after the
     * action.
     */
    success(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(
                NotificationType.Success,
                content.title,
                content.message,
                (duration ? duration * 1000 : duration),
                actions
            )
        )
    },
    /**
     * This opens an error message in the lower right corner of the screen. The content and actions can be specified
     * using the parameters down below. The notification can either be closed manually or automatically after the
     * specified amount of time.
     *
     *
     * @param content - The content to be displayed. Must contain a message, but may optionally also contain a title,
     * which will be displayed as title in the notification.
     * @param duration - The duration for which the notification is shown (in seconds) and after which is automatically
     * closed. May be undefined to indicate no automatic closure.
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `function`
     * (action) pairs. The action returns a boolean which indicates whether to close the notification after the
     * action.
     */
    error(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Error,
                content.title,
                content.message,
                (duration ? duration * 1000 : duration),
                actions
            )
        )
    }
}
export default notification