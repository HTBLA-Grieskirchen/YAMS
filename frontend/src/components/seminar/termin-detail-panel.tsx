"use client";

import { useMemo, useState } from "react";

import {
  useAlleKlientenQuery,
  useSeminarBuchungStornierenMutation,
  useSeminarTerminAbgehaltenMutation,
  useSeminarTerminAbsagenMutation,
  useSeminarUmsatzVorschauQuery,
  useYamsApiReady,
} from "@/api/hooks";
import { SeminarBuchungStatus, SeminarTerminStatus } from "@/api/schema";
import type { Seminar, SeminarTermin } from "@/api/types";
import { TerminBuchungForm } from "@/components/seminar/termin-buchung-form";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { downloadBlob } from "@/lib/dates";

type TerminDetailPanelProps = {
  termin: SeminarTermin;
  seminar?: Seminar;
  onUpdated: (termin: SeminarTermin) => void;
};

export function TerminDetailPanel({
  termin,
  seminar,
  onUpdated,
}: TerminDetailPanelProps) {
  const { api } = useYamsApiReady();
  const klientenQuery = useAlleKlientenQuery();
  const umsatz = useSeminarUmsatzVorschauQuery(termin.id);
  const abgehaltenMutation = useSeminarTerminAbgehaltenMutation();
  const absagenMutation = useSeminarTerminAbsagenMutation();
  const stornoMutation = useSeminarBuchungStornierenMutation();
  const [absagegrund, setAbsagegrund] = useState("Wetter");
  const [pdfError, setPdfError] = useState<string | null>(null);
  const [pdfLoading, setPdfLoading] = useState<string | null>(null);

  const klientenById = useMemo(
    () =>
      new Map((klientenQuery.data ?? []).map((klient) => [klient.id, klient])),
    [klientenQuery.data],
  );

  async function markAbgehalten() {
    const updated = await abgehaltenMutation.mutateAsync(termin.id);
    onUpdated(updated);
  }

  async function absagen() {
    const updated = await absagenMutation.mutateAsync({
      terminId: termin.id,
      body: { grund: absagegrund },
    });
    onUpdated(updated);
  }

  async function stornieren(buchungId: string) {
    const updated = await stornoMutation.mutateAsync({
      terminId: termin.id,
      buchungId,
    });
    onUpdated(updated);
  }

  async function downloadTeilnahmePdf(buchungId: string) {
    if (!api) return;
    setPdfError(null);
    setPdfLoading(buchungId);
    try {
      const blob = await api.teilnahmebestätigungPdf(termin.id, buchungId);
      downloadBlob(blob, `teilnahme-${buchungId}.pdf`);
    } catch (error) {
      setPdfError(String(error));
    } finally {
      setPdfLoading(null);
    }
  }

  const mutationError =
    abgehaltenMutation.error ??
    absagenMutation.error ??
    stornoMutation.error;

  return (
    <Card>
      <CardHeader>
        <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
          <div>
            <CardTitle>{seminar?.titel ?? "Seminar-Termin"}</CardTitle>
            <CardDescription>
              {new Date(termin.beginn).toLocaleString("de-AT")} –{" "}
              {new Date(termin.ende).toLocaleString("de-AT")}
            </CardDescription>
          </div>
          <Badge variant={statusBadgeVariant(termin.status)}>
            {termin.status}
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-6">
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          {termin.ort.ortName ?? "Ort offen"}
          {termin.maxTeilnehmer
            ? ` · max. ${termin.maxTeilnehmer} Teilnehmer`
            : ""}
          · {termin.buchungen.length} Buchung(en)
        </p>

        {umsatz.data ? (
          <dl className="grid gap-2 text-sm sm:grid-cols-2">
            <div>
              <dt className="text-zinc-500">Umsatz netto</dt>
              <dd className="font-mono">{umsatz.data.gesamtNetto} €</dd>
            </div>
            <div>
              <dt className="text-zinc-500">Umsatz brutto</dt>
              <dd className="font-mono">{umsatz.data.gesamtBrutto} €</dd>
            </div>
          </dl>
        ) : umsatz.isPending ? (
          <p className="text-sm text-zinc-500">Lade Umsatz…</p>
        ) : null}

        <section>
          <h3 className="mb-3 text-sm font-semibold">Teilnehmer</h3>
          {termin.buchungen.length === 0 ? (
            <p className="text-sm text-zinc-500">Noch keine Buchungen.</p>
          ) : (
            <ul className="space-y-2">
              {termin.buchungen.map((buchung) => {
                const klient = klientenById.get(buchung.klientId);
                return (
                  <li
                    key={buchung.id}
                    className="flex flex-col gap-2 rounded-lg border border-zinc-200 p-3 sm:flex-row sm:items-center sm:justify-between dark:border-zinc-800"
                  >
                    <div>
                      <p className="font-medium text-sm">
                        {klient
                          ? `${klient.vorname} ${klient.nachname}`
                          : buchung.klientId.slice(0, 8)}
                      </p>
                      <p className="text-xs text-zinc-500">
                        {buchung.status} · Rabatt {buchung.rabatt}
                      </p>
                    </div>
                    <div className="flex flex-wrap gap-2">
                      {buchung.status === SeminarBuchungStatus.Best_tigt &&
                      termin.status === SeminarTerminStatus.Geplant ? (
                        <Button
                          type="button"
                          variant="secondary"
                          size="sm"
                          disabled={stornoMutation.isPending}
                          onClick={() => stornieren(buchung.id)}
                        >
                          Stornieren
                        </Button>
                      ) : null}
                      {buchung.leistungId ? (
                        <Button
                          type="button"
                          variant="secondary"
                          size="sm"
                          disabled={pdfLoading === buchung.id}
                          onClick={() => downloadTeilnahmePdf(buchung.id)}
                        >
                          {pdfLoading === buchung.id ? "PDF…" : "Teilnahme-PDF"}
                        </Button>
                      ) : null}
                    </div>
                  </li>
                );
              })}
            </ul>
          )}
        </section>

        <TerminBuchungForm termin={termin} onBooked={onUpdated} />

        {termin.status === SeminarTerminStatus.Geplant ? (
          <div className="space-y-3 border-t border-zinc-200 pt-4 dark:border-zinc-800">
            <Button
              type="button"
              disabled={abgehaltenMutation.isPending}
              onClick={markAbgehalten}
            >
              {abgehaltenMutation.isPending
                ? "Markiere…"
                : "Als abgehalten markieren"}
            </Button>
            <div className="flex flex-wrap items-end gap-3">
              <Field label="Absagegrund" className="min-w-48 flex-1">
                <Input
                  value={absagegrund}
                  onChange={(e) => setAbsagegrund(e.target.value)}
                />
              </Field>
              <Button
                type="button"
                variant="secondary"
                disabled={absagenMutation.isPending}
                onClick={absagen}
              >
                Termin absagen
              </Button>
            </div>
          </div>
        ) : null}

        {mutationError ? (
          <Alert variant="error">{String(mutationError)}</Alert>
        ) : null}
        {pdfError ? <Alert variant="error">{pdfError}</Alert> : null}
      </CardContent>
    </Card>
  );
}

function statusBadgeVariant(
  status: SeminarTerminStatus,
): "default" | "success" | "error" {
  if (status === SeminarTerminStatus.Abgesagt) return "error";
  if (status === SeminarTerminStatus.Abgehalten) return "success";
  return "default";
}
