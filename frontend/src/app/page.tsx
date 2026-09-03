"use client";

import { useState } from "react";

import type {
  Behandlung,
  Haustier,
  Klient,
  Leistung,
  Produkt,
  Rechnung,
  Seminar,
  SeminarTermin,
} from "@/api/types";
import { DeploymentStatus } from "@/components/workflow/deployment-status";
import { HaustierForm } from "@/components/workflow/haustier-form";
import { ObjektePanel } from "@/components/workflow/objekte-panel";
import { KatalogForm } from "@/components/workflow/katalog-form";
import { KlientForm } from "@/components/workflow/klient-form";
import { LeistungForm } from "@/components/workflow/leistung-form";
import { RechnungenPanel } from "@/components/workflow/rechnungen-panel";
import { SeminarBuchungForm } from "@/components/workflow/seminar-buchung-form";
import { SeminarForm } from "@/components/workflow/seminar-form";
import { SeminarTerminForm } from "@/components/workflow/seminar-termin-form";
import { SeminarTerminPanel } from "@/components/workflow/seminar-termin-panel";
import { TagesabschlussForm } from "@/components/workflow/tagesabschluss-form";
import { WorkflowSteps } from "@/components/workflow/workflow-steps";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/cn";

type TabId = "abrechnung" | "seminar" | "objekte";

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

const tabs: { id: TabId; label: string }[] = [
  { id: "abrechnung", label: "Abrechnung" },
  { id: "seminar", label: "Seminar" },
  { id: "objekte", label: "Objekte" },
];

export default function Home() {
  const [activeTab, setActiveTab] = useState<TabId>("abrechnung");

  const [klient, setKlient] = useState<Klient | null>(null);
  const [haustier, setHaustier] = useState<Haustier | null>(null);
  const [produkt, setProdukt] = useState<Produkt | null>(null);
  const [behandlung, setBehandlung] = useState<Behandlung | null>(null);
  const [lastLeistung, setLastLeistung] = useState<Leistung | null>(null);
  const [rechnungen, setRechnungen] = useState<Rechnung[]>([]);

  const [seminar, setSeminar] = useState<Seminar | null>(null);
  const [seminarTermin, setSeminarTermin] = useState<SeminarTermin | null>(null);

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
          Verwaltung über Tauri (embedded) oder HTTP (remote).
        </p>
        <nav className="flex flex-wrap gap-2">
          {tabs.map((tab) => (
            <Button
              key={tab.id}
              type="button"
              variant={activeTab === tab.id ? "primary" : "secondary"}
              onClick={() => setActiveTab(tab.id)}
              className={cn(activeTab !== tab.id && "font-normal")}
            >
              {tab.label}
            </Button>
          ))}
        </nav>
      </header>

      <DeploymentStatus />

      {activeTab === "abrechnung" ? (
        <>
          <p className="text-sm text-zinc-600 dark:text-zinc-400">
            Abrechnungs-Workflow: Klient → Haustier → Katalog → Leistung →
            Tagesabschluss → Rechnungen.
          </p>
          <WorkflowSteps steps={steps} currentStepId={currentStep} />
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
        </>
      ) : null}

      {activeTab === "seminar" ? (
        <>
          <p className="text-sm text-zinc-600 dark:text-zinc-400">
            Seminar-Workflow: Seminar → Termin → Buchung → Abhalten / PDF.
          </p>
          <SeminarForm seminar={seminar} onCreated={setSeminar} />
          <SeminarTerminForm
            seminar={seminar}
            termin={seminarTermin}
            onCreated={setSeminarTermin}
          />
          <SeminarBuchungForm
            klient={klient}
            termin={seminarTermin}
            onBooked={setSeminarTermin}
          />
          <SeminarTerminPanel
            termin={seminarTermin}
            onUpdated={setSeminarTermin}
          />
          {!klient ? (
            <p className="text-sm text-amber-700 dark:text-amber-400">
              Hinweis: Für Buchungen zuerst im Tab Abrechnung einen Klienten
              anlegen.
            </p>
          ) : null}
        </>
      ) : null}

      {activeTab === "objekte" ? <ObjektePanel /> : null}
    </main>
  );
}
