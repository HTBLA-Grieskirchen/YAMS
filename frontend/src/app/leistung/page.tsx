"use client";

import { useSearchParams } from "next/navigation";
import { Suspense } from "react";

import { LeistungBuchenForm } from "@/components/leistung/leistung-buchen-form";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

function LeistungPageContent() {
  const searchParams = useSearchParams();
  const klientId = searchParams.get("klientId");

  return (
    <div className="space-y-6 p-6">
      <div className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">
          Leistung buchen
        </h1>
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          Produkt oder Behandlung an einen Klienten verkaufen — optional mit
          Haustierbezug.
        </p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Neue Leistung</CardTitle>
          <CardDescription>
            Offene Leistungen erscheinen später in der Abrechnung.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <LeistungBuchenForm initialKlientId={klientId} />
        </CardContent>
      </Card>
    </div>
  );
}

export default function LeistungPage() {
  return (
    <Suspense fallback={<p className="p-6 text-sm text-zinc-500">Lade…</p>}>
      <LeistungPageContent />
    </Suspense>
  );
}
