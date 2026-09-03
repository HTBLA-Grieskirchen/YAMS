"use client";

import { type FormEvent, useEffect, useState } from "react";

import { useSeminarTerminPlanenMutation } from "@/api/hooks";
import type { Seminar, SeminarTermin, SeminarTerminErstellung } from "@/api/types";
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
import { datetimeLocalToIso, defaultDatetimeLocal } from "@/lib/dates";

type SeminarTerminFormProps = {
  seminar: Seminar | null;
  termin: SeminarTermin | null;
  onCreated: (termin: SeminarTermin) => void;
};

function defaultTermin(seminarId: string): SeminarTerminErstellung {
  return {
    seminarId,
    beginn: datetimeLocalToIso(defaultDatetimeLocal(10)),
    ende: datetimeLocalToIso(defaultDatetimeLocal(16)),
    ort: { ortName: "Hof", adresse: undefined },
    maxTeilnehmer: 8,
  };
}

export function SeminarTerminForm({
  seminar,
  termin,
  onCreated,
}: SeminarTerminFormProps) {
  const mutation = useSeminarTerminPlanenMutation();
  const [form, setForm] = useState<SeminarTerminErstellung | null>(null);
  const [beginnLocal, setBeginnLocal] = useState(defaultDatetimeLocal(10));
  const [endeLocal, setEndeLocal] = useState(defaultDatetimeLocal(16));

  useEffect(() => {
    if (seminar && !termin) {
      setForm(defaultTermin(seminar.id));
    }
  }, [seminar, termin]);

  if (!seminar) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Termin planen</CardTitle>
          <CardDescription>Lege zuerst ein Seminar an.</CardDescription>
        </CardHeader>
      </Card>
    );
  }

  if (!form) {
    return null;
  }

  function updateOrt(ortName: string) {
    setForm((prev) =>
      prev ? { ...prev, ort: { ...prev.ort, ortName } } : prev,
    );
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!form) return;
    const payload: SeminarTerminErstellung = {
      seminarId: form.seminarId,
      beginn: datetimeLocalToIso(beginnLocal),
      ende: datetimeLocalToIso(endeLocal),
      ort: form.ort,
      maxTeilnehmer: form.maxTeilnehmer,
    };
    const created = await mutation.mutateAsync(payload);
    onCreated(created);
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Termin planen</CardTitle>
        <CardDescription>
          Plant einen Termin für „{seminar.titel}“.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {termin ? (
          <Alert variant="success">
            Termin geplant: {termin.status} · {termin.beginn} – {termin.ende}
          </Alert>
        ) : (
          <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="grid gap-4 sm:grid-cols-2">
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
                  value={form.ort.ortName ?? ""}
                  onChange={(e) => updateOrt(e.target.value)}
                />
              </Field>
              <Field label="Max. Teilnehmer">
                <Input
                  type="number"
                  min={1}
                  value={form.maxTeilnehmer ?? ""}
                  onChange={(e) =>
                    setForm((prev) =>
                      prev
                        ? {
                            ...prev,
                            maxTeilnehmer: Number(e.target.value) || undefined,
                          }
                        : prev,
                    )
                  }
                />
              </Field>
            </div>

            {mutation.error ? (
              <Alert variant="error">{String(mutation.error)}</Alert>
            ) : null}

            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending ? "Speichern…" : "Termin planen"}
            </Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
