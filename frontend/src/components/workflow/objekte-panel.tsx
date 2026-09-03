"use client";

import type { ReactNode } from "react";
import type { UseQueryResult } from "@tanstack/react-query";

import { Alert } from "@/components/ui/alert";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  useAlleBehandlungenQuery,
  useAlleHaustiereQuery,
  useAlleKlientenQuery,
  useAlleLeistungenQuery,
  useAlleProdukteQuery,
  useAlleRechnungenQuery,
  useAlleSeminareQuery,
  useAlleSeminarTermineQuery,
} from "@/api/hooks";

type EntityListProps<T> = {
  title: string;
  description: string;
  query: UseQueryResult<T[]>;
  renderItem: (item: T) => ReactNode;
  itemKey: (item: T) => string;
  emptyLabel: string;
};

function EntityList<T>({
  title,
  description,
  query,
  renderItem,
  itemKey,
  emptyLabel,
}: EntityListProps<T>) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent>
        {query.isPending ? (
          <p className="text-sm text-zinc-500">Lade…</p>
        ) : query.error ? (
          <Alert variant="error">{String(query.error)}</Alert>
        ) : (query.data ?? []).length === 0 ? (
          <p className="text-sm text-zinc-500">{emptyLabel}</p>
        ) : (
          <ul className="space-y-3">
            {(query.data ?? []).map((item) => (
              <li
                key={itemKey(item)}
                className="rounded-lg border border-zinc-200 p-3 dark:border-zinc-800"
              >
                {renderItem(item)}
              </li>
            ))}
          </ul>
        )}
      </CardContent>
    </Card>
  );
}

export function ObjektePanel() {
  const klienten = useAlleKlientenQuery();
  const haustiere = useAlleHaustiereQuery();
  const produkte = useAlleProdukteQuery();
  const behandlungen = useAlleBehandlungenQuery();
  const leistungen = useAlleLeistungenQuery();
  const rechnungen = useAlleRechnungenQuery();
  const seminare = useAlleSeminareQuery();
  const termine = useAlleSeminarTermineQuery();

  return (
    <div className="space-y-6">
      <EntityList
        title="Klienten"
        description="Alle Klienten inkl. verknüpfter Haustiere."
        query={klienten}
        itemKey={(k) => k.id}
        emptyLabel="Noch keine Klienten."
        renderItem={(klient) => (
          <>
            <p className="font-medium">
              {klient.vorname} {klient.nachname} · KNr {klient.kundennummer}
            </p>
            <p className="text-sm text-zinc-500">
              {klient.email} · {klient.adresse.stadt} ·{" "}
              {klient.haustiere.length} Haustier(e)
            </p>
          </>
        )}
      />

      <EntityList
        title="Haustiere"
        description="Alle Haustiere im System."
        query={haustiere}
        itemKey={(h) => h.id}
        emptyLabel="Noch keine Haustiere."
        renderItem={(haustier) => (
          <>
            <p className="font-medium">
              {haustier.name} ({haustier.tierart})
            </p>
            <p className="text-sm text-zinc-500">
              Geb. {haustier.geburtstag} · Klient {haustier.klientId.slice(0, 8)}…
            </p>
          </>
        )}
      />

      <EntityList
        title="Produkte"
        description="Katalog-Produkte."
        query={produkte}
        itemKey={(p) => p.id}
        emptyLabel="Noch keine Produkte."
        renderItem={(produkt) => (
          <>
            <p className="font-medium">{produkt.name}</p>
            <p className="text-sm text-zinc-500">
              {produkt.einzelpreis} € netto · MwSt {produkt.mwst}
            </p>
          </>
        )}
      />

      <EntityList
        title="Behandlungen"
        description="Katalog-Behandlungen."
        query={behandlungen}
        itemKey={(b) => b.id}
        emptyLabel="Noch keine Behandlungen."
        renderItem={(behandlung) => (
          <>
            <p className="font-medium">{behandlung.name}</p>
            <p className="text-sm text-zinc-500">
              {behandlung.standardpreis} € · MwSt {behandlung.mwst}
            </p>
          </>
        )}
      />

      <EntityList
        title="Leistungen"
        description="Gebuchte Leistungen (offen und abgerechnet)."
        query={leistungen}
        itemKey={(l) => l.id}
        emptyLabel="Noch keine Leistungen."
        renderItem={(leistung) => (
          <>
            <p className="font-medium">{leistung.beschreibung}</p>
            <p className="text-sm text-zinc-500">
              {leistung.leistungsdatum} · {leistung.status} · Klient{" "}
              {leistung.klientId.slice(0, 8)}…
            </p>
          </>
        )}
      />

      <EntityList
        title="Rechnungen"
        description="Alle Rechnungen."
        query={rechnungen}
        itemKey={(r) => r.id}
        emptyLabel="Noch keine Rechnungen."
        renderItem={(rechnung) => (
          <>
            <p className="font-medium">
              #{rechnung.rechnungsnummer} · {rechnung.gesamtbetragBrutto} €
            </p>
            <p className="text-sm text-zinc-500">
              {rechnung.rechnungsdatum} · {rechnung.status} ·{" "}
              {rechnung.positionen.length} Position(en)
            </p>
          </>
        )}
      />

      <EntityList
        title="Seminare"
        description="Seminar-Stammdaten."
        query={seminare}
        itemKey={(s) => s.id}
        emptyLabel="Noch keine Seminare."
        renderItem={(seminar) => (
          <>
            <p className="font-medium">{seminar.titel}</p>
            <p className="text-sm text-zinc-500">
              {seminar.teilnahmegebührBasis} € netto · MwSt {seminar.mwst}
            </p>
          </>
        )}
      />

      <EntityList
        title="Seminar-Termine"
        description="Geplante und abgeschlossene Termine."
        query={termine}
        itemKey={(t) => t.id}
        emptyLabel="Noch keine Termine."
        renderItem={(termin) => (
          <>
            <p className="font-medium">
              {termin.status} · {termin.beginn} – {termin.ende}
            </p>
            <p className="text-sm text-zinc-500">
              {termin.buchungen.length} Buchung(en) · Ort{" "}
              {termin.ort.ortName ?? "—"}
            </p>
          </>
        )}
      />
    </div>
  );
}
