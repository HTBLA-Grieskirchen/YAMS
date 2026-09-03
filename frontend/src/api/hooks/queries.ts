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

export function useAlleKlientenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.klienten.list(),
    queryFn: () => api!.alleKlienten(),
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

export function useAlleProdukteQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.produkte.list(),
    queryFn: () => api!.alleProdukte(),
    enabled: isReady,
  });
}

export function useAlleBehandlungenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.behandlungen.list(),
    queryFn: () => api!.alleBehandlungen(),
    enabled: isReady,
  });
}

export function useAlleLeistungenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.leistungen.list(),
    queryFn: () => api!.alleLeistungen(),
    enabled: isReady,
  });
}

export function useAlleRechnungenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.rechnungen.list(),
    queryFn: () => api!.alleRechnungen(),
    enabled: isReady,
  });
}

export function useAlleSeminareQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminare.list(),
    queryFn: () => api!.alleSeminare(),
    enabled: isReady,
  });
}

export function useAlleSeminarTermineQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminarTermine.list(),
    queryFn: () => api!.alleSeminarTermine(),
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

export function useRechnungPdfQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.rechnungen.pdf(id ?? ""),
    queryFn: () => api!.rechnungPdf(id!),
    enabled: isReady && id !== undefined && id.length > 0,
  });
}

export function useTeilnahmebestätigungPdfQuery(
  terminId: string | undefined,
  buchungId: string | undefined,
) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.teilnahmebestätigung.pdf(terminId ?? "", buchungId ?? ""),
    queryFn: () => api!.teilnahmebestätigungPdf(terminId!, buchungId!),
    enabled:
      isReady &&
      terminId !== undefined &&
      terminId.length > 0 &&
      buchungId !== undefined &&
      buchungId.length > 0,
  });
}

export function useSeminarByIdQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminare.detail(id ?? ""),
    queryFn: () => api!.seminarById(id!),
    enabled: isReady && id !== undefined && id.length > 0,
  });
}

export function useSeminarTerminByIdQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminarTermine.detail(id ?? ""),
    queryFn: () => api!.seminarTerminById(id!),
    enabled: isReady && id !== undefined && id.length > 0,
  });
}

export function useSeminarUmsatzVorschauQuery(terminId: string | undefined) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminarTermine.umsatz(terminId ?? ""),
    queryFn: () => api!.seminarUmsatzVorschau(terminId!),
    enabled: isReady && terminId !== undefined && terminId.length > 0,
  });
}

export function useSeminarUmsatzPrognoseQuery(stichtag: string | undefined) {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminarPrognose(stichtag ?? ""),
    queryFn: () => api!.seminarUmsatzPrognose(stichtag!),
    enabled: isReady && stichtag !== undefined && stichtag.length > 0,
  });
}
