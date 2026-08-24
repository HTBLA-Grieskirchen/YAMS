"use client";

import { type FormEvent, useState } from "react";

import { useTagesabschlussDurchführenMutation } from "@/api/hooks";
import type { Leistung, Rechnung } from "@/api/types";
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
import { todayIsoDate } from "@/lib/dates";

type TagesabschlussFormProps = {
  lastLeistung: Leistung | null;
  rechnungen: Rechnung[];
  onCompleted: (rechnungen: Rechnung[]) => void;
};

export function TagesabschlussForm({
  lastLeistung,
  rechnungen,
  onCompleted,
}: TagesabschlussFormProps) {
  const mutation = useTagesabschlussDurchführenMutation();
  const [abschlussdatum, setAbschlussdatum] = useState(todayIsoDate());

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    const result = await mutation.mutateAsync({ abschlussdatum });
    onCompleted(result);
  }

  const hasResult = rechnungen.length > 0;

  return (
    <Card>
      <CardHeader>
        <CardTitle>5. Tagesabschluss</CardTitle>
        <CardDescription>
          Schließt offene Leistungen ab und erzeugt Rechnungen.
        </CardDescription>
      </CardHeader>
      <CardContent>
        {!lastLeistung ? (
          <p className="text-sm text-zinc-500">
            Buche zuerst eine Leistung, bevor du den Tagesabschluss startest.
          </p>
        ) : hasResult ? (
          <Alert variant="success">
            Tagesabschluss abgeschlossen — {rechnungen.length} Rechnung(en)
            erstellt.
          </Alert>
        ) : (
          <form className="space-y-4" onSubmit={handleSubmit}>
            <Field label="Abschlussdatum">
              <Input
                type="date"
                value={abschlussdatum}
                onChange={(e) => setAbschlussdatum(e.target.value)}
              />
            </Field>

            {mutation.error ? (
              <Alert variant="error">{String(mutation.error)}</Alert>
            ) : null}

            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending ? "Abschließen…" : "Tagesabschluss durchführen"}
            </Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}
