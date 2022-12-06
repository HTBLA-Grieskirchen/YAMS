import rawDefaultConfig from "./config.json"
import {tauri as _tauri} from "@tauri-apps/api";

const defaultConfig = rawDefaultConfig as FrontendConfig
export {defaultConfig}
type FrontendConfig = {
    remoteDatabaseLocation: string | null
}


export type TauriType = typeof _tauri
const tauri: TauriType | undefined = typeof window !== "undefined" && window.__TAURI__ ? window.__TAURI__ : undefined
export {tauri}

const loadConfig = async () => {
    if (tauri) {
        return await tauri.invoke("frontend_config") as FrontendConfig
    } else {
        return defaultConfig
    }
}

const loadPromise = loadConfig()

export default async function config() {
    return await loadPromise
}
