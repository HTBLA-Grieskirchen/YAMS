import { invoke, isTauri } from "@tauri-apps/api/core";

import browserConfig from "@/yams-config.json";
import { HttpYamsApi } from "./http-client";
import { TauriYamsApi } from "./tauri-client";
import type { FrontendConfig } from "./types";
import type { YamsApi } from "./yams-api";

const DEFAULT_REMOTE_API_URL = "http://127.0.0.1:3000/api";

let cachedConfig: FrontendConfig | null = null;
let cachedApi: YamsApi | null = null;

type BrowserConfigFile = {
  remoteApiUrl?: string;
  dev?: boolean;
};

function envRemoteApiUrl(): string | undefined {
  return process.env.NEXT_PUBLIC_YAMS_API_URL;
}

function envDev(): boolean | undefined {
  const value = process.env.NEXT_PUBLIC_YAMS_DEV;
  if (value === undefined) {
    return undefined;
  }
  return (
    value === "1" ||
    value.toLowerCase() === "true" ||
    value.toLowerCase() === "yes"
  );
}

function normalizeApiBaseUrl(url: string): string {
  const trimmed = url.replace(/\/$/, "");
  if (trimmed.endsWith("/api")) {
    return trimmed;
  }
  return `${trimmed}/api`;
}

function loadBrowserConfig(): FrontendConfig {
  const file = browserConfig as BrowserConfigFile;
  const remoteApiUrl = normalizeApiBaseUrl(
    envRemoteApiUrl() ?? file.remoteApiUrl ?? DEFAULT_REMOTE_API_URL,
  );
  const dev = envDev() ?? file.dev ?? false;
  return { mode: "remote", remoteApiUrl, dev };
}

export async function loadFrontendConfig(): Promise<FrontendConfig> {
  if (cachedConfig) {
    return cachedConfig;
  }

  cachedConfig = isTauri()
    ? await invoke<FrontendConfig>("frontend_config")
    : loadBrowserConfig();

  return cachedConfig;
}

export async function createYamsApi(): Promise<YamsApi> {
  if (cachedApi) {
    return cachedApi;
  }

  const config = await loadFrontendConfig();
  cachedApi =
    config.mode === "embedded"
      ? new TauriYamsApi()
      : new HttpYamsApi(normalizeApiBaseUrl(config.remoteApiUrl));

  return cachedApi;
}

export function resetYamsApiCache(): void {
  cachedApi = null;
  cachedConfig = null;
}

export async function getYamsApi(): Promise<YamsApi> {
  return createYamsApi();
}
