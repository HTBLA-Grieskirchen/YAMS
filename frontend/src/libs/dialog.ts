import {ReactElement} from "react";
import {makeAutoObservable} from "mobx";
import store from "../stores";
import * as uuid from "uuid";

class DialogInfo {
    readonly uuid: string
    readonly component: DialogComponent

    constructor(component: DialogComponent) {
        this.uuid = uuid.v4()
        this.component = component

        makeAutoObservable(this)
    }
}

type DialogInfoType = DialogInfo
export type {DialogInfoType as DialogInfo}

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
 */
export default function dialog(dialog: DialogComponent) {
    store.dialogStore.addDialog(new DialogInfo(dialog))
}
