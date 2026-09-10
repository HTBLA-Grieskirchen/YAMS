"use client";

import { type FormEvent, useState } from "react";

import { useSeminarErstellenMutation } from "@/api/hooks";
import type { SeminarErstellung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

const emptySeminar = (): SeminarErstellung => ({
  titel: "",
  beschreibung: "",
  teilnahmegebührBasis: "",
  mwst: "0.20",
});

export function SeminarCreateForm() {
  const mutation = useSeminarErstellenMutation();
  const [form, setForm] = useState<SeminarErstellung>(emptySeminar);
  const [success, setSuccess] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSuccess(null);
    const created = await mutation.mutateAsync(form);
    setSuccess(`Seminar „${created.titel}“ angelegt.`);
    setForm(emptySeminar());
  }

  return (
    <form
      className="space-y-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-800"
      onSubmit={handleSubmit}
    >
      <h3 className="text-sm font-semibold">Neues Seminar</h3>
      <p className="text-xs text-zinc-500">
        Stammdaten — Termine werden unter Seminar verwaltet.
      </p>
      <div className="grid gap-4 sm:grid-cols-2">
        <Field label="Titel">
          <Input
            value={form.titel}
            onChange={(e) => setForm((s) => ({ ...s, titel: e.target.value }))}
            required
          />
        </Field>
        <Field label="Teilnahmegebühr (netto)">
          <Input
            value={form.teilnahmegebührBasis}
            onChange={(e) =>
              setForm((s) => ({ ...s, teilnahmegebührBasis: e.target.value }))
            }
            required
          />
        </Field>
        <Field label="MwSt" hint="Anteil 0–1, z. B. 0.20">
          <Input
            value={form.mwst}
            onChange={(e) => setForm((s) => ({ ...s, mwst: e.target.value }))}
            required
          />
        </Field>
        <Field label="Beschreibung" className="sm:col-span-2">
          <Input
            value={form.beschreibung}
            onChange={(e) =>
              setForm((s) => ({ ...s, beschreibung: e.target.value }))
            }
            required
          />
        </Field>
      </div>
      {success ? <Alert variant="success">{success}</Alert> : null}
      {mutation.error ? (
        <Alert variant="error">{String(mutation.error)}</Alert>
      ) : null}
      <Button type="submit" disabled={mutation.isPending}>
        {mutation.isPending ? "Speichern…" : "Seminar anlegen"}
      </Button>
    </form>
  );
}
