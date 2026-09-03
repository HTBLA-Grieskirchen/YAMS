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
  Seminar,
  SeminarBuchungErstellung,
  SeminarErstellung,
  SeminarTermin,
  SeminarTerminAbsage,
  SeminarTerminAktualisierung,
  SeminarTerminErstellung,
  SeminarUmsatzPrognose,
  SeminarUmsatzVorschau,
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

  async alleKlienten(): Promise<Klient[]> {
    return invokeCommand("alle_klienten");
  }

  async alleProdukte(): Promise<Produkt[]> {
    return invokeCommand("alle_produkte");
  }

  async alleBehandlungen(): Promise<Behandlung[]> {
    return invokeCommand("alle_behandlungen");
  }

  async alleLeistungen(): Promise<Leistung[]> {
    return invokeCommand("alle_leistungen");
  }

  async alleRechnungen(): Promise<Rechnung[]> {
    return invokeCommand("alle_rechnungen");
  }

  async alleSeminare(): Promise<Seminar[]> {
    return invokeCommand("alle_seminare");
  }

  async alleSeminarTermine(): Promise<SeminarTermin[]> {
    return invokeCommand("alle_seminar_termine");
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

  async seminarErstellen(body: SeminarErstellung): Promise<Seminar> {
    return invokeCommand("seminar_erstellen", { erstellung: body });
  }

  async seminarById(id: string): Promise<Seminar> {
    return invokeCommand("seminar_by_id", { id });
  }

  async seminarTerminPlanen(
    body: SeminarTerminErstellung,
  ): Promise<SeminarTermin> {
    return invokeCommand("seminar_termin_planen", { erstellung: body });
  }

  async seminarTerminById(id: string): Promise<SeminarTermin> {
    return invokeCommand("seminar_termin_by_id", { id });
  }

  async seminarTerminAktualisieren(
    id: string,
    body: SeminarTerminAktualisierung,
  ): Promise<SeminarTermin> {
    return invokeCommand("seminar_termin_aktualisieren", {
      id,
      aktualisierung: body,
    });
  }

  async seminarBuchungAnlegen(
    terminId: string,
    body: SeminarBuchungErstellung,
  ): Promise<SeminarTermin> {
    return invokeCommand("seminar_buchung_anlegen", {
      termin_id: terminId,
      erstellung: body,
    });
  }

  async seminarBuchungStornieren(
    terminId: string,
    buchungId: string,
  ): Promise<SeminarTermin> {
    return invokeCommand("seminar_buchung_stornieren", {
      termin_id: terminId,
      buchung_id: buchungId,
    });
  }

  async seminarTerminAbsagen(
    terminId: string,
    body: SeminarTerminAbsage,
  ): Promise<SeminarTermin> {
    return invokeCommand("seminar_termin_absagen", {
      termin_id: terminId,
      absage: body,
    });
  }

  async seminarTerminAbgehalten(terminId: string): Promise<SeminarTermin> {
    return invokeCommand("seminar_termin_abgehalten", { termin_id: terminId });
  }

  async seminarUmsatzVorschau(terminId: string): Promise<SeminarUmsatzVorschau> {
    return invokeCommand("seminar_umsatz_vorschau", { termin_id: terminId });
  }

  async seminarUmsatzPrognose(stichtag: string): Promise<SeminarUmsatzPrognose> {
    return invokeCommand("seminar_umsatz_prognose", { stichtag });
  }
}
