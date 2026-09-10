"use client";

import { type FormEvent, useState } from "react";

import { useProduktErstellenMutation } from "@/api/hooks";
import type { ProduktErstellung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

const emptyProdukt = (): ProduktErstellung => ({
  name: "",
  beschreibung: "",
  einzelpreis: "",
  mwst: "0.20",
});

export function ProduktCreateForm() {
  const mutation = useProduktErstellenMutation();
  const [form, setForm] = useState<ProduktErstellung>(emptyProdukt);
  const [success, setSuccess] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSuccess(null);
    const created = await mutation.mutateAsync(form);
    setSuccess(`${created.name} angelegt.`);
    setForm(emptyProdukt());
  }

  return (
    <form
      className="space-y-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-800"
      onSubmit={handleSubmit}
    >
      <h3 className="text-sm font-semibold">Neues Produkt</h3>
      <div className="grid gap-4 sm:grid-cols-2">
        <Field label="Name">
          <Input
            value={form.name}
            onChange={(e) => setForm((p) => ({ ...p, name: e.target.value }))}
            required
          />
        </Field>
        <Field label="Beschreibung">
          <Input
            value={form.beschreibung}
            onChange={(e) =>
              setForm((p) => ({ ...p, beschreibung: e.target.value }))
            }
            required
          />
        </Field>
        <Field label="Einzelpreis (netto)" hint="z. B. 24.99">
          <Input
            value={form.einzelpreis}
            onChange={(e) =>
              setForm((p) => ({ ...p, einzelpreis: e.target.value }))
            }
            required
          />
        </Field>
        <Field label="MwSt" hint="Anteil 0–1, z. B. 0.20">
          <Input
            value={form.mwst}
            onChange={(e) => setForm((p) => ({ ...p, mwst: e.target.value }))}
            required
          />
        </Field>
      </div>
      {success ? <Alert variant="success">{success}</Alert> : null}
      {mutation.error ? (
        <Alert variant="error">{String(mutation.error)}</Alert>
      ) : null}
      <Button type="submit" disabled={mutation.isPending}>
        {mutation.isPending ? "Speichern…" : "Produkt anlegen"}
      </Button>
    </form>
  );
}
