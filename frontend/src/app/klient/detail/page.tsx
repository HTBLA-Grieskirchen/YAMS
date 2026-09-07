"use client";

import Link from "next/link";
import { Suspense } from "react";
import { useSearchParams } from "next/navigation";

import { useAlleKlientenQuery } from "@/api/hooks";
import { KlientDetail } from "@/components/klient/klient-detail";
import { Alert } from "@/components/ui/alert";
import { paths } from "@/lib/navigation";

function KlientDetailContent() {
  const searchParams = useSearchParams();
  const id = searchParams.get("id");
  const klientenQuery = useAlleKlientenQuery();

  if (!id) {
    return (
      <div className="p-6">
        <Alert variant="error">Keine Klient-ID angegeben.</Alert>
      </div>
    );
  }

  const klient = (klientenQuery.data ?? []).find((entry) => entry.id === id);

  if (klientenQuery.isPending || (klientenQuery.isFetching && !klient)) {
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
        <Alert variant="error">
          Klient nicht gefunden. Bitte zur{" "}
          <Link href={paths.klienten} className="underline">
            Klientenübersicht
          </Link>{" "}
          zurückkehren.
        </Alert>
      </div>
    );
  }

  return <KlientDetail klient={klient} />;
}

export default function KlientDetailPage() {
  return (
    <Suspense fallback={<p className="p-6 text-sm text-zinc-500">Lade…</p>}>
      <KlientDetailContent />
    </Suspense>
  );
}
