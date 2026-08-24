"use client";

import { type FormEvent, useState } from "react";

import {
  useLeistungAusBehandlungBuchenMutation,
  useLeistungAusProduktBuchenMutation,
} from "@/api/hooks";
import type {
  Behandlung,
  Haustier,
  Klient,
  Leistung,
  Produkt,
} from "@/api/types";
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
import { Select } from "@/components/ui/select";
import { todayIsoDate } from "@/lib/dates";

type LeistungFormProps = {
  klient: Klient | null;
  haustier: Haustier | null;
  produkt: Produkt | null;
  behandlung: Behandlung | null;
  lastLeistung: Leistung | null;
  onBooked: (leistung: Leistung) => void;
};

type LeistungQuelle = "produkt" | "behandlung";

export function LeistungForm({
  klient,
  haustier,
  produkt,
  behandlung,
  lastLeistung,
  onBooked,
}: LeistungFormProps) {
  const produktMutation = useLeistungAusProduktBuchenMutation();
  const behandlungMutation = useLeistungAusBehandlungBuchenMutation();

  const canBookProdukt = klient && produkt;
  const canBookBehandlung = klient && behandlung;
  const defaultQuelle: LeistungQuelle =
    canBookProdukt ? "produkt" : canBookBehandlung ? "behandlung" : "produkt";

  const [quelle, setQuelle] = useState<LeistungQuelle>(defaultQuelle);
  const [leistungsdatum, setLeistungsdatum] = useState(todayIsoDate());
  const [menge, setMenge] = useState("1");

  const mutation = quelle === "produkt" ? produktMutation : behandlungMutation;
  const isReady = quelle === "produkt" ? canBookProdukt : canBookBehandlung;

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!klient) return;

    if (quelle === "produkt" && produkt) {
      const leistung = await produktMutation.mutateAsync({
        klientId: klient.id,
        produktId: produkt.id,
        haustierId: haustier?.id,
        menge,
        leistungsdatum,
      });
      onBooked(leistung);
      return;
    }

    if (quelle === "behandlung" && behandlung) {
      const leistung = await behandlungMutation.mutateAsync({
        klientId: klient.id,
        behandlungId: behandlung.id,
        haustierId: haustier?.id,
        leistungsdatum,
      });
      onBooked(leistung);
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>4. Leistung buchen</CardTitle>
        <CardDescription>
          Bucht eine offene Leistung für den Klienten aus dem Katalog.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {!klient ? (
          <p className="text-sm text-zinc-500">
            Lege zuerst einen Klienten an.
          </p>
        ) : lastLeistung ? (
          <Alert variant="success">
            Leistung gebucht: {lastLeistung.beschreibung} — {lastLeistung.betrag}{" "}
            € (Status: {lastLeistung.status})
          </Alert>
        ) : (
          <form className="space-y-4" onSubmit={handleSubmit}>
            <Field label="Quelle">
              <Select
                value={quelle}
                onChange={(e) => setQuelle(e.target.value as LeistungQuelle)}
              >
                <option value="produkt" disabled={!canBookProdukt}>
                  Produkt
                </option>
                <option value="behandlung" disabled={!canBookBehandlung}>
                  Behandlung
                </option>
              </Select>
            </Field>

            <div className="grid gap-4 sm:grid-cols-2">
              <Field label="Leistungsdatum">
                <Input
                  type="date"
                  value={leistungsdatum}
                  onChange={(e) => setLeistungsdatum(e.target.value)}
                  required
                />
              </Field>
              {quelle === "produkt" ? (
                <Field label="Menge">
                  <Input
                    value={menge}
                    onChange={(e) => setMenge(e.target.value)}
                    required
                  />
                </Field>
              ) : null}
            </div>

            {haustier ? (
              <p className="text-sm text-zinc-600 dark:text-zinc-400">
                Verknüpft mit Haustier: {haustier.name}
              </p>
            ) : null}

            {!isReady ? (
              <Alert variant="info">
                Erstelle zuerst ein {quelle === "produkt" ? "Produkt" : "Behandlung"}{" "}
                im Katalog.
              </Alert>
            ) : null}

            {mutation.error ? (
              <Alert variant="error">{String(mutation.error)}</Alert>
            ) : null}

            <Button type="submit" disabled={!isReady || mutation.isPending}>
              {mutation.isPending ? "Buchen…" : "Leistung buchen"}
            </Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
