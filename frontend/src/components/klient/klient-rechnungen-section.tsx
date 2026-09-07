"use client";

import { useMemo } from "react";

import { RechnungStatus } from "@/api/schema";
import { useRechnungenFürKlientQuery } from "@/api/hooks";
import { RechnungenTable } from "@/components/rechnung/rechnungen-table";
import { Alert } from "@/components/ui/alert";

type KlientRechnungenSectionProps = {
  klientId: string;
};

export function KlientRechnungenSection({
  klientId,
}: KlientRechnungenSectionProps) {
  const query = useRechnungenFürKlientQuery(klientId);

  const { offen, bezahlt } = useMemo(() => {
    const rechnungen = query.data ?? [];
    return {
      offen: rechnungen.filter(
        (rechnung) => rechnung.status === RechnungStatus.Offen,
      ),
      bezahlt: rechnungen.filter(
        (rechnung) => rechnung.status === RechnungStatus.Bezahlt,
      ),
    };
  }, [query.data]);

  if (query.isPending) {
    return <p className="text-sm text-zinc-500">Lade Rechnungen…</p>;
  }

  if (query.error) {
    return <Alert variant="error">{String(query.error)}</Alert>;
  }

  if ((query.data ?? []).length === 0) {
    return (
      <p className="text-sm text-zinc-500">
        Noch keine Rechnungen — Leistungen werden beim Tagesabschluss
        abgerechnet.
      </p>
    );
  }

  return (
    <div className="space-y-6">
      {offen.length > 0 ? (
        <div className="space-y-2">
          <p className="text-xs font-semibold tracking-wide text-zinc-500 uppercase">
            Offene Rechnungen
          </p>
          <RechnungenTable rechnungen={offen} />
        </div>
      ) : null}

      {bezahlt.length > 0 ? (
        <div className="space-y-2">
          <p className="text-xs font-semibold tracking-wide text-zinc-500 uppercase">
            Bezahlte Rechnungen
          </p>
          <RechnungenTable rechnungen={bezahlt} />
        </div>
      ) : null}
    </div>
  );
}
