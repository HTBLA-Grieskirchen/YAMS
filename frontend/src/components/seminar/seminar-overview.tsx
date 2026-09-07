"use client";

import { useMemo, useRef, useState } from "react";

import { useAlleSeminareQuery, useAlleSeminarTermineQuery } from "@/api/hooks";
import { SeminarTerminStatus } from "@/api/schema";
import type { SeminarTermin } from "@/api/types";
import { TerminDetailPanel } from "@/components/seminar/termin-detail-panel";
import {
  buildTerminEventsByDay,
  TerminCalendar,
} from "@/components/seminar/termin-calendar";
import { TerminPlanForm } from "@/components/seminar/termin-plan-form";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { SearchInput } from "@/components/ui/search-input";
import { cn } from "@/lib/cn";
import { matchesSearchQuery } from "@/lib/format";

type TerminFilter = "alle" | "geplant" | "vergangen";

export function SeminarOverview() {
  const seminareQuery = useAlleSeminareQuery();
  const termineQuery = useAlleSeminarTermineQuery();
  const [filter, setFilter] = useState("");
  const [category, setCategory] = useState<TerminFilter>("geplant");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [selectedCalendarDate, setSelectedCalendarDate] = useState<string | null>(
    null,
  );
  const [planPrefillDate, setPlanPrefillDate] = useState<string | null>(null);
  const planFormRef = useRef<HTMLDivElement>(null);

  const seminareById = useMemo(
    () =>
      new Map(
        (seminareQuery.data ?? []).map((seminar) => [seminar.id, seminar]),
      ),
    [seminareQuery.data],
  );

  const termine = termineQuery.data ?? [];
  const now = Date.now();

  const filteredTermine = useMemo(() => {
    return termine
      .filter((termin) => {
        const seminar = seminareById.get(termin.seminarId);
        if (
          !matchesSearchQuery(filter, [
            seminar?.titel,
            termin.status,
            termin.ort.ortName,
            termin.ort.adresse?.stadt,
          ])
        ) {
          return false;
        }
        if (category === "geplant") {
          return termin.status === SeminarTerminStatus.Geplant;
        }
        if (category === "vergangen") {
          return (
            termin.status !== SeminarTerminStatus.Geplant ||
            new Date(termin.ende).getTime() < now
          );
        }
        return true;
      })
      .sort(
        (a, b) => new Date(a.beginn).getTime() - new Date(b.beginn).getTime(),
      );
  }, [category, filter, now, seminareById, termine]);

  const selectedTermin = useMemo(() => {
    const id = selectedId ?? filteredTermine[0]?.id ?? null;
    if (!id) return null;
    return termine.find((t) => t.id === id) ?? null;
  }, [filteredTermine, selectedId, termine]);

  const visibleTerminIds = useMemo(
    () => new Set(filteredTermine.map((termin) => termin.id)),
    [filteredTermine],
  );

  const eventsByDay = useMemo(
    () => buildTerminEventsByDay(termine, visibleTerminIds),
    [termine, visibleTerminIds],
  );

  const isLoading = seminareQuery.isPending || termineQuery.isPending;
  const error = seminareQuery.error ?? termineQuery.error;

  function handleTerminUpdated(updated: SeminarTermin) {
    setSelectedId(updated.id);
  }

  function handlePlanTerminOnDate(date: string) {
    setPlanPrefillDate(date);
    planFormRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
  }

  return (
    <div className="space-y-6 p-6">
      <div className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">Seminar</h1>
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          Termine planen, Teilnehmer buchen und Termine verwalten.
        </p>
      </div>

      <div className="grid gap-6 xl:grid-cols-[minmax(0,1fr)_minmax(0,1.2fr)]">
        <div className="space-y-4">
          <Card>
            <CardHeader>
              <div className="flex flex-col gap-3 lg:flex-row lg:items-end lg:justify-between">
                <div>
                  <CardTitle>Termine</CardTitle>
                  <CardDescription>
                    {filteredTermine.length} Termin(e) in dieser Ansicht
                  </CardDescription>
                </div>
                <SearchInput
                  value={filter}
                  onChange={(e) => setFilter(e.target.value)}
                  placeholder="Termin suchen…"
                />
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex flex-wrap gap-2">
                {(
                  [
                    ["geplant", "Geplant"],
                    ["alle", "Alle"],
                    ["vergangen", "Vergangen/Abgeschlossen"],
                  ] as const
                ).map(([value, label]) => (
                  <button
                    key={value}
                    type="button"
                    onClick={() => setCategory(value)}
                    className={cn(
                      "rounded-lg px-3 py-1.5 text-xs font-medium",
                      category === value
                        ? "bg-emerald-600 text-white"
                        : "bg-zinc-100 text-zinc-700 dark:bg-zinc-800 dark:text-zinc-200",
                    )}
                  >
                    {label}
                  </button>
                ))}
              </div>

              {isLoading ? (
                <p className="text-sm text-zinc-500">Lade Termine…</p>
              ) : null}
              {error ? <Alert variant="error">{String(error)}</Alert> : null}

              <div className="divide-y divide-zinc-200 dark:divide-zinc-800">
                {!isLoading && filteredTermine.length === 0 ? (
                  <p className="py-4 text-sm text-zinc-500">
                    Keine Termine — plane einen neuen Termin.
                  </p>
                ) : null}

                {filteredTermine.map((termin) => {
                  const seminar = seminareById.get(termin.seminarId);
                  const active =
                    selectedTermin?.id === termin.id ||
                    (!selectedId && termin.id === filteredTermine[0]?.id);
                  return (
                    <button
                      key={termin.id}
                      type="button"
                      onClick={() => setSelectedId(termin.id)}
                      className={cn(
                        "w-full py-4 text-left transition-colors",
                        active && "bg-emerald-50/60 dark:bg-emerald-950/20",
                      )}
                    >
                      <div className="flex items-start justify-between gap-3">
                        <div>
                          <p className="font-medium">
                            {seminar?.titel ?? "Seminar"}
                          </p>
                          <p className="text-sm text-zinc-600 dark:text-zinc-400">
                            {new Date(termin.beginn).toLocaleString("de-AT")}
                          </p>
                          <p className="text-xs text-zinc-500">
                            {termin.buchungen.length} Teilnehmer ·{" "}
                            {termin.ort.ortName ?? "Ort offen"}
                          </p>
                        </div>
                        <Badge variant={statusBadgeVariant(termin.status)}>
                          {termin.status}
                        </Badge>
                      </div>
                    </button>
                  );
                })}
              </div>
            </CardContent>
          </Card>

          <div ref={planFormRef}>
            <TerminPlanForm
              prefillDate={planPrefillDate}
              onPlanned={() => setPlanPrefillDate(null)}
            />
          </div>
        </div>

        <div className="space-y-6">
          <TerminCalendar
            eventsByDay={eventsByDay}
            selectedDate={selectedCalendarDate}
            onSelectDate={setSelectedCalendarDate}
            onPlanTermin={handlePlanTerminOnDate}
          />

          {selectedTermin ? (
            <TerminDetailPanel
              termin={selectedTermin}
              seminar={seminareById.get(selectedTermin.seminarId)}
              onUpdated={handleTerminUpdated}
            />
          ) : (
            <Card>
              <CardHeader>
                <CardTitle>Termin-Details</CardTitle>
                <CardDescription>
                  Wähle einen Termin oder lege einen neuen an.
                </CardDescription>
              </CardHeader>
            </Card>
          )}
        </div>
      </div>
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
