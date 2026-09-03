"use client";

import { type FormEvent, useEffect, useState } from "react";

import { useSeminarBuchungAnlegenMutation } from "@/api/hooks";
import type {
  Klient,
  SeminarBuchungErstellung,
  SeminarTermin,
} from "@/api/types";
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

type SeminarBuchungFormProps = {
  klient: Klient | null;
  termin: SeminarTermin | null;
  onBooked: (termin: SeminarTermin) => void;
};

function defaultBuchung(klientId: string): SeminarBuchungErstellung {
  return {
    klientId,
    rabatt: "0.20",
  };
}

export function SeminarBuchungForm({
  klient,
  termin,
  onBooked,
}: SeminarBuchungFormProps) {
  const mutation = useSeminarBuchungAnlegenMutation();
  const [form, setForm] = useState<SeminarBuchungErstellung | null>(null);

  useEffect(() => {
    if (klient && termin && termin.status === "Geplant") {
      setForm(defaultBuchung(klient.id));
    }
  }, [klient, termin]);

  if (!termin) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Buchung anlegen</CardTitle>
          <CardDescription>Plane zuerst einen Termin.</CardDescription>
        </CardHeader>
      </Card>
    );
  }

  if (!klient) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Buchung anlegen</CardTitle>
          <CardDescription>
            Lege zuerst einen Klienten an (Tab Abrechnung).
          </CardDescription>
        </CardHeader>
      </Card>
    );
  }

  if (termin.status !== "Geplant") {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Buchung anlegen</CardTitle>
          <CardDescription>
            Termin ist {termin.status} — keine neuen Buchungen möglich.
          </CardDescription>
        </CardHeader>
      </Card>
    );
  }

  if (!form) {
    return null;
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const updated = await mutation.mutateAsync({
      terminId: termin!.id,
      body: form!,
    });
    onBooked(updated);
    setForm(defaultBuchung(klient!.id));
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Buchung anlegen</CardTitle>
        <CardDescription>
          Bucht {klient.vorname} {klient.nachname} für den Termin.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form className="space-y-4" onSubmit={handleSubmit}>
          <Field label="Rabatt (0–1)">
            <Input
              value={form.rabatt}
              onChange={(e) =>
                setForm((prev) =>
                  prev ? { ...prev, rabatt: e.target.value } : prev,
                )
              }
              required
            />
          </Field>

          {mutation.error ? (
            <Alert variant="error">{String(mutation.error)}</Alert>
          ) : null}

          <Button type="submit" disabled={mutation.isPending}>
            {mutation.isPending ? "Buchen…" : "Buchung anlegen"}
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}
