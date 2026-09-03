"use client";

import { useState } from "react";

import {
  useSeminarBuchungStornierenMutation,
  useSeminarTerminAbgehaltenMutation,
  useSeminarTerminAbsagenMutation,
  useSeminarUmsatzVorschauQuery,
  useYamsApiReady,
} from "@/api/hooks";
import type { SeminarTermin } from "@/api/types";
import { Alert } from "@/components/ui/alert";
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

type SeminarTerminPanelProps = {
  termin: SeminarTermin | null;
  onUpdated: (termin: SeminarTermin) => void;
};

export function SeminarTerminPanel({
  termin,
  onUpdated,
}: SeminarTerminPanelProps) {
  const { api } = useYamsApiReady();
  const umsatz = useSeminarUmsatzVorschauQuery(termin?.id);
  const abgehaltenMutation = useSeminarTerminAbgehaltenMutation();
  const absagenMutation = useSeminarTerminAbsagenMutation();
  const stornoMutation = useSeminarBuchungStornierenMutation();
  const [absagegrund, setAbsagegrund] = useState("Wetter");
  const [pdfError, setPdfError] = useState<string | null>(null);
  const [pdfLoading, setPdfLoading] = useState<string | null>(null);

  if (!termin) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Termin verwalten</CardTitle>
          <CardDescription>Kein Termin ausgewählt.</CardDescription>
        </CardHeader>
      </Card>
    );
  }

  const terminId = termin.id;

  async function markAbgehalten() {
    const updated = await abgehaltenMutation.mutateAsync(terminId);
    onUpdated(updated);
  }

  async function absagen() {
    const updated = await absagenMutation.mutateAsync({
      terminId,
      body: { grund: absagegrund },
    });
    onUpdated(updated);
  }

  async function stornieren(buchungId: string) {
    const updated = await stornoMutation.mutateAsync({
      terminId,
      buchungId,
    });
    onUpdated(updated);
  }

  async function downloadTeilnahmePdf(buchungId: string) {
    if (!api) return;
    setPdfError(null);
    setPdfLoading(buchungId);
    try {
      const blob = await api.teilnahmebestätigungPdf(terminId, buchungId);
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
        <CardTitle>Termin verwalten</CardTitle>
        <CardDescription>
          Status: {termin.status} · {termin.buchungen.length} Buchung(en)
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
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

        {termin.buchungen.length > 0 ? (
          <ul className="space-y-3">
            {termin.buchungen.map((buchung) => (
              <li
                key={buchung.id}
                className="rounded-lg border border-zinc-200 p-3 dark:border-zinc-800"
              >
                <p className="text-sm">
                  Buchung {buchung.id.slice(0, 8)}… · {buchung.status} · Rabatt{" "}
                  {buchung.rabatt}
                </p>
                <div className="mt-2 flex flex-wrap gap-2">
                  {buchung.status === "Bestätigt" && termin.status === "Geplant" ? (
                    <Button
                      type="button"
                      variant="secondary"
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
                      disabled={pdfLoading === buchung.id}
                      onClick={() => downloadTeilnahmePdf(buchung.id)}
                    >
                      {pdfLoading === buchung.id
                        ? "PDF…"
                        : "Teilnahme-PDF"}
                    </Button>
                  ) : null}
                </div>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-sm text-zinc-500">Noch keine Buchungen.</p>
        )}

        {termin.status === "Geplant" ? (
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
