"use client";

import { type FormEvent, useEffect, useMemo, useState } from "react";

import {
  useAlleKlientenQuery,
  useSeminarBuchungAnlegenMutation,
} from "@/api/hooks";
import { SeminarTerminStatus } from "@/api/schema";
import type { SeminarTermin } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";

type TerminBuchungFormProps = {
  termin: SeminarTermin;
  onBooked: (termin: SeminarTermin) => void;
};

export function TerminBuchungForm({
  termin,
  onBooked,
}: TerminBuchungFormProps) {
  const klientenQuery = useAlleKlientenQuery();
  const mutation = useSeminarBuchungAnlegenMutation();
  const klienten = klientenQuery.data ?? [];

  const [klientId, setKlientId] = useState("");
  const [rabatt, setRabatt] = useState("0");
  const [success, setSuccess] = useState<string | null>(null);

  const bookedKlientIds = useMemo(
    () => new Set(termin.buchungen.map((b) => b.klientId)),
    [termin.buchungen],
  );

  const availableKlienten = klienten.filter((k) => !bookedKlientIds.has(k.id));

  useEffect(() => {
    if (
      availableKlienten.length > 0 &&
      !availableKlienten.some((k) => k.id === klientId)
    ) {
      setKlientId(availableKlienten[0]?.id ?? "");
    }
  }, [availableKlienten, klientId]);

  if (termin.status !== SeminarTerminStatus.Geplant) {
    return (
      <p className="text-sm text-zinc-500">
        Termin ist {termin.status} — keine neuen Buchungen möglich.
      </p>
    );
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!klientId) return;
    setSuccess(null);
    const updated = await mutation.mutateAsync({
      terminId: termin.id,
      body: { klientId, rabatt },
    });
    const klient = klienten.find((k) => k.id === klientId);
    setSuccess(
      klient
        ? `${klient.vorname} ${klient.nachname} gebucht.`
        : "Teilnehmer gebucht.",
    );
    onBooked(updated);
  }

  return (
    <form
      className="space-y-4 border-t border-zinc-200 pt-4 dark:border-zinc-800"
      onSubmit={handleSubmit}
    >
      <h3 className="text-sm font-semibold">Teilnehmer buchen</h3>

      {availableKlienten.length === 0 ? (
        <Alert variant="info">
          Alle Klienten sind bereits gebucht oder es gibt keine Klienten.
        </Alert>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2">
          <Field label="Klient">
            <Select
              value={klientId}
              onChange={(e) => setKlientId(e.target.value)}
              required
            >
              {availableKlienten.map((k) => (
                <option key={k.id} value={k.id}>
                  {k.nachname}, {k.vorname}
                </option>
              ))}
            </Select>
          </Field>
          <Field label="Rabatt" hint="Anteil 0–1, z. B. 0.20">
            <Input
              value={rabatt}
              onChange={(e) => setRabatt(e.target.value)}
              required
            />
          </Field>
        </div>
      )}

      {success ? <Alert variant="success">{success}</Alert> : null}
      {mutation.error ? (
        <Alert variant="error">{String(mutation.error)}</Alert>
      ) : null}

      <Button
        type="submit"
        disabled={
          availableKlienten.length === 0 || !klientId || mutation.isPending
        }
      >
        {mutation.isPending ? "Buchen…" : "Teilnehmer buchen"}
      </Button>
    </form>
  );
}
