import { invoke, isTauri } from "@tauri-apps/api/core";

import { HttpYamsApi } from "./http-client";
import { TauriYamsApi } from "./tauri-client";
import type { FrontendConfig } from "./types";
import type { DeploymentMode, YamsApi } from "./yams-api";

const DEFAULT_REMOTE_API_URL = "http://127.0.0.1:3000/api";

let cachedApi: YamsApi | null = null;

function envDeploymentMode(): DeploymentMode | undefined {
  const value = process.env.NEXT_PUBLIC_YAMS_MODE;
  if (value === "embedded" || value === "remote") {
    return value;
  }
  return undefined;
}

function envRemoteApiUrl(): string | undefined {
  return process.env.NEXT_PUBLIC_YAMS_API_URL;
}

function normalizeApiBaseUrl(url: string): string {
  const trimmed = url.replace(/\/$/, "");
  if (trimmed.endsWith("/api")) {
    return trimmed;
  }
  return `${trimmed}/api`;
}

async function tauriFrontendConfig(): Promise<FrontendConfig | null> {
  if (!isTauri()) {
    return null;
  }
  return invoke<FrontendConfig>("frontend_config");
}

export async function resolveDeploymentMode(): Promise<DeploymentMode> {
  const forced = envDeploymentMode();
  if (forced) {
    return forced;
  }

  if (isTauri()) {
    const config = await tauriFrontendConfig();
    if (config?.remoteDatabaseLocation) {
      return "remote";
    }
    return "embedded";
  }

  return "remote";
}

export async function resolveRemoteApiBaseUrl(): Promise<string> {
  const fromEnv = envRemoteApiUrl();
  if (fromEnv) {
    return normalizeApiBaseUrl(fromEnv);
  }

  if (isTauri()) {
    const config = await tauriFrontendConfig();
    if (config?.remoteDatabaseLocation) {
      return normalizeApiBaseUrl(config.remoteDatabaseLocation);
    }
  }

  return DEFAULT_REMOTE_API_URL;
}

export async function createYamsApi(): Promise<YamsApi> {
  if (cachedApi) {
    return cachedApi;
  }

  const mode = await resolveDeploymentMode();

  if (mode === "embedded") {
    cachedApi = new TauriYamsApi();
  } else {
    const baseUrl = await resolveRemoteApiBaseUrl();
    cachedApi = new HttpYamsApi(baseUrl);
  }

  return cachedApi;
}

export function resetYamsApiCache(): void {
  cachedApi = null;
}

export async function getYamsApi(): Promise<YamsApi> {
  return createYamsApi();
}
