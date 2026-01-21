import { useMemo } from "react";
import { getBackendClient, BackendClient } from "./client";
// Import config if needed, or use a default
import config from "../../yamsconfig.json";

export const useBackend = (): BackendClient => {
  const backendClient = useMemo(() => {
    const isStandalone = !!(config as any).remoteDatabaseLocation;
    return getBackendClient({
      mode: isStandalone ? 'standalone' : 'embedded',
      serverUrl: (config as any).remoteDatabaseLocation?.replace("/rpc", ""), // Strip /rpc for REST API
    });
  }, []);

  return backendClient;
};
