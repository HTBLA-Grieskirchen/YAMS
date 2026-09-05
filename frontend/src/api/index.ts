import { invoke, isTauri } from "@tauri-apps/api/core";
import { getConfig } from "@/lib/config";
import { HttpYamsApi } from "./http-client";
import { TauriYamsApi } from "./tauri-client";
import type { YamsApi } from "./yams-api";

const DEFAULT_REMOTE_API_URL = "http://127.0.0.1:3000/api";

let cachedApi: YamsApi | null = null;

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

export async function createYamsApi(): Promise<YamsApi> {
  if (cachedApi) {
    return cachedApi;
  }

  const config = await getConfig();
  cachedApi =
    config.mode === "embedded"
      ? new TauriYamsApi()
      : new HttpYamsApi(normalizeApiBaseUrl(config.remoteApiUrl));

  return cachedApi;
}

export function resetYamsApiCache(): void {
  cachedApi = null;
}

export async function getYamsApi(): Promise<YamsApi> {
  return createYamsApi();
}
