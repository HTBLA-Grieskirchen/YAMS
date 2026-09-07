import { useQuery } from "@tanstack/react-query";
import { yamsKeys } from "../query-keys";
import type { YamsApi } from "../yams-api";
import { useYamsApiReady } from "./use-yams-api-ready";

function requireApi(api: YamsApi | null): YamsApi {
  if (!api) {
    throw new Error("YamsApi is not ready");
  }
  return api;
}

export function useHealthQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.health(),
    queryFn: () => api?.health(),
    enabled: isReady,
  });
}

export function useAlleKlientenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.klienten.list(),
    queryFn: () => api?.alleKlienten(),
    enabled: isReady,
  });
}

export function useAlleHaustiereQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.haustiere.list(),
    queryFn: () => api?.alleHaustiere(),
    enabled: isReady,
  });
}

export function useAlleProdukteQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.produkte.list(),
    queryFn: () => api?.alleProdukte(),
    enabled: isReady,
  });
}

export function useAlleBehandlungenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.behandlungen.list(),
    queryFn: () => api?.alleBehandlungen(),
    enabled: isReady,
  });
}

export function useAlleLeistungenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.leistungen.list(),
    queryFn: () => api?.alleLeistungen(),
    enabled: isReady,
  });
}

export function useAlleRechnungenQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.rechnungen.list(),
    queryFn: () => api?.alleRechnungen(),
    enabled: isReady,
  });
}

export function useAlleSeminareQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminare.list(),
    queryFn: () => api?.alleSeminare(),
    enabled: isReady,
  });
}

export function useAlleSeminarTermineQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.seminarTermine.list(),
    queryFn: () => api?.alleSeminarTermine(),
    enabled: isReady,
  });
}

export function useHaustierByIdQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();
  const resolvedId = id ?? "";

  return useQuery({
    queryKey: yamsKeys.haustiere.detail(resolvedId),
    queryFn: () => requireApi(api).haustierById(resolvedId),
    enabled: isReady && resolvedId.length > 0,
  });
}

export function useRechnungenFürKlientQuery(klientId: string | undefined) {
  const { api, isReady } = useYamsApiReady();
  const resolvedKlientId = klientId ?? "";

  return useQuery({
    queryKey: yamsKeys.rechnungen.byKlient(resolvedKlientId),
    queryFn: () => requireApi(api).rechnungenFürKlient(resolvedKlientId),
    enabled: isReady && resolvedKlientId.length > 0,
  });
}

export function useRechnungPdfQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();
  const resolvedId = id ?? "";

  return useQuery({
    queryKey: yamsKeys.rechnungen.pdf(resolvedId),
    queryFn: () => requireApi(api).rechnungPdf(resolvedId),
    enabled: isReady && resolvedId.length > 0,
  });
}

export function useTeilnahmebestätigungPdfQuery(
  terminId: string | undefined,
  buchungId: string | undefined,
) {
  const { api, isReady } = useYamsApiReady();
  const resolvedTerminId = terminId ?? "";
  const resolvedBuchungId = buchungId ?? "";

  return useQuery({
    queryKey: yamsKeys.teilnahmebestätigung.pdf(
      resolvedTerminId,
      resolvedBuchungId,
    ),
    queryFn: () =>
      requireApi(api).teilnahmebestätigungPdf(
        resolvedTerminId,
        resolvedBuchungId,
      ),
    enabled:
      isReady && resolvedTerminId.length > 0 && resolvedBuchungId.length > 0,
  });
}

export function useSeminarByIdQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();
  const resolvedId = id ?? "";

  return useQuery({
    queryKey: yamsKeys.seminare.detail(resolvedId),
    queryFn: () => requireApi(api).seminarById(resolvedId),
    enabled: isReady && resolvedId.length > 0,
  });
}

export function useSeminarTerminByIdQuery(id: string | undefined) {
  const { api, isReady } = useYamsApiReady();
  const resolvedId = id ?? "";

  return useQuery({
    queryKey: yamsKeys.seminarTermine.detail(resolvedId),
    queryFn: () => requireApi(api).seminarTerminById(resolvedId),
    enabled: isReady && resolvedId.length > 0,
  });
}

export function useSeminarUmsatzVorschauQuery(terminId: string | undefined) {
  const { api, isReady } = useYamsApiReady();
  const resolvedTerminId = terminId ?? "";

  return useQuery({
    queryKey: yamsKeys.seminarTermine.umsatz(resolvedTerminId),
    queryFn: () => requireApi(api).seminarUmsatzVorschau(resolvedTerminId),
    enabled: isReady && resolvedTerminId.length > 0,
  });
}

export function useSeminarUmsatzPrognoseQuery(stichtag: string | undefined) {
  const { api, isReady } = useYamsApiReady();
  const resolvedStichtag = stichtag ?? "";

  return useQuery({
    queryKey: yamsKeys.seminarPrognose(resolvedStichtag),
    queryFn: () => requireApi(api).seminarUmsatzPrognose(resolvedStichtag),
    enabled: isReady && resolvedStichtag.length > 0,
  });
}
