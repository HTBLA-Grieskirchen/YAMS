"use client";

import Link from "next/link";
import { useMemo, useState } from "react";

import {
  useAlleSeminareQuery,
  useAlleSeminarTermineQuery,
} from "@/api/hooks";
import { SeminarTerminStatus } from "@/api/schema";
import type { SeminarTermin } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { SearchInput } from "@/components/ui/search-input";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { SeminarWorkflowPanel } from "@/components/seminar/seminar-workflow-panel";
import { matchesSearchQuery } from "@/lib/format";
import { paths } from "@/lib/navigation";
import { cn } from "@/lib/cn";

type TerminFilter = "kommend" | "zukunft" | "vergangen";

function terminTimestamp(termin: SeminarTermin): number {
  return new Date(termin.beginn).getTime();
}

function classifyTermin(termin: SeminarTermin, now: number): TerminFilter {
  const start = terminTimestamp(termin);
  const end = new Date(termin.ende).getTime();
  if (end < now) return "vergangen";
  if (start >= now) return "zukunft";
  return "kommend";
}

export function SeminarOverview() {
  const seminareQuery = useAlleSeminareQuery();
  const termineQuery = useAlleSeminarTermineQuery();
  const [filter, setFilter] = useState("");
  const [category, setCategory] = useState<TerminFilter>("kommend");
  const [showPastInUpcoming, setShowPastInUpcoming] = useState(true);

  const seminareById = useMemo(() => {
    return new Map((seminareQuery.data ?? []).map((seminar) => [seminar.id, seminar]));
  }, [seminareQuery.data]);

  const filteredTermine = useMemo(() => {
    const now = Date.now();
    const all = (termineQuery.data ?? []).filter((termin) => {
      const seminar = seminareById.get(termin.seminarId);
      return matchesSearchQuery(filter, [
        seminar?.titel,
        termin.status,
        termin.ort.ortName,
        termin.ort.adresse?.straßeUndHausnummer,
        termin.ort.adresse?.stadt,
      ]);
    });

    if (category === "zukunft") {
      return all
        .filter((termin) => terminTimestamp(termin) >= now)
        .sort((a, b) => terminTimestamp(a) - terminTimestamp(b));
    }

    if (category === "vergangen") {
      return all
        .filter((termin) => new Date(termin.ende).getTime() < now)
        .sort((a, b) => terminTimestamp(b) - terminTimestamp(a));
    }

    const upcoming = all
      .filter((termin) => {
        const bucket = classifyTermin(termin, now);
        if (bucket === "zukunft") return true;
        if (bucket === "vergangen") return showPastInUpcoming;
        return true;
      })
      .sort((a, b) => terminTimestamp(a) - terminTimestamp(b));

    return upcoming.slice(0, 8);
  }, [
    category,
    filter,
    seminareById,
    showPastInUpcoming,
    termineQuery.data,
  ]);

  const isLoading = seminareQuery.isPending || termineQuery.isPending;
  const error = seminareQuery.error ?? termineQuery.error;

  return (
    <div className="space-y-6 p-6">
      <div className="grid gap-6 xl:grid-cols-[minmax(0,1fr)_22rem]">
        <Card>
          <CardHeader>
            <div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
              <div>
                <CardTitle>Seminar-Termine</CardTitle>
                <CardDescription>
                  Übersicht wie in der Legacy-Events-Ansicht: filtern, suchen,
                  Termine verwalten.
                </CardDescription>
              </div>
              <SearchInput
                value={filter}
                onChange={(event) => setFilter(event.target.value)}
                placeholder="Seminar suchen…"
              />
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-col gap-3 sm:flex-row sm:items-center">
              <label className="text-sm text-zinc-600 dark:text-zinc-400">
                Anzeige
                <select
                  className="mt-1 block h-10 rounded-lg border border-zinc-300 bg-white px-3 text-sm dark:border-zinc-700 dark:bg-zinc-900"
                  value={category}
                  onChange={(event) =>
                    setCategory(event.target.value as TerminFilter)
                  }
                >
                  <option value="kommend">Kommende</option>
                  <option value="zukunft">Alle zukünftigen</option>
                  <option value="vergangen">Vergangene</option>
                </select>
              </label>

              {category === "kommend" ? (
                <label className="flex items-center gap-2 text-sm">
                  <input
                    type="checkbox"
                    checked={showPastInUpcoming}
                    onChange={(event) =>
                      setShowPastInUpcoming(event.target.checked)
                    }
                  />
                  Vergangene einbeziehen
                </label>
              ) : null}
            </div>

            {isLoading ? (
              <p className="text-sm text-zinc-500">Lade Termine…</p>
            ) : null}
            {error ? <Alert variant="error">{String(error)}</Alert> : null}

            <div className="divide-y divide-zinc-200 dark:divide-zinc-800">
              {!isLoading && filteredTermine.length === 0 ? (
                <p className="py-6 text-sm text-zinc-500">
                  Keine Termine in dieser Ansicht.
                </p>
              ) : null}

              {filteredTermine.map((termin) => {
                const seminar = seminareById.get(termin.seminarId);
                return (
                  <article key={termin.id} className="py-4">
                    <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                      <div>
                        <h3 className="font-medium">
                          {seminar?.titel ?? "Seminar"}
                        </h3>
                        <p className="text-sm text-zinc-600 dark:text-zinc-400">
                          {new Date(termin.beginn).toLocaleString("de-AT")} –{" "}
                          {new Date(termin.ende).toLocaleString("de-AT")}
                        </p>
                        <p className="text-sm text-zinc-500">
                          {termin.ort.ortName ?? "Ort offen"} ·{" "}
                          {termin.buchungen.length} Buchung(en)
                        </p>
                      </div>
                      <Badge variant={statusBadgeVariant(termin.status)}>
                        {termin.status}
                      </Badge>
                    </div>
                  </article>
                );
              })}
            </div>
          </CardContent>
        </Card>

        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Kalender</CardTitle>
              <CardDescription>Platzhalter wie in Legacy.</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex h-48 items-center justify-center rounded-lg border border-dashed border-zinc-300 text-sm text-zinc-500 dark:border-zinc-700">
                Kalender folgt
              </div>
            </CardContent>
          </Card>

          <Link
            href={paths.abrechnung}
            className={cn(
              "block rounded-xl border border-zinc-200 bg-white p-4 text-sm shadow-sm hover:bg-zinc-50 dark:border-zinc-800 dark:bg-zinc-950 dark:hover:bg-zinc-900",
            )}
          >
            Klient für Buchung im Abrechnungs-Workflow anlegen →
          </Link>
        </div>
      </div>

      <SeminarWorkflowPanel />
    </div>
  );
}

function statusBadgeVariant(
  status: SeminarTerminStatus,
): "default" | "success" | "error" {
  if (status === SeminarTerminStatus.Abgesagt) return "error";
  if (status === SeminarTerminStatus.Abgehalten) return "success";
  return "default";
}
