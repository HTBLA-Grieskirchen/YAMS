import store from "./index";
import FrontendConfig, {defaultConfig} from "../config/frontendConfig";
import {tauri} from "@tauri-apps/api";

export default class ConfigStore {
    config!: FrontendConfig
    tauri: typeof tauri | null
    private root: typeof store

    constructor(root: typeof store) {
        this.root = root
        if (typeof window !== "undefined" && window.__TAURI__) {
            this.tauri = window.__TAURI__
        } else {
            this.tauri = null
        }
    }

    async setup() {
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