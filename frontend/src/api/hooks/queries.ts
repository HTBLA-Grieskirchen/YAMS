import { useQuery } from "@tanstack/react-query";

import { yamsKeys } from "../query-keys";
import { useYamsApiReady } from "./use-yams-api-ready";

export function useHealthQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.health(),
    queryFn: () => api!.health(),
    enabled: isReady,
  });
}

export function useAlleHaustiereQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.haustiere.list(),
    queryFn: () => api!.alleHaustiere(),
    enabled: isReady,
  });
}

export function useHaustierByIdQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.haustiere.detail(id ?? ""),
    queryFn: () => api!.haustierById(id!),
    enabled: isReady && id !== undefined && id.length > 0,
  });
}

export function useRechnungenFürKlientQuery(klientId: string | undefined) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.rechnungen.byKlient(klientId ?? ""),
    queryFn: () => api!.rechnungenFürKlient(klientId!),
    enabled: isReady && klientId !== undefined && klientId.length > 0,
  });
}
