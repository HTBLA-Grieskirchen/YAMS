import store from "./index";
import {action, makeAutoObservable, observable} from "mobx";
import {DialogInfo} from "../libs/dialog";

export default class DialogStore {
    private readonly currentDialog: DialogInfo[]
    private readonly root: typeof store

    constructor(root: typeof store) {
        this.root = root
        this.currentDialog = observable([])

        makeAutoObservable(this, {
            closeDialog: action.bound
        })
    }

    addDialog(dialog: DialogInfo) {
        this.currentDialog.push(dialog)
    }

    closeDialog(allDialogs?: boolean) {
        const all = allDialogs ?? true
        if (all) {
            this.currentDialog.length = 0
        } else {
            this.currentDialog.pop()
        }
    }

    dialogsShown(): boolean {
        return !!this.currentDialog.length
    }

    dialogs(): DialogInfo[] {
        return this.currentDialog
    }
}
