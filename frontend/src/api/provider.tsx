"use client";

import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";

import { createYamsApi, loadFrontendConfig, resetYamsApiCache } from "./index";
import type { FrontendConfig } from "./types";
import type { DeploymentMode, YamsApi } from "./yams-api";

type YamsApiContextValue = {
  api: YamsApi | null;
  mode: DeploymentMode | null;
  remoteApiBaseUrl: string | null;
  dev: boolean;
  loading: boolean;
  error: string | null;
  reload: () => void;
};

const YamsApiContext = createContext<YamsApiContextValue | null>(null);

function configView(config: FrontendConfig): {
  mode: DeploymentMode;
  remoteApiBaseUrl: string | null;
  dev: boolean;
} {
  if (config.mode === "remote") {
    return {
      mode: config.mode,
      remoteApiBaseUrl: config.remoteApiUrl,
      dev: config.dev,
    };
  }
  return { mode: config.mode, remoteApiBaseUrl: null, dev: config.dev };
}

export function YamsApiProvider({ children }: { children: ReactNode }) {
  const [api, setApi] = useState<YamsApi | null>(null);
  const [mode, setMode] = useState<DeploymentMode | null>(null);
  const [remoteApiBaseUrl, setRemoteApiBaseUrl] = useState<string | null>(null);
  const [dev, setDev] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [reloadToken, setReloadToken] = useState(0);

  const reload = useCallback(() => {
    resetYamsApiCache();
    setReloadToken((value) => value + 1);
  }, []);

  // reloadToken retriggers bootstrap after resetYamsApiCache()
  useEffect(() => {
    void reloadToken;
    let cancelled = false;

    async function bootstrap() {
      setLoading(true);
      setError(null);

      try {
        const config = await loadFrontendConfig();
        const view = configView(config);
        const resolvedApi = await createYamsApi();

        if (!cancelled) {
          setMode(view.mode);
          setRemoteApiBaseUrl(view.remoteApiBaseUrl);
          setDev(view.dev);
          setApi(resolvedApi);
        }
      } catch (bootstrapError) {
        if (!cancelled) {
          setError(String(bootstrapError));
          setApi(null);
          setMode(null);
          setRemoteApiBaseUrl(null);
          setDev(false);
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    bootstrap();

    return () => {
      cancelled = true;
    };
  }, [reloadToken]);

  const value = useMemo(
    () => ({
      api,
      mode,
      remoteApiBaseUrl,
      dev,
      loading,
      error,
      reload,
    }),
    [api, mode, remoteApiBaseUrl, dev, loading, error, reload],
  );

  return (
    <YamsApiContext.Provider value={value}>
      {children}
      {dev ? <ReactQueryDevtools initialIsOpen={false} /> : null}
    </YamsApiContext.Provider>
  );
}

export function useYamsApi(): YamsApiContextValue {
  const context = useContext(YamsApiContext);
  if (!context) {
    throw new Error("useYamsApi must be used within YamsApiProvider");
  }
  return context;
}
