"use client";

import { type FormEvent, useState } from "react";

import { useHaustierErstellenMutation } from "@/api/hooks";
import type { HaustierErstellung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

const emptyHaustier = (klientId: string): HaustierErstellung => ({
  klientId,
  name: "",
  geburtstag: "",
  tierart: "",
  beschreibung: "",
});

type HaustierCreateFormProps = {
  klientId: string;
};

export function HaustierCreateForm({ klientId }: HaustierCreateFormProps) {
  const mutation = useHaustierErstellenMutation();
  const [form, setForm] = useState<HaustierErstellung>(() =>
    emptyHaustier(klientId),
  );
  const [success, setSuccess] = useState<string | null>(null);

  function updateField<K extends keyof HaustierErstellung>(
    key: K,
    value: HaustierErstellung[K],
  ) {
    setForm((prev) => ({ ...prev, [key]: value }));
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSuccess(null);
    const created = await mutation.mutateAsync(form);
    setSuccess(`${created.name} (${created.tierart}) registriert.`);
    setForm(emptyHaustier(klientId));
  }

  return (
    <form className="space-y-4 border-t border-zinc-200 pt-4 dark:border-zinc-800" onSubmit={handleSubmit}>
      <h3 className="text-sm font-semibold">Haustier hinzufügen</h3>
      <div className="grid gap-4 sm:grid-cols-2">
        <Field label="Name">
          <Input
            value={form.name}
            onChange={(e) => updateField("name", e.target.value)}
            required
          />
        </Field>
        <Field label="Tierart">
          <Input
            value={form.tierart}
            onChange={(e) => updateField("tierart", e.target.value)}
            required
          />
        </Field>
        <Field label="Geburtstag">
          <Input
            type="date"
            value={form.geburtstag}
            onChange={(e) => updateField("geburtstag", e.target.value)}
            required
          />
        </Field>
        <Field label="Beschreibung">
          <Input
            value={form.beschreibung}
            onChange={(e) => updateField("beschreibung", e.target.value)}
            required
          />
        </Field>
      </div>
      {success ? <Alert variant="success">{success}</Alert> : null}
      {mutation.error ? (
        <Alert variant="error">{String(mutation.error)}</Alert>
      ) : null}
      <Button type="submit" disabled={mutation.isPending}>
        {mutation.isPending ? "Speichern…" : "Haustier anlegen"}
      </Button>
    </form>
  );
}
