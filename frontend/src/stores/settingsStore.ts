import autoStore from "../libs/autoStore";
import store from "./index";
import {autorun, makeAutoObservable} from "mobx";

export default class SettingsStore {
    theme: string | null
    language?: string

    constructor(root: typeof store) {
        this.theme = null
        this.language = "en_US"

        makeAutoObservable(this)
    }

    async setup() {
        autorun(() => {
            if (typeof document === "undefined") return
            if (!this.theme) {
                document.body.removeAttribute("data-theme")
            } else {
                document.body.setAttribute("data-theme", this.theme)
            }
        })

        autoStore(this, "settings")
    }

    setTheme(theme: SettingsStore["theme"]) {
        this.theme = theme
    }

    setLanguage(language: SettingsStore["language"]) {
        this.language = language
    }
}