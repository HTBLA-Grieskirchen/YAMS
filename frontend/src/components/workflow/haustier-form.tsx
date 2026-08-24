"use client";

import { type FormEvent, useEffect, useState } from "react";

import { useHaustierErstellenMutation } from "@/api/hooks";
import type { Haustier, HaustierErstellung } from "@/api/types";
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

type HaustierFormProps = {
  klientId: string | undefined;
  haustier: Haustier | null;
  onCreated: (haustier: Haustier) => void;
};

function defaultHaustier(klientId: string): HaustierErstellung {
  return {
    klientId,
    name: "Bello",
    geburtstag: "2020-06-01",
    tierart: "Hund",
    beschreibung: "Freundlicher Labrador",
  };
}

export function HaustierForm({
  klientId,
  haustier,
  onCreated,
}: HaustierFormProps) {
  const mutation = useHaustierErstellenMutation();
  const [form, setForm] = useState<HaustierErstellung | null>(null);

  useEffect(() => {
    if (klientId && !haustier) {
      setForm(defaultHaustier(klientId));
    }
  }, [klientId, haustier]);

  if (!klientId) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>2. Haustier anlegen (optional)</CardTitle>
          <CardDescription>
            Lege zuerst einen Klienten an, um ein Haustier zu verknüpfen.
          </CardDescription>
        </CardHeader>
      </Card>
    );
  }

  if (!form) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>2. Haustier anlegen (optional)</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-zinc-500">Formular wird vorbereitet…</p>
        </CardContent>
      </Card>
    );
  }

  function updateField<K extends keyof HaustierErstellung>(
    key: K,
    value: HaustierErstellung[K],
  ) {
    setForm((prev) => (prev ? { ...prev, [key]: value } : prev));
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!form) return;
    const created = await mutation.mutateAsync(form);
    onCreated(created);
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>2. Haustier anlegen (optional)</CardTitle>
        <CardDescription>
          Verknüpft ein Haustier mit dem Klienten für Leistungen. Optional —
          du kannst direkt zum Katalog weitergehen.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {haustier ? (
          <Alert variant="success">
            Haustier erstellt: {haustier.name} ({haustier.tierart})
          </Alert>
        ) : (
          <form className="space-y-4" onSubmit={handleSubmit}>
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

            {mutation.error ? (
              <Alert variant="error">{String(mutation.error)}</Alert>
            ) : null}

            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending ? "Speichern…" : "Haustier erstellen"}
            </Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
