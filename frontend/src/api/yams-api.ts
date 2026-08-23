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

/** Framework-agnostic YAMS API — implemented via OpenAPI HTTP or Tauri invoke. */
export interface YamsApi {
  health(): Promise<string>;
  klientErstellen(body: KlientErstellung): Promise<Klient>;
  haustierErstellen(body: HaustierErstellung): Promise<Haustier>;
  alleHaustiere(): Promise<Haustier[]>;
  haustierById(id: string): Promise<Haustier>;
  produktErstellen(body: ProduktErstellung): Promise<Produkt>;
  behandlungErstellen(body: BehandlungErstellung): Promise<Behandlung>;
  leistungAusProduktBuchen(body: LeistungAusProduktErstellung): Promise<Leistung>;
  leistungAusBehandlungBuchen(
    body: LeistungAusBehandlungErstellung,
  ): Promise<Leistung>;
  leistungManuellErfassen(body: LeistungManuelleErstellung): Promise<Leistung>;
  tagesabschlussDurchführen(body: TagesabschlussErstellung): Promise<Rechnung[]>;
  rechnungenFürKlient(klientId: string): Promise<Rechnung[]>;
}

export type DeploymentMode = "embedded" | "remote";
