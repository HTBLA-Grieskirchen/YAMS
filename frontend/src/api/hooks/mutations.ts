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
  TagesabschlussErstellung,
} from "../types";
import { useYamsApiReady } from "./use-yams-api-ready";

export function useKlientErstellenMutation() {
  const { api } = useYamsApiReady();

  return useMutation({
    mutationFn: (body: KlientErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.klientErstellen(body);
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

  return useMutation({
    mutationFn: (body: ProduktErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.produktErstellen(body);
    },
  });
}

export function useBehandlungErstellenMutation() {
  const { api } = useYamsApiReady();

  return useMutation({
    mutationFn: (body: BehandlungErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.behandlungErstellen(body);
    },
  });
}

export function useLeistungAusProduktBuchenMutation() {
  const { api } = useYamsApiReady();

  return useMutation({
    mutationFn: (body: LeistungAusProduktErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.leistungAusProduktBuchen(body);
    },
  });
}

export function useLeistungAusBehandlungBuchenMutation() {
  const { api } = useYamsApiReady();

  return useMutation({
    mutationFn: (body: LeistungAusBehandlungErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.leistungAusBehandlungBuchen(body);
    },
  });
}

export function useLeistungManuellErfassenMutation() {
  const { api } = useYamsApiReady();

  return useMutation({
    mutationFn: (body: LeistungManuelleErstellung) => {
      if (!api) {
        throw new Error("YamsApi is not ready");
      }
      return api.leistungManuellErfassen(body);
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
      const klientIds = new Set(rechnungen.map((r) => r.klientId));
      for (const klientId of klientIds) {
        queryClient.invalidateQueries({
          queryKey: yamsKeys.rechnungen.byKlient(klientId),
        });
      }
    },
  });
}
