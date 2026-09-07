import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type { paths } from "./schema";
import type {
  BehandlungErstellung,
  HaustierErstellung,
  KlientErstellung,
  LeistungAusBehandlungErstellung,
  LeistungAusProduktErstellung,
  LeistungManuelleErstellung,
  ProduktErstellung,
  RechnungBezahltMarkieren,
  SeminarBuchungErstellung,
  SeminarErstellung,
  SeminarTerminAbsage,
  SeminarTerminAktualisierung,
  SeminarTerminErstellung,
  TagesabschlussErstellung,
} from "./types";
import type { YamsApi } from "./yams-api";

type JsonClient = ReturnType<typeof createClient<paths>>;

async function unwrap<T>(result: { data?: T; error?: unknown }): Promise<T> {
  if (result.error !== undefined) {
    throw ApiError.fromUnknown(result.error);
  }
  return result.data as T;
}

export class HttpYamsApi implements YamsApi {
  private readonly client: JsonClient;
  private readonly baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
    this.client = createClient<paths>({ baseUrl: this.baseUrl });
  }

  async health(): Promise<string> {
    return unwrap(await this.client.GET("/health"));
  }

  async klientErstellen(body: KlientErstellung) {
    return unwrap(
      await this.client.POST("/klient", {
        body,
      }),
    );
  }

  async haustierErstellen(body: HaustierErstellung) {
    return unwrap(
      await this.client.POST("/haustier", {
        body,
      }),
    );
  }

  async alleHaustiere() {
    return unwrap(await this.client.GET("/haustier"));
  }

  async alleKlienten() {
    return unwrap(await this.client.GET("/klient", {}));
  }

  async alleProdukte() {
    return unwrap(await this.client.GET("/produkt", {}));
  }

  async alleBehandlungen() {
    return unwrap(await this.client.GET("/behandlung", {}));
  }

  async alleLeistungen() {
    return unwrap(await this.client.GET("/leistung", {}));
  }

  async alleRechnungen() {
    return unwrap(await this.client.GET("/rechnungen", {}));
  }

  async alleSeminare() {
    return unwrap(await this.client.GET("/seminar", {}));
  }

  async alleSeminarTermine() {
    return unwrap(await this.client.GET("/seminar-termin", {}));
  }

  async haustierById(id: string) {
    return unwrap(
      await this.client.GET("/haustier/{id}", {
        params: { path: { id } },
      }),
    );
  }

  async produktErstellen(body: ProduktErstellung) {
    return unwrap(
      await this.client.POST("/produkt", {
        body,
      }),
    );
  }

  async behandlungErstellen(body: BehandlungErstellung) {
    return unwrap(
      await this.client.POST("/behandlung", {
        body,
      }),
    );
  }

  async leistungAusProduktBuchen(body: LeistungAusProduktErstellung) {
    return unwrap(
      await this.client.POST("/leistung/produkt", {
        body,
      }),
    );
  }

  async leistungAusBehandlungBuchen(body: LeistungAusBehandlungErstellung) {
    return unwrap(
      await this.client.POST("/leistung/behandlung", {
        body,
      }),
    );
  }

  async leistungManuellErfassen(body: LeistungManuelleErstellung) {
    return unwrap(
      await this.client.POST("/leistung/manuell", {
        body,
      }),
    );
  }

  async tagesabschlussDurchführen(body: TagesabschlussErstellung) {
    return unwrap(
      await this.client.POST("/tagesabschluss", {
        body,
      }),
    );
  }

  async rechnungenFürKlient(klientId: string) {
    return unwrap(
      await this.client.GET("/klient/{klient_id}/rechnungen", {
        params: { path: { klient_id: klientId } },
      }),
    );
  }

  async rechnungAlsBezahltMarkieren(id: string, body: RechnungBezahltMarkieren) {
    return unwrap(
      await this.client.POST("/rechnungen/{id}/bezahlt", {
        params: { path: { id } },
        body,
      }),
    );
  }

  async rechnungPdf(id: string) {
    return this.fetchPdf(`/rechnungen/${encodeURIComponent(id)}/pdf`);
  }

  async teilnahmebestätigungPdf(terminId: string, buchungId: string) {
    return this.fetchPdf(
      `/seminar-termin/${encodeURIComponent(terminId)}/buchung/${encodeURIComponent(buchungId)}/teilnahmebestaetigung`,
    );
  }

  private async fetchPdf(path: string): Promise<Blob> {
    const response = await fetch(`${this.baseUrl}${path}`, {
      headers: { Accept: "application/pdf" },
    });

    if (!response.ok) {
      throw ApiError.fromUnknown(await response.text());
    }

    return response.blob();
  }

  async seminarErstellen(body: SeminarErstellung) {
    return unwrap(
      await this.client.POST("/seminar", {
        body,
      }),
    );
  }

  async seminarById(id: string) {
    return unwrap(
      await this.client.GET("/seminar/{id}", {
        params: { path: { id } },
      }),
    );
  }

  async seminarTerminPlanen(body: SeminarTerminErstellung) {
    return unwrap(
      await this.client.POST("/seminar-termin", {
        body,
      }),
    );
  }

  async seminarTerminById(id: string) {
    return unwrap(
      await this.client.GET("/seminar-termin/{id}", {
        params: { path: { id } },
      }),
    );
  }

  async seminarTerminAktualisieren(
    id: string,
    body: SeminarTerminAktualisierung,
  ) {
    return unwrap(
      await this.client.PUT("/seminar-termin/{id}", {
        params: { path: { id } },
        body,
      }),
    );
  }

  async seminarBuchungAnlegen(
    terminId: string,
    body: SeminarBuchungErstellung,
  ) {
    return unwrap(
      await this.client.POST("/seminar-termin/{id}/buchung", {
        params: { path: { id: terminId } },
        body,
      }),
    );
  }

  async seminarBuchungStornieren(terminId: string, buchungId: string) {
    return unwrap(
      await this.client.POST(
        "/seminar-termin/{id}/buchung/{buchung_id}/storno",
        {
          params: { path: { id: terminId, buchung_id: buchungId } },
        },
      ),
    );
  }

  async seminarTerminAbsagen(terminId: string, body: SeminarTerminAbsage) {
    return unwrap(
      await this.client.POST("/seminar-termin/{id}/absagen", {
        params: { path: { id: terminId } },
        body,
      }),
    );
  }

  async seminarTerminAbgehalten(terminId: string) {
    return unwrap(
      await this.client.POST("/seminar-termin/{id}/abgehalten", {
        params: { path: { id: terminId } },
      }),
    );
  }

  async seminarUmsatzVorschau(terminId: string) {
    return unwrap(
      await this.client.GET("/seminar-termin/{id}/umsatz", {
        params: { path: { id: terminId } },
      }),
    );
  }

  async seminarUmsatzPrognose(stichtag: string) {
    return unwrap(
      await this.client.GET("/seminar-prognose", {
        params: { query: { stichtag } },
      }),
    );
  }
}
