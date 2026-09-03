import { useMutation, useQueryClient } from "@tanstack/react-query";

import { yamsKeys } from "../query-keys";
import type {
  BehandlungErstellung,
  HaustierErstellung,
  KlientErstellung,
  LeistungAusBehandlungErstellung,
  LeistungAusProduktErstellung,
  LeistungManuelleErstellung,
  ProduktErstellung,
  SeminarBuchungErstellung,
  SeminarErstellung,
  SeminarTerminAbsage,
  SeminarTerminAktualisierung,
  SeminarTerminErstellung,
  TagesabschlussErstellung,
} from "../types";
import { useYamsApiReady } from "./use-yams-api-ready";

export function useKlientErstellenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: KlientErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.klientErstellen(body);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.klienten.all() });
    },
  });
}

export function useHaustierErstellenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: HaustierErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.haustierErstellen(body);
    },
    onSuccess: (haustier) => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.haustiere.all() });
      queryClient.setQueryData(
        yamsKeys.haustiere.detail(haustier.id),
        haustier,
      );
    },
  });
}

export function useProduktErstellenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: ProduktErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.produktErstellen(body);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.produkte.all() });
    },
  });
}

export function useBehandlungErstellenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: BehandlungErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.behandlungErstellen(body);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.behandlungen.all() });
    },
  });
}

export function useLeistungAusProduktBuchenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: LeistungAusProduktErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.leistungAusProduktBuchen(body);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.leistungen.all() });
    },
  });
}

export function useLeistungAusBehandlungBuchenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: LeistungAusBehandlungErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.leistungAusBehandlungBuchen(body);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.leistungen.all() });
    },
  });
}

export function useLeistungManuellErfassenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: LeistungManuelleErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.leistungManuellErfassen(body);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.leistungen.all() });
    },
  });
}

export function useTagesabschlussDurchführenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: TagesabschlussErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.tagesabschlussDurchführen(body);
    },
    onSuccess: (rechnungen) => {
      queryClient.invalidateQueries({ queryKey: yamsKeys.rechnungen.all() });
      const klientIds = new Set(rechnungen.map((r) => r.klientId));
      for (const klientId of klientIds) {
        queryClient.invalidateQueries({
          queryKey: yamsKeys.rechnungen.byKlient(klientId),
        });
      }
    },
  });
}

export function useSeminarErstellenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: SeminarErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.seminarErstellen(body);
    },
    onSuccess: (seminar) => {
      queryClient.setQueryData(yamsKeys.seminare.detail(seminar.id), seminar);
      queryClient.invalidateQueries({ queryKey: yamsKeys.seminare.all() });
    },
  });
}

export function useSeminarTerminPlanenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: SeminarTerminErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.seminarTerminPlanen(body);
    },
    onSuccess: (termin) => {
      queryClient.setQueryData(
        yamsKeys.seminarTermine.detail(termin.id),
        termin,
      );
      queryClient.invalidateQueries({ queryKey: yamsKeys.seminarTermine.all() });
    },
  });
}

export function useSeminarBuchungAnlegenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      terminId,
      body,
    }: {
      terminId: string;
      body: SeminarBuchungErstellung;
    }) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.seminarBuchungAnlegen(terminId, body);
    },
    onSuccess: (termin) => {
      queryClient.setQueryData(
        yamsKeys.seminarTermine.detail(termin.id),
        termin,
      );
      queryClient.invalidateQueries({
        queryKey: yamsKeys.seminarTermine.umsatz(termin.id),
      });
    },
  });
}

export function useSeminarBuchungStornierenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      terminId,
      buchungId,
    }: {
      terminId: string;
      buchungId: string;
    }) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.seminarBuchungStornieren(terminId, buchungId);
    },
    onSuccess: (termin) => {
      queryClient.setQueryData(
        yamsKeys.seminarTermine.detail(termin.id),
        termin,
      );
      queryClient.invalidateQueries({
        queryKey: yamsKeys.seminarTermine.umsatz(termin.id),
      });
    },
  });
}

export function useSeminarTerminAbsagenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      terminId,
      body,
    }: {
      terminId: string;
      body: SeminarTerminAbsage;
    }) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.seminarTerminAbsagen(terminId, body);
    },
    onSuccess: (termin) => {
      queryClient.setQueryData(
        yamsKeys.seminarTermine.detail(termin.id),
        termin,
      );
    },
  });
}

export function useSeminarTerminAbgehaltenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (terminId: string) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.seminarTerminAbgehalten(terminId);
    },
    onSuccess: (termin) => {
      queryClient.setQueryData(
        yamsKeys.seminarTermine.detail(termin.id),
        termin,
      );
    },
  });
}

export function useSeminarTerminAktualisierenMutation() {
  const { api } = useYamsApiReady();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      id,
      body,
    }: {
      id: string;
      body: SeminarTerminAktualisierung;
    }) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.seminarTerminAktualisieren(id, body);
    },
    onSuccess: (termin) => {
      queryClient.setQueryData(
        yamsKeys.seminarTermine.detail(termin.id),
        termin,
      );
    },
  });
}
