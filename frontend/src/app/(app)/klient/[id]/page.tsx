"use client";

import { useParams } from "next/navigation";

import { useAlleKlientenQuery } from "@/api/hooks";
import { KlientDetail } from "@/components/klient/klient-detail";
import { Alert } from "@/components/ui/alert";

export default function KlientDetailPage() {
  const params = useParams<{ id: string }>();
  const klientenQuery = useAlleKlientenQuery();
  const klient = (klientenQuery.data ?? []).find(
    (entry) => entry.id === params.id,
  );

  if (klientenQuery.isPending) {
    return <p className="p-6 text-sm text-zinc-500">Lade Klient…</p>;
  }

  if (klientenQuery.error) {
    return (
      <div className="p-6">
        <Alert variant="error">{String(klientenQuery.error)}</Alert>
      </div>
    );
  }

  if (!klient) {
    return (
      <div className="p-6">
        <Alert variant="error">Klient nicht gefunden.</Alert>
      </div>
    );
  }

  return <KlientDetail klient={klient} />;
}
