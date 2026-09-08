"use client";

import { type FormEvent, useEffect, useState } from "react";

import {
  useAlleSeminareQuery,
  useSeminarTerminPlanenMutation,
} from "@/api/hooks";
import type { SeminarTerminErstellung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import {
  datetimeLocalForDate,
  datetimeLocalToIso,
  defaultDatetimeLocal,
} from "@/lib/dates";

type TerminPlanFormProps = {
  onPlanned?: () => void;
  prefillDate?: string | null;
};

export function TerminPlanForm({
  onPlanned,
  prefillDate,
}: TerminPlanFormProps) {
  const seminareQuery = useAlleSeminareQuery();
  const mutation = useSeminarTerminPlanenMutation();
  const seminare = seminareQuery.data ?? [];

  const [seminarId, setSeminarId] = useState("");
  const [beginnLocal, setBeginnLocal] = useState(defaultDatetimeLocal(10));
  const [endeLocal, setEndeLocal] = useState(defaultDatetimeLocal(16));
  const [ortName, setOrtName] = useState("");
  const [maxTeilnehmer, setMaxTeilnehmer] = useState("8");
  const [success, setSuccess] = useState<string | null>(null);

  useEffect(() => {
    if (seminare.length > 0 && !seminarId) {
      setSeminarId(seminare[0]?.id ?? "");
    }
  }, [seminare, seminarId]);

  useEffect(() => {
    if (!prefillDate) return;
    setBeginnLocal(datetimeLocalForDate(prefillDate, 10));
    setEndeLocal(datetimeLocalForDate(prefillDate, 16));
  }, [prefillDate]);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!seminarId) return;
    setSuccess(null);

    const body: SeminarTerminErstellung = {
      seminarId,
      beginn: datetimeLocalToIso(beginnLocal),
      ende: datetimeLocalToIso(endeLocal),
      ort: { ortName: ortName || undefined, adresse: undefined },
      maxTeilnehmer: maxTeilnehmer ? Number(maxTeilnehmer) : undefined,
    };

    const termin = await mutation.mutateAsync(body);
    setSuccess(
      `Termin geplant (${new Date(termin.beginn).toLocaleString("de-AT")}).`,
    );
    onPlanned?.();
  }

  return (
    <form
      className="space-y-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-800"
      onSubmit={handleSubmit}
    >
      <h3 className="text-sm font-semibold">Termin planen</h3>

      {seminare.length === 0 ? (
        <Alert variant="info">Zuerst ein Seminar im Katalog anlegen.</Alert>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2">
          <Field label="Seminar" className="sm:col-span-2">
            <Select
              value={seminarId}
              onChange={(e) => setSeminarId(e.target.value)}
              required
            >
              {seminare.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.titel}
                </option>
              ))}
            </Select>
          </Field>
          <Field label="Beginn">
            <Input
              type="datetime-local"
              value={beginnLocal}
              onChange={(e) => setBeginnLocal(e.target.value)}
              required
            />
          </Field>
          <Field label="Ende">
            <Input
              type="datetime-local"
              value={endeLocal}
              onChange={(e) => setEndeLocal(e.target.value)}
              required
            />
          </Field>
          <Field label="Ort">
            <Input
              value={ortName}
              onChange={(e) => setOrtName(e.target.value)}
              placeholder="Seminarhof"
            />
          </Field>
          <Field label="Max. Teilnehmer">
            <Input
              type="number"
              min={1}
              value={maxTeilnehmer}
              onChange={(e) => setMaxTeilnehmer(e.target.value)}
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
        disabled={seminare.length === 0 || !seminarId || mutation.isPending}
      >
        {mutation.isPending ? "Speichern…" : "Termin planen"}
      </Button>
    </form>
  );
}
