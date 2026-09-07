"use client";

import { type FormEvent, useMemo, useState } from "react";

import { LeistungStatus } from "@/api/schema";
import {
  useAlleKlientenQuery,
  useAlleLeistungenQuery,
  useTagesabschlussDurchführenMutation,
} from "@/api/hooks";
import type { Rechnung } from "@/api/types";
import { RechnungenTable } from "@/components/rechnung/rechnungen-table";
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
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { formatDate } from "@/lib/format";
import { todayIsoDate } from "@/lib/dates";

export function AbrechnungPanel() {
  const leistungenQuery = useAlleLeistungenQuery();
  const klientenQuery = useAlleKlientenQuery();
  const mutation = useTagesabschlussDurchführenMutation();
  const [abschlussdatum, setAbschlussdatum] = useState(todayIsoDate());
  const [resultRechnungen, setResultRechnungen] = useState<Rechnung[] | null>(
    null,
  );

  const klientenById = useMemo(() => {
    return new Map((klientenQuery.data ?? []).map((klient) => [klient.id, klient]));
  }, [klientenQuery.data]);

  const offeneLeistungen = useMemo(() => {
    return (leistungenQuery.data ?? []).filter(
      (leistung) => leistung.status === LeistungStatus.Offen,
    );
  }, [leistungenQuery.data]);

  const isLoading = leistungenQuery.isPending || klientenQuery.isPending;
  const error = leistungenQuery.error ?? klientenQuery.error;

  async function handleTagesabschluss(event: FormEvent) {
    event.preventDefault();
    const rechnungen = await mutation.mutateAsync({ abschlussdatum });
    setResultRechnungen(rechnungen);
  }

  return (
    <div className="space-y-6 p-6">
      <div className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">Abrechnung</h1>
        <p className="text-sm text-zinc-600">
          Offene Leistungen prüfen und Tagesabschluss durchführen.
        </p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Offene Leistungen</CardTitle>
          <CardDescription>
            {offeneLeistungen.length} Leistung(en) warten auf Abrechnung.
          </CardDescription>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <p className="text-sm text-zinc-500">Lade Leistungen…</p>
          ) : null}
          {error ? <Alert variant="error">{String(error)}</Alert> : null}

          {!isLoading && !error ? (
            offeneLeistungen.length === 0 ? (
              <Alert variant="info">Keine offenen Leistungen.</Alert>
            ) : (
              <Table>
                <TableHead>
                  <TableRow>
                    <TableHeader>Datum</TableHeader>
                    <TableHeader>Beschreibung</TableHeader>
                    <TableHeader>Klient</TableHeader>
                    <TableHeader>Betrag</TableHeader>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {offeneLeistungen.map((leistung) => {
                    const klient = klientenById.get(leistung.klientId);
                    return (
                      <TableRow key={leistung.id}>
                        <TableCell>{formatDate(leistung.leistungsdatum)}</TableCell>
                        <TableCell className="font-medium">
                          {leistung.beschreibung}
                        </TableCell>
                        <TableCell>
                          {klient
                            ? `${klient.vorname} ${klient.nachname}`
                            : leistung.klientId.slice(0, 8)}
                        </TableCell>
                        <TableCell>{leistung.betrag} €</TableCell>
                      </TableRow>
                    );
                  })}
                </TableBody>
              </Table>
            )
          ) : null}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Tagesabschluss</CardTitle>
          <CardDescription>
            Erzeugt Rechnungen für alle offenen Leistungen.
          </CardDescription>
        </CardHeader>
        <CardContent>
          {resultRechnungen ? (
            <div className="space-y-4">
              <Alert variant="success">
                Tagesabschluss abgeschlossen — {resultRechnungen.length}{" "}
                Rechnung(en) erstellt.
              </Alert>
              <RechnungenTable
                rechnungen={resultRechnungen}
                klientenById={klientenById}
                showKlient
              />
            </div>
          ) : (
            <form className="space-y-4" onSubmit={handleTagesabschluss}>
              <Field label="Abschlussdatum">
                <Input
                  type="date"
                  value={abschlussdatum}
                  onChange={(event) => setAbschlussdatum(event.target.value)}
                  required
                />
              </Field>

              {mutation.error ? (
                <Alert variant="error">{String(mutation.error)}</Alert>
              ) : null}

              <Button
                type="submit"
                disabled={
                  mutation.isPending || offeneLeistungen.length === 0 || isLoading
                }
              >
                {mutation.isPending
                  ? "Abschließen…"
                  : "Tagesabschluss durchführen"}
              </Button>
            </form>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
