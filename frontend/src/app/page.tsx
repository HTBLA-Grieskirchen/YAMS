"use client";

import { useState } from "react";

import type {
  Behandlung,
  Haustier,
  Klient,
  Leistung,
  Produkt,
  Rechnung,
} from "@/api/types";
import { DeploymentStatus } from "@/components/workflow/deployment-status";
import { HaustierForm } from "@/components/workflow/haustier-form";
import { KatalogForm } from "@/components/workflow/katalog-form";
import { KlientForm } from "@/components/workflow/klient-form";
import { LeistungForm } from "@/components/workflow/leistung-form";
import { RechnungenPanel } from "@/components/workflow/rechnungen-panel";
import { TagesabschlussForm } from "@/components/workflow/tagesabschluss-form";
import { WorkflowSteps } from "@/components/workflow/workflow-steps";

function deriveCurrentStep(
  klient: Klient | null,
  produkt: Produkt | null,
  behandlung: Behandlung | null,
  lastLeistung: Leistung | null,
  rechnungen: Rechnung[],
): string {
  if (!klient) return "klient";
  if (!produkt && !behandlung) return "katalog";
  if (!lastLeistung) return "leistung";
  if (rechnungen.length === 0) return "abschluss";
  return "rechnungen";
}

export default function Home() {
  const [klient, setKlient] = useState<Klient | null>(null);
  const [haustier, setHaustier] = useState<Haustier | null>(null);
  const [produkt, setProdukt] = useState<Produkt | null>(null);
  const [behandlung, setBehandlung] = useState<Behandlung | null>(null);
  const [lastLeistung, setLastLeistung] = useState<Leistung | null>(null);
  const [rechnungen, setRechnungen] = useState<Rechnung[]>([]);

  const currentStep = deriveCurrentStep(
    klient,
    produkt,
    behandlung,
    lastLeistung,
    rechnungen,
  );

  const steps = [
    { id: "klient", label: "Klient", done: klient !== null },
    {
      id: "haustier",
      label: "Haustier",
      done: haustier !== null,
      optional: true,
    },
    {
      id: "katalog",
      label: "Katalog",
      done: produkt !== null || behandlung !== null,
    },
    { id: "leistung", label: "Leistung", done: lastLeistung !== null },
    {
      id: "abschluss",
      label: "Abschluss",
      done: rechnungen.length > 0,
    },
    {
      id: "rechnungen",
      label: "Rechnungen",
      done: rechnungen.length > 0,
    },
  ];

  return (
    <main className="mx-auto flex min-h-screen max-w-4xl flex-col gap-6 p-6 font-sans sm:p-8">
      <header className="space-y-3">
        <h1 className="text-3xl font-semibold tracking-tight">YAMS</h1>
        <p className="text-zinc-600 dark:text-zinc-400">
          Abrechnungs-Workflow: Klient → Haustier → Katalog → Leistung →
          Tagesabschluss → Rechnungen.
        </p>
        <WorkflowSteps steps={steps} currentStepId={currentStep} />
      </header>

      <DeploymentStatus />

      <KlientForm klient={klient} onCreated={setKlient} />

      <HaustierForm
        klientId={klient?.id}
        haustier={haustier}
        onCreated={setHaustier}
      />

      <KatalogForm
        produkt={produkt}
        behandlung={behandlung}
        onProduktCreated={setProdukt}
        onBehandlungCreated={setBehandlung}
      />

      <LeistungForm
        klient={klient}
        haustier={haustier}
        produkt={produkt}
        behandlung={behandlung}
        lastLeistung={lastLeistung}
        onBooked={setLastLeistung}
      />

      <TagesabschlussForm
        lastLeistung={lastLeistung}
        rechnungen={rechnungen}
        onCompleted={setRechnungen}
      />

      <RechnungenPanel klient={klient} localRechnungen={rechnungen} />
    </main>
  );
}
