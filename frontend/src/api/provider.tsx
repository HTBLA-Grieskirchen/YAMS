"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";

import {
  createYamsApi,
  resolveDeploymentMode,
  resolveRemoteApiBaseUrl,
} from "./index";
import type { DeploymentMode, YamsApi } from "./yams-api";

type YamsApiContextValue = {
  api: YamsApi | null;
  mode: DeploymentMode | null;
  remoteApiBaseUrl: string | null;
  loading: boolean;
  error: string | null;
  reload: () => void;
};

const YamsApiContext = createContext<YamsApiContextValue | null>(null);

export function YamsApiProvider({ children }: { children: ReactNode }) {
  const [api, setApi] = useState<YamsApi | null>(null);
  const [mode, setMode] = useState<DeploymentMode | null>(null);
  const [remoteApiBaseUrl, setRemoteApiBaseUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [reloadToken, setReloadToken] = useState(0);

  const reload = useCallback(() => {
    setReloadToken((value) => value + 1);
  }, []);

  useEffect(() => {
    let cancelled = false;

    async function bootstrap() {
      setLoading(true);
      setError(null);

      try {
        const resolvedMode = await resolveDeploymentMode();
        const baseUrl =
          resolvedMode === "remote" ? await resolveRemoteApiBaseUrl() : null;
        const resolvedApi = await createYamsApi();

        if (!cancelled) {
          setMode(resolvedMode);
          setRemoteApiBaseUrl(baseUrl);
          setApi(resolvedApi);
        }
      } catch (bootstrapError) {
        if (!cancelled) {
          setError(String(bootstrapError));
          setApi(null);
          setMode(null);
          setRemoteApiBaseUrl(null);
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
      loading,
      error,
      reload,
    }),
    [api, mode, remoteApiBaseUrl, loading, error, reload],
  );

  return (
    <YamsApiContext.Provider value={value}>{children}</YamsApiContext.Provider>
  );
}

export function useYamsApi(): YamsApiContextValue {
  const context = useContext(YamsApiContext);
  if (!context) {
    throw new Error("useYamsApi must be used within YamsApiProvider");
  }
  return context;
}
