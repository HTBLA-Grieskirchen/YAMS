"use client";

import { type FormEvent, useState } from "react";

import {
  useBehandlungErstellenMutation,
  useProduktErstellenMutation,
} from "@/api/hooks";
import type {
  Behandlung,
  BehandlungErstellung,
  Produkt,
  ProduktErstellung,
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

type KatalogFormProps = {
  produkt: Produkt | null;
  behandlung: Behandlung | null;
  onProduktCreated: (produkt: Produkt) => void;
  onBehandlungCreated: (behandlung: Behandlung) => void;
};

const defaultProdukt: ProduktErstellung = {
  name: "Hundefutter Premium",
  beschreibung: "5 kg Trockenfutter",
  einzelpreis: "24.99",
  mwstProzentsatz: "20.00",
};

const defaultBehandlung: BehandlungErstellung = {
  name: "Kastration",
  beschreibung: "Routine-OP",
  standardpreis: "180.00",
  mwstProzentsatz: "20.00",
};

export function KatalogForm({
  produkt,
  behandlung,
  onProduktCreated,
  onBehandlungCreated,
}: KatalogFormProps) {
  const produktMutation = useProduktErstellenMutation();
  const behandlungMutation = useBehandlungErstellenMutation();
  const [produktForm, setProduktForm] = useState<ProduktErstellung>(defaultProdukt);
  const [behandlungForm, setBehandlungForm] =
    useState<BehandlungErstellung>(defaultBehandlung);

  async function handleProduktSubmit(event: FormEvent) {
    event.preventDefault();
    const created = await produktMutation.mutateAsync(produktForm);
    onProduktCreated(created);
  }

  async function handleBehandlungSubmit(event: FormEvent) {
    event.preventDefault();
    const created = await behandlungMutation.mutateAsync(behandlungForm);
    onBehandlungCreated(created);
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>3. Katalog</CardTitle>
        <CardDescription>
          Lege mindestens ein Produkt oder eine Behandlung für Leistungen an.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-8">
        <section className="space-y-4">
          <h3 className="text-sm font-semibold text-zinc-800 dark:text-zinc-200">
            Produkt
          </h3>
          {produkt ? (
            <Alert variant="success">
              Produkt erstellt: {produkt.name} ({produkt.einzelpreis} €)
            </Alert>
          ) : (
            <form className="space-y-4" onSubmit={handleProduktSubmit}>
              <div className="grid gap-4 sm:grid-cols-2">
                <Field label="Name">
                  <Input
                    value={produktForm.name}
                    onChange={(e) =>
                      setProduktForm((p) => ({ ...p, name: e.target.value }))
                    }
                    required
                  />
                </Field>
                <Field label="Beschreibung">
                  <Input
                    value={produktForm.beschreibung}
                    onChange={(e) =>
                      setProduktForm((p) => ({
                        ...p,
                        beschreibung: e.target.value,
                      }))
                    }
                    required
                  />
                </Field>
                <Field label="Einzelpreis" hint="Dezimal als String, z. B. 24.99">
                  <Input
                    value={produktForm.einzelpreis}
                    onChange={(e) =>
                      setProduktForm((p) => ({
                        ...p,
                        einzelpreis: e.target.value,
                      }))
                    }
                    required
                  />
                </Field>
                <Field label="MwSt %" hint="Dezimal als String, z. B. 20.00">
                  <Input
                    value={produktForm.mwstProzentsatz}
                    onChange={(e) =>
                      setProduktForm((p) => ({
                        ...p,
                        mwstProzentsatz: e.target.value,
                      }))
                    }
                    required
                  />
                </Field>
              </div>
              {produktMutation.error ? (
                <Alert variant="error">{String(produktMutation.error)}</Alert>
              ) : null}
              <Button type="submit" disabled={produktMutation.isPending}>
                {produktMutation.isPending ? "Speichern…" : "Produkt erstellen"}
              </Button>
            </form>
          )}
        </section>

        <section className="space-y-4 border-t border-zinc-200 pt-6 dark:border-zinc-800">
          <h3 className="text-sm font-semibold text-zinc-800 dark:text-zinc-200">
            Behandlung
          </h3>
          {behandlung ? (
            <Alert variant="success">
              Behandlung erstellt: {behandlung.name} ({behandlung.standardpreis}{" "}
              €)
            </Alert>
          ) : (
            <form className="space-y-4" onSubmit={handleBehandlungSubmit}>
              <div className="grid gap-4 sm:grid-cols-2">
                <Field label="Name">
                  <Input
                    value={behandlungForm.name}
                    onChange={(e) =>
                      setBehandlungForm((b) => ({ ...b, name: e.target.value }))
                    }
                    required
                  />
                </Field>
                <Field label="Beschreibung">
                  <Input
                    value={behandlungForm.beschreibung}
                    onChange={(e) =>
                      setBehandlungForm((b) => ({
                        ...b,
                        beschreibung: e.target.value,
                      }))
                    }
                    required
                  />
                </Field>
                <Field label="Standardpreis">
                  <Input
                    value={behandlungForm.standardpreis}
                    onChange={(e) =>
                      setBehandlungForm((b) => ({
                        ...b,
                        standardpreis: e.target.value,
                      }))
                    }
                    required
                  />
                </Field>
                <Field label="MwSt %">
                  <Input
                    value={behandlungForm.mwstProzentsatz}
                    onChange={(e) =>
                      setBehandlungForm((b) => ({
                        ...b,
                        mwstProzentsatz: e.target.value,
                      }))
                    }
                    required
                  />
                </Field>
              </div>
              {behandlungMutation.error ? (
                <Alert variant="error">{String(behandlungMutation.error)}</Alert>
              ) : null}
              <Button type="submit" disabled={behandlungMutation.isPending}>
                {behandlungMutation.isPending
                  ? "Speichern…"
                  : "Behandlung erstellen"}
              </Button>
            </form>
          )}
        </section>
      </CardContent>
    </Card>
  );
}
