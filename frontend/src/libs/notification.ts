import {makeAutoObservable} from "mobx";
import store from "../stores";
import * as uuid from "uuid";

class NotificationInfo {
    readonly uuid: string
    type: NotificationType
    title?: string
    message: string
    actions?: NotificationActions
    msPassed: number
    readonly duration?: number

    constructor(type: NotificationType, message: string, title?: string, duration?: number, actions?: NotificationActions) {
        this.type = type
        this.title = title
        this.message = message
        this.actions = actions
        this.msPassed = 0
        this.duration = duration
        this.uuid = uuid.v4()

        makeAutoObservable(this, {
            actions: false
        })
    }

    tick(elapsed: number) {
        this.msPassed += elapsed
    }
}

export enum NotificationType {
    Neutral,
    Info,
    Success,
    Warn,
    Error
}

type NotificationActions = {
    [key: string]: NotificationBehaviour
}

export type NotificationBehaviour = {
    action: () => Promise<boolean> | boolean
    type?: "standard" | "neutral" | "ghost"
    disabled?: () => boolean
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
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `object`
     * (behaviour) pairs. The `action` in the behaviour returns a boolean which indicates whether to close the
     * notification after the action. The `disabled` flag in the behaviour determines if the action can currently
     * be clicked (defaults to false).
     */
    neutral(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Neutral, content.message, content.title, (duration ? duration * 1000 : duration), actions)
        )
    },
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
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `object`
     * (behaviour) pairs. The `action` in the behaviour returns a boolean which indicates whether to close the
     * notification after the action. The `disabled` flag in the behaviour determines if the action can currently
     * be clicked (defaults to false).
     */
    info(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Info, content.message, content.title, (duration ? duration * 1000 : duration), actions)
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
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `object`
     * (behaviour) pairs. The `action` in the behaviour returns a boolean which indicates whether to close the
     * notification after the action. The `disabled` flag in the behaviour determines if the action can currently
     * be clicked (defaults to false).
     */
    warn(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Warn, content.message, content.title, (duration ? duration * 1000 : duration), actions)
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
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `object`
     * (behaviour) pairs. The `action` in the behaviour returns a boolean which indicates whether to close the
     * notification after the action. The `disabled` flag in the behaviour determines if the action can currently
     * be clicked (defaults to false).
     */
    success(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Success, content.message, content.title, (duration ? duration * 1000 : duration), actions)
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
     * @param actions - The possible actions in response to the notification. These are `string` (label) - `object`
     * (behaviour) pairs. The `action` in the behaviour returns a boolean which indicates whether to close the
     * notification after the action. The `disabled` flag in the behaviour determines if the action can currently
     * be clicked (defaults to false).
     */
    error(content: NotificationContent, duration?: number, actions?: NotificationActions) {
        store.notificationStore.addNotification(
            new NotificationInfo(NotificationType.Error, content.message, content.title, (duration ? duration * 1000 : duration), actions)
        )
    }
}
export default notification