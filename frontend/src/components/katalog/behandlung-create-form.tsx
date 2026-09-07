"use client";

import { type FormEvent, useState } from "react";

import { useBehandlungErstellenMutation } from "@/api/hooks";
import type { BehandlungErstellung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

const emptyBehandlung = (): BehandlungErstellung => ({
  name: "",
  beschreibung: "",
  standardpreis: "",
  mwst: "0.20",
});

export function BehandlungCreateForm() {
  const mutation = useBehandlungErstellenMutation();
  const [form, setForm] = useState<BehandlungErstellung>(emptyBehandlung);
  const [success, setSuccess] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSuccess(null);
    const created = await mutation.mutateAsync(form);
    setSuccess(`${created.name} angelegt.`);
    setForm(emptyBehandlung());
  }

  return (
    <form
      className="space-y-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-800"
      onSubmit={handleSubmit}
    >
      <h3 className="text-sm font-semibold">Neue Behandlung</h3>
      <div className="grid gap-4 sm:grid-cols-2">
        <Field label="Name">
          <Input
            value={form.name}
            onChange={(e) => setForm((b) => ({ ...b, name: e.target.value }))}
            required
          />
        </Field>
        <Field label="Beschreibung">
          <Input
            value={form.beschreibung}
            onChange={(e) =>
              setForm((b) => ({ ...b, beschreibung: e.target.value }))
            }
            required
          />
        </Field>
        <Field label="Standardpreis (netto)">
          <Input
            value={form.standardpreis}
            onChange={(e) =>
              setForm((b) => ({ ...b, standardpreis: e.target.value }))
            }
            required
          />
        </Field>
        <Field label="MwSt" hint="Anteil 0–1, z. B. 0.20">
          <Input
            value={form.mwst}
            onChange={(e) => setForm((b) => ({ ...b, mwst: e.target.value }))}
            required
          />
        </Field>
      </div>
      {success ? <Alert variant="success">{success}</Alert> : null}
      {mutation.error ? (
        <Alert variant="error">{String(mutation.error)}</Alert>
      ) : null}
      <Button type="submit" disabled={mutation.isPending}>
        {mutation.isPending ? "Speichern…" : "Behandlung anlegen"}
      </Button>
    </form>
  );
}
