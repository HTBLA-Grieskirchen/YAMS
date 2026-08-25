import { invoke } from "@tauri-apps/api/core";

import { ApiError } from "./errors";
import type { YamsApi } from "./yams-api";
import type {
  Behandlung,
  BehandlungErstellung,
  Haustier,
  HaustierErstellung,
  Klient,
  KlientErstellung,
  Leistung,
  LeistungAusBehandlungErstellung,
  LeistungAusProduktErstellung,
  LeistungManuelleErstellung,
  Produkt,
  ProduktErstellung,
  Rechnung,
  TagesabschlussErstellung,
} from "./types";

async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw new ApiError(String(error));
  }
}

export class TauriYamsApi implements YamsApi {
  async health(): Promise<string> {
    return "OK";
  }

  async klientErstellen(body: KlientErstellung): Promise<Klient> {
    return invokeCommand("klient_erstellen", { erstellung: body });
  }

  async haustierErstellen(body: HaustierErstellung): Promise<Haustier> {
    return invokeCommand("haustier_erstellen", { erstellung: body });
  }

  async alleHaustiere(): Promise<Haustier[]> {
    return invokeCommand("alle_haustiere");
  }

  async haustierById(id: string): Promise<Haustier> {
    return invokeCommand("haustier_by_id", { id });
  }

  async produktErstellen(body: ProduktErstellung): Promise<Produkt> {
    return invokeCommand("produkt_erstellen", { erstellung: body });
  }

  async behandlungErstellen(body: BehandlungErstellung): Promise<Behandlung> {
    return invokeCommand("behandlung_erstellen", { erstellung: body });
  }

  async leistungAusProduktBuchen(
    body: LeistungAusProduktErstellung,
  ): Promise<Leistung> {
    return invokeCommand("leistung_aus_produkt_buchen", { erstellung: body });
  }

  async leistungAusBehandlungBuchen(
    body: LeistungAusBehandlungErstellung,
  ): Promise<Leistung> {
    return invokeCommand("leistung_aus_behandlung_buchen", { erstellung: body });
  }

  async leistungManuellErfassen(
    body: LeistungManuelleErstellung,
  ): Promise<Leistung> {
    return invokeCommand("leistung_manuell_erfassen", { erstellung: body });
  }

  async tagesabschlussDurchführen(
    body: TagesabschlussErstellung,
  ): Promise<Rechnung[]> {
    return invokeCommand("tagesabschluss_durchführen", { erstellung: body });
  }

  async rechnungenFürKlient(klientId: string): Promise<Rechnung[]> {
    return invokeCommand("rechnungen_für_klient", { klient_id: klientId });
  }

  async rechnungPdf(id: string): Promise<Blob> {
    const bytes = await invokeCommand<number[]>("rechnung_pdf", { id });
    return new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
  }

  async teilnahmebestätigungPdf(
    terminId: string,
    buchungId: string,
  ): Promise<Blob> {
    const bytes = await invokeCommand<number[]>("teilnahmebestätigung_pdf", {
      termin_id: terminId,
      buchung_id: buchungId,
    });
    return new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
  }
}
