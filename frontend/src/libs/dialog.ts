import {ReactElement} from "react";
import {makeAutoObservable} from "mobx";
import store from "../stores";
import * as uuid from "uuid";

class DialogInfo {
    readonly uuid: string
    readonly type: DialogType
    readonly component: DialogComponent

    constructor(component: DialogComponent, type: DialogType = "static") {
        this.uuid = uuid.v4()
        this.type = type
        this.component = component

        makeAutoObservable(this)
    }
}

type DialogInfoType = DialogInfo
export type {DialogInfoType as DialogInfo}
/**
 * - responsive: displayed at bottom on small screens
 * - static: always displayed at center
 */
export type DialogType = "responsive" | "static"

/**
 * A DialogComponent returns a ReactElement which will be displayed in the dialog. It receives a closing
 * function, which clauses the opened dialog upon invocation. The close function accepts a boolean arguments, which,
 * if true or not given/by default, closes all currently active dialogs.
 */
export type DialogComponent = (close: (allDialogs?: boolean) => void) => ReactElement

// TODO: Add possibility to also display native dialog on host system if in Tauri
/**
 * The dialog function generally speaking creates a modal dialog and displays the given
 * [DialogComponent](DialogComponent). For more information on that component, refer to its documentation.
 *
 * @param dialog - a simple [dialog content component](DialogComponent)
 * @param type - the [type of dialog](DialogType)
 */
export default function dialog(dialog: DialogComponent, type: DialogType = "static") {
    store.dialogStore.addDialog(new DialogInfo(dialog, type))
}
