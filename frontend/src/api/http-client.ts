import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type { paths } from "./schema";
import type { YamsApi } from "./yams-api";
import type {
  BehandlungErstellung,
  HaustierErstellung,
  KlientErstellung,
  LeistungAusBehandlungErstellung,
  LeistungAusProduktErstellung,
  LeistungManuelleErstellung,
  ProduktErstellung,
  TagesabschlussErstellung,
} from "./types";

type JsonClient = ReturnType<typeof createClient<paths>>;

async function unwrap<T>(result: {
  data?: T;
  error?: unknown;
}): Promise<T> {
  if (result.error !== undefined) {
    throw ApiError.fromUnknown(result.error);
  }
  return result.data as T;
}

export class HttpYamsApi implements YamsApi {
  private readonly client: JsonClient;

  constructor(baseUrl: string) {
    this.client = createClient<paths>({ baseUrl });
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
      await this.client.GET("/rechnung/{klient_id}", {
        params: { path: { klient_id: klientId } },
      }),
    );
  }

  async rechnungPdf(id: string) {
    return unwrap(
      await this.client.GET("/rechnung/{id}/pdf", {
        params: { path: { id } },
        parseAs: "blob",
      }),
    );
  }

  async teilnahmebestätigungPdf(terminId: string, buchungId: string) {
    return unwrap(
      await this.client.GET(
        "/seminar-termin/{id}/buchung/{buchung_id}/teilnahmebestätigung",
        {
          params: { path: { id: terminId, buchung_id: buchungId } },
          parseAs: "blob",
        },
      ),
    );
  }
}
