"use client";

import { useRouter } from "next/navigation";
import { type FormEvent, useState } from "react";

import { useKlientErstellenMutation, useYamsApiReady } from "@/api/hooks";
import type { Klient, KlientErstellung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { paths } from "@/lib/navigation";

function defaultKlient(): KlientErstellung {
  return {
    vorname: "",
    nachname: "",
    geburtstag: "",
    email: "",
    mobilnummer: "",
    kundennummer: Date.now(),
    einwilligung: false,
    adresse: {
      postleitzahl: "",
      stadt: "",
      straßeUndHausnummer: "",
      ländercode: "AT",
    },
  };
}

type KlientRegisterFormProps = {
  onCreated?: (klient: Klient) => void;
  redirectOnSuccess?: boolean;
};

export function KlientRegisterForm({
  onCreated,
  redirectOnSuccess = true,
}: KlientRegisterFormProps) {
  const router = useRouter();
  const { isReady, error: apiError } = useYamsApiReady();
  const mutation = useKlientErstellenMutation();
  const [form, setForm] = useState<KlientErstellung>(defaultKlient);

  function updateField<K extends keyof KlientErstellung>(
    key: K,
    value: KlientErstellung[K],
  ) {
    setForm((prev) => ({ ...prev, [key]: value }));
  }

  function updateAdresse(
    key: keyof KlientErstellung["adresse"],
    value: string,
  ) {
    setForm((prev) => ({
      ...prev,
      adresse: { ...prev.adresse, [key]: value },
    }));
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const created = await mutation.mutateAsync(form);
    onCreated?.(created);
    if (redirectOnSuccess) {
      router.push(paths.klient(created.id));
    }
  }

  return (
    <form className="space-y-6" onSubmit={handleSubmit}>
      <div className="grid gap-4 md:grid-cols-3">
        <Field label="Vorname">
          <Input
            value={form.vorname}
            onChange={(e) => updateField("vorname", e.target.value)}
            placeholder="Maria"
            required
          />
        </Field>
        <Field label="Nachname">
          <Input
            value={form.nachname}
            onChange={(e) => updateField("nachname", e.target.value)}
            placeholder="Muster"
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
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <Field label="Mobilnummer">
          <Input
            type="tel"
            value={form.mobilnummer}
            onChange={(e) => updateField("mobilnummer", e.target.value)}
            placeholder="+43 699 12345678"
            required
          />
        </Field>
        <Field label="E-Mail">
          <Input
            type="email"
            value={form.email}
            onChange={(e) => updateField("email", e.target.value)}
            placeholder="maria.muster@example.com"
            required
          />
        </Field>
        <Field label="Kundennummer">
          <Input
            type="number"
            min={1}
            value={form.kundennummer}
            onChange={(e) =>
              updateField("kundennummer", Number(e.target.value))
            }
            required
          />
        </Field>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        <Field label="Straße und Hausnummer">
          <Input
            value={form.adresse.straßeUndHausnummer}
            onChange={(e) =>
              updateAdresse("straßeUndHausnummer", e.target.value)
            }
            required
          />
        </Field>
        <Field label="Postleitzahl">
          <Input
            value={form.adresse.postleitzahl}
            onChange={(e) => updateAdresse("postleitzahl", e.target.value)}
            required
          />
        </Field>
        <Field label="Stadt">
          <Input
            value={form.adresse.stadt}
            onChange={(e) => updateAdresse("stadt", e.target.value)}
            required
          />
        </Field>
        <Field label="Ländercode">
          <Input
            value={form.adresse.ländercode}
            onChange={(e) => updateAdresse("ländercode", e.target.value)}
            required
          />
        </Field>
      </div>

      <Checkbox
        label="Einwilligung erteilt"
        checked={form.einwilligung}
        onChange={(e) => updateField("einwilligung", e.target.checked)}
      />

      {apiError ? <Alert variant="error">{apiError}</Alert> : null}

      {mutation.error ? (
        <Alert variant="error">{String(mutation.error)}</Alert>
      ) : null}

      <div className="flex justify-end gap-3">
        <Button type="button" variant="secondary" onClick={() => router.back()}>
          Zurück
        </Button>
        <Button type="submit" disabled={!isReady || mutation.isPending}>
          {mutation.isPending ? "Speichern…" : "Klient erstellen"}
        </Button>
      </div>
    </form>
  );
}
