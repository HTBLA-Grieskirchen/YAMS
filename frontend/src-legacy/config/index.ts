import rawDefaultConfig from "./config.json";
import { invoke } from "@tauri-apps/api/core";

const defaultConfig = rawDefaultConfig as FrontendConfig;
export { defaultConfig };
type FrontendConfig = {
  remoteDatabaseLocation: string | null;
};

const isTauri =
  typeof window !== "undefined" &&
  (window as any).__TAURI_INTERNALS__ !== undefined;

export interface TauriType {
  invoke: typeof invoke;
}

const tauri: TauriType | undefined = isTauri ? { invoke } : undefined;
export { tauri };

const loadConfig = async () => {
  if (tauri) {
    return (await invoke("frontend_config")) as FrontendConfig;
  } else {
    return defaultConfig;
  }
};

const loadPromise = loadConfig();

export default async function config() {
  return await loadPromise;
}
