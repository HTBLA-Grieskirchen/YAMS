"use client";

import { useRechnungenFürKlientQuery } from "@/api/hooks";
import type { Klient, Rechnung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

type RechnungenPanelProps = {
  klient: Klient | null;
  localRechnungen: Rechnung[];
};

export function RechnungenPanel({
  klient,
  localRechnungen,
}: RechnungenPanelProps) {
  const query = useRechnungenFürKlientQuery(klient?.id);
  const rechnungen =
    localRechnungen.length > 0 ? localRechnungen : (query.data ?? []);

  return (
    <Card>
      <CardHeader>
        <CardTitle>6. Rechnungen</CardTitle>
        <CardDescription>
          Rechnungen für den aktuellen Klienten nach dem Tagesabschluss.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {!klient ? (
          <p className="text-sm text-zinc-500">Noch kein Klient ausgewählt.</p>
        ) : query.isPending && localRechnungen.length === 0 ? (
          <p className="text-sm text-zinc-500">Lade Rechnungen…</p>
        ) : query.error && localRechnungen.length === 0 ? (
          <Alert variant="error">{String(query.error)}</Alert>
        ) : rechnungen.length === 0 ? (
          <p className="text-sm text-zinc-500">
            Noch keine Rechnungen — führe den Tagesabschluss durch.
          </p>
        ) : (
          <ul className="space-y-4">
            {rechnungen.map((rechnung) => (
              <li
                key={rechnung.id}
                className="rounded-lg border border-zinc-200 p-4 dark:border-zinc-800"
              >
                <div className="flex flex-wrap items-baseline justify-between gap-2">
                  <p className="font-medium">
                    Rechnung #{rechnung.rechnungsnummer}
                  </p>
                  <p className="text-sm text-zinc-500">
                    {rechnung.rechnungsdatum} · {rechnung.status}
                  </p>
                </div>
                <p className="mt-1 text-sm">
                  Gesamt:{" "}
                  <span className="font-mono">{rechnung.gesamtbetragBrutto}</span>{" "}
                  €
                </p>
                <ul className="mt-3 space-y-1 text-sm text-zinc-600 dark:text-zinc-400">
                  {rechnung.positionen.map((position) => (
                    <li key={position.leistungId}>
                      {position.beschreibung} — {position.gesamtpreisBrutto} €
                    </li>
                  ))}
                </ul>
              </li>
            ))}
          </ul>
        )}
      </CardContent>
    </Card>
  );
}
