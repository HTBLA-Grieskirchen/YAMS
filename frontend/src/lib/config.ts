import { invoke, isTauri } from "@tauri-apps/api/core";
import standaloneConfig from "@/standalone-config.json";

export enum DeploymentMode {
  Embedded = "embedded",
  Remote = "remote",
}

type BaseConfig = {
  isTauri: boolean;
  dev: boolean;
};

export type FrontendConfig = BaseConfig &
  (
    | (typeof standaloneConfig & {
        mode: DeploymentMode.Remote;
      })
    | {
        mode: DeploymentMode.Embedded;
      }
  );

async function loadStandaloneConfig(): Promise<FrontendConfig> {
  return {
    isTauri: false,
    dev: process.env.NODE_ENV === "development",
    mode: DeploymentMode.Remote,
    ...standaloneConfig,
  };
}

async function loadTauriConfig(): Promise<FrontendConfig> {
  const config = (await invoke<Omit<FrontendConfig, "isTauri">>(
    "frontend_config",
  )) as any;
  config.isTauri = true;
  return config as FrontendConfig;
}

async function loadConfig(): Promise<FrontendConfig> {
  return isTauri() ? await loadTauriConfig() : await loadStandaloneConfig();
}

let cachedConfig: FrontendConfig | null = null;
export async function getConfig(): Promise<FrontendConfig> {
  if (cachedConfig) {
    return cachedConfig;
  }
  cachedConfig = await loadConfig();
  return cachedConfig;
}
