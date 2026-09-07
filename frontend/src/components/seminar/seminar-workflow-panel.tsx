"use client";

import { useState } from "react";

import type { Klient, Seminar, SeminarTermin } from "@/api/types";
import { KlientForm } from "@/components/workflow/klient-form";
import { SeminarBuchungForm } from "@/components/workflow/seminar-buchung-form";
import { SeminarForm } from "@/components/workflow/seminar-form";
import { SeminarTerminForm } from "@/components/workflow/seminar-termin-form";
import { SeminarTerminPanel } from "@/components/workflow/seminar-termin-panel";

export function SeminarWorkflowPanel() {
  const [klient, setKlient] = useState<Klient | null>(null);
  const [seminar, setSeminar] = useState<Seminar | null>(null);
  const [seminarTermin, setSeminarTermin] = useState<SeminarTermin | null>(
    null,
  );

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h2 className="text-xl font-semibold">Seminar-Workflow</h2>
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          Seminar → Termin → Buchung → Abhalten / PDF
        </p>
      </div>

      {!klient ? (
        <KlientForm klient={klient} onCreated={setKlient} />
      ) : (
        <p className="text-sm text-emerald-700 dark:text-emerald-300">
          Klient: {klient.vorname} {klient.nachname}
        </p>
      )}

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
      <SeminarTerminPanel termin={seminarTermin} onUpdated={setSeminarTermin} />
    </div>
  );
}
