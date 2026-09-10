"use client";

import { useMemo, useState } from "react";
import { useAlleKlientenQuery, useAlleRechnungenQuery } from "@/api/hooks";
import { RechnungStatus } from "@/api/schema";
import { RechnungenTable } from "@/components/rechnung/rechnungen-table";
import { Alert } from "@/components/ui/alert";
import { SearchInput } from "@/components/ui/search-input";
import { matchesSearchQuery } from "@/lib/format";

type StatusFilter = "alle" | "offen" | "bezahlt";

export function RechnungenOverview() {
  const rechnungenQuery = useAlleRechnungenQuery();
  const klientenQuery = useAlleKlientenQuery();
  const [filter, setFilter] = useState("");
  const [statusFilter, setStatusFilter] = useState<StatusFilter>("alle");

  const klientenById = useMemo(() => {
    return new Map(
      (klientenQuery.data ?? []).map((klient) => [klient.id, klient]),
    );
  }, [klientenQuery.data]);

  const filtered = useMemo(() => {
    const rechnungen = rechnungenQuery.data ?? [];
    return rechnungen
      .filter((rechnung) => {
        if (statusFilter === "offen") {
          return rechnung.status === RechnungStatus.Offen;
        }
        if (statusFilter === "bezahlt") {
          return rechnung.status === RechnungStatus.Bezahlt;
        }
        return true;
      })
      .filter((rechnung) => {
        const klient = klientenById.get(rechnung.klientId);
        return matchesSearchQuery(filter, [
          rechnung.rechnungsnummer,
          rechnung.gesamtbetragBrutto,
          rechnung.status,
          klient?.vorname,
          klient?.nachname,
          klient?.kundennummer,
        ]);
      })
      .sort((a, b) => b.rechnungsdatum.localeCompare(a.rechnungsdatum));
  }, [filter, klientenById, rechnungenQuery.data, statusFilter]);

  if (rechnungenQuery.isPending || klientenQuery.isPending) {
    return <p className="text-sm text-zinc-500">Lade Rechnungen…</p>;
  }

  if (rechnungenQuery.error || klientenQuery.error) {
    return (
      <Alert variant="error">
        {String(rechnungenQuery.error ?? klientenQuery.error)}
      </Alert>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          {filtered.length} Rechnung(en)
        </p>
        <div className="flex flex-col gap-2 sm:flex-row sm:items-center">
          <div className="flex rounded-lg border border-zinc-200 p-1 dark:border-zinc-800">
            {(
              [
                ["alle", "Alle"],
                ["offen", "Offen"],
                ["bezahlt", "Bezahlt"],
              ] as const
            ).map(([value, label]) => (
              <button
                key={value}
                type="button"
                onClick={() => setStatusFilter(value)}
                className={
                  statusFilter === value
                    ? "rounded-md bg-zinc-900 px-3 py-1.5 text-xs font-medium text-white dark:bg-zinc-100 dark:text-zinc-900"
                    : "rounded-md px-3 py-1.5 text-xs font-medium text-zinc-600 hover:bg-zinc-100 dark:text-zinc-400 dark:hover:bg-zinc-900"
                }
              >
                {label}
              </button>
            ))}
          </div>
          <SearchInput
            value={filter}
            onChange={(event) => setFilter(event.target.value)}
            placeholder="Suchen…"
          />
        </div>
      </div>

      {filtered.length === 0 ? (
        <Alert variant="info">Keine Rechnungen gefunden.</Alert>
      ) : (
        <RechnungenTable
          rechnungen={filtered}
          klientenById={klientenById}
          showKlient
        />
      )}
    </div>
  );
}
