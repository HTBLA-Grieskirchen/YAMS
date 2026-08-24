"use client";

import { type FormEvent, useState } from "react";

import { useKlientErstellenMutation } from "@/api/hooks";
import type { Klient, KlientErstellung } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

type KlientFormProps = {
  klient: Klient | null;
  onCreated: (klient: Klient) => void;
};

function defaultKlient(): KlientErstellung {
  const kundennummer = Date.now() % 1_000_000;

  return {
    vorname: "Maria",
    nachname: "Muster",
    geburtstag: "1985-03-15",
    email: "maria.muster@example.com",
    mobilnummer: "+43 699 12345678",
    kundennummer,
    einwilligung: true,
    adresse: {
      postleitzahl: "4040",
      stadt: "Linz",
      straßeUndHausnummer: "Landesstraße 1",
      ländercode: "AT",
    },
  };
}

export function KlientForm({ klient, onCreated }: KlientFormProps) {
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
    onCreated(created);
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>1. Klient anlegen</CardTitle>
        <CardDescription>
          Erstellt einen neuen Klienten für die Abrechnung.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {klient ? (
          <Alert variant="success">
            Klient erstellt: {klient.vorname} {klient.nachname} (Kundennummer{" "}
            {klient.kundennummer})
          </Alert>
        ) : (
          <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="grid gap-4 sm:grid-cols-2">
              <Field label="Vorname">
                <Input
                  value={form.vorname}
                  onChange={(e) => updateField("vorname", e.target.value)}
                  required
                />
              </Field>
              <Field label="Nachname">
                <Input
                  value={form.nachname}
                  onChange={(e) => updateField("nachname", e.target.value)}
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
              <Field label="E-Mail">
                <Input
                  type="email"
                  value={form.email}
                  onChange={(e) => updateField("email", e.target.value)}
                  required
                />
              </Field>
              <Field label="Mobilnummer">
                <Input
                  value={form.mobilnummer}
                  onChange={(e) => updateField("mobilnummer", e.target.value)}
                  required
                />
              </Field>
            </div>

            <div className="grid gap-4 sm:grid-cols-2">
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

            {mutation.error ? (
              <Alert variant="error">{String(mutation.error)}</Alert>
            ) : null}

            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending ? "Speichern…" : "Klient erstellen"}
            </Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
