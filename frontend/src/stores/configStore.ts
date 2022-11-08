import store from "./index";
import FrontendConfig, {defaultConfig} from "../config/frontendConfig";
import {tauri} from "@tauri-apps/api";
import {makeAutoObservable} from "mobx";

export default class ConfigStore {
    private root: typeof store
    config!: FrontendConfig
    tauri: typeof tauri | null

    constructor(root: typeof store) {
        this.root = root
        if (typeof window !== "undefined" && window.__TAURI__) {
            this.tauri = window.__TAURI__
        } else {
            this.tauri = null
        }

        makeAutoObservable(this)
    }

    async init() {
        if (this.tauri != null) {
            this.config = await this.tauri.invoke("frontend_config")
        } else {
            this.config = defaultConfig
        }
    }

    isTauri(): boolean {
        return this.tauri != null
    }
}