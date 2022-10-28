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

export type DialogComponent = (close: (allDialogs?: boolean) => void) => ReactElement

// TODO: Add possibility to also display native dialog on host system if in Tauri
export default function dialog(dialog: DialogComponent) {
    store.dialogStore.addDialog(new DialogInfo(dialog))
}
