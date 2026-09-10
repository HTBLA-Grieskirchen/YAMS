"use client";

import { type FormEvent, useState } from "react";

import { useSeminarErstellenMutation } from "@/api/hooks";
import type { Seminar, SeminarErstellung } from "@/api/types";
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

type SeminarFormProps = {
  seminar: Seminar | null;
  onCreated: (seminar: Seminar) => void;
};

const defaultSeminar: SeminarErstellung = {
  titel: "Hufseminar",
  beschreibung: "Einführung in die Hufpflege",
  teilnahmegebührBasis: "100.00",
  mwst: "0.20",
};

export function SeminarForm({ seminar, onCreated }: SeminarFormProps) {
  const mutation = useSeminarErstellenMutation();
  const [form, setForm] = useState<SeminarErstellung>(defaultSeminar);

  function updateField<K extends keyof SeminarErstellung>(
    key: K,
    value: SeminarErstellung[K],
  ) {
    setForm((prev) => ({ ...prev, [key]: value }));
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const created = await mutation.mutateAsync(form);
    onCreated(created);
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Seminar anlegen</CardTitle>
        <CardDescription>
          Legt ein neues Seminar mit Teilnahmegebühr und MwSt an.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {seminar ? (
          <Alert variant="success">
            Seminar erstellt: {seminar.titel} ({seminar.teilnahmegebührBasis} €
            netto, MwSt {seminar.mwst})
          </Alert>
        ) : (
          <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="grid gap-4 sm:grid-cols-2">
              <Field label="Titel">
                <Input
                  value={form.titel}
                  onChange={(e) => updateField("titel", e.target.value)}
                  required
                />
              </Field>
              <Field label="Teilnahmegebühr (netto)">
                <Input
                  value={form.teilnahmegebührBasis}
                  onChange={(e) =>
                    updateField("teilnahmegebührBasis", e.target.value)
                  }
                  required
                />
              </Field>
              <Field label="MwSt (0–1)">
                <Input
                  value={form.mwst}
                  onChange={(e) => updateField("mwst", e.target.value)}
                  required
                />
              </Field>
              <Field label="Beschreibung" className="sm:col-span-2">
                <Input
                  value={form.beschreibung}
                  onChange={(e) => updateField("beschreibung", e.target.value)}
                  required
                />
              </Field>
            </div>

            {mutation.error ? (
              <Alert variant="error">{String(mutation.error)}</Alert>
            ) : null}

            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending ? "Speichern…" : "Seminar erstellen"}
            </Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
