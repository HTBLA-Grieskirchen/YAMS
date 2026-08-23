import type { components } from "./schema";

export type Adresse = components["schemas"]["Adresse"];
export type Behandlung = components["schemas"]["Behandlung"];
export type BehandlungErstellung = components["schemas"]["BehandlungErstellung"];
export type Haustier = components["schemas"]["Haustier"];
export type HaustierErstellung = components["schemas"]["HaustierErstellung"];
export type Klient = components["schemas"]["Klient"];
export type KlientErstellung = components["schemas"]["KlientErstellung"];
export type Leistung = components["schemas"]["Leistung"];
export type LeistungAusBehandlungErstellung =
  components["schemas"]["LeistungAusBehandlungErstellung"];
export type LeistungAusProduktErstellung =
  components["schemas"]["LeistungAusProduktErstellung"];
export type LeistungManuelleErstellung =
  components["schemas"]["LeistungManuelleErstellung"];
export type Produkt = components["schemas"]["Produkt"];
export type ProduktErstellung = components["schemas"]["ProduktErstellung"];
export type Rechnung = components["schemas"]["Rechnung"];
export type StructuredError = components["schemas"]["StructuredError"];
export type TagesabschlussErstellung =
  components["schemas"]["TagesabschlussErstellung"];

export type FrontendConfig = {
  remoteDatabaseLocation?: string | null;
};
