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

/** Framework-agnostic YAMS API — implemented via OpenAPI HTTP or Tauri invoke. */
export interface YamsApi {
  health(): Promise<string>;
  klientErstellen(body: KlientErstellung): Promise<Klient>;
  alleKlienten(): Promise<Klient[]>;
  haustierErstellen(body: HaustierErstellung): Promise<Haustier>;
  alleHaustiere(): Promise<Haustier[]>;
  haustierById(id: string): Promise<Haustier>;
  produktErstellen(body: ProduktErstellung): Promise<Produkt>;
  alleProdukte(): Promise<Produkt[]>;
  behandlungErstellen(body: BehandlungErstellung): Promise<Behandlung>;
  alleBehandlungen(): Promise<Behandlung[]>;
  leistungAusProduktBuchen(body: LeistungAusProduktErstellung): Promise<Leistung>;
  leistungAusBehandlungBuchen(
    body: LeistungAusBehandlungErstellung,
  ): Promise<Leistung>;
  leistungManuellErfassen(body: LeistungManuelleErstellung): Promise<Leistung>;
  alleLeistungen(): Promise<Leistung[]>;
  tagesabschlussDurchführen(body: TagesabschlussErstellung): Promise<Rechnung[]>;
  alleRechnungen(): Promise<Rechnung[]>;
  rechnungenFürKlient(klientId: string): Promise<Rechnung[]>;
  rechnungPdf(id: string): Promise<Blob>;
  teilnahmebestätigungPdf(terminId: string, buchungId: string): Promise<Blob>;
  seminarErstellen(body: SeminarErstellung): Promise<Seminar>;
  alleSeminare(): Promise<Seminar[]>;
  seminarById(id: string): Promise<Seminar>;
  seminarTerminPlanen(body: SeminarTerminErstellung): Promise<SeminarTermin>;
  alleSeminarTermine(): Promise<SeminarTermin[]>;
  seminarTerminById(id: string): Promise<SeminarTermin>;
  seminarTerminAktualisieren(
    id: string,
    body: SeminarTerminAktualisierung,
  ): Promise<SeminarTermin>;
  seminarBuchungAnlegen(
    terminId: string,
    body: SeminarBuchungErstellung,
  ): Promise<SeminarTermin>;
  seminarBuchungStornieren(
    terminId: string,
    buchungId: string,
  ): Promise<SeminarTermin>;
  seminarTerminAbsagen(
    terminId: string,
    body: SeminarTerminAbsage,
  ): Promise<SeminarTermin>;
  seminarTerminAbgehalten(terminId: string): Promise<SeminarTermin>;
  seminarUmsatzVorschau(terminId: string): Promise<SeminarUmsatzVorschau>;
  seminarUmsatzPrognose(stichtag: string): Promise<SeminarUmsatzPrognose>;
}

export type DeploymentMode = "embedded" | "remote";
