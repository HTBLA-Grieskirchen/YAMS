"use client";

import Link from "next/link";
import { useMemo } from "react";

import {
  useAlleLeistungenQuery,
  useAlleRechnungenQuery,
  useRechnungenFürKlientQuery,
} from "@/api/hooks";
import { LeistungStatus, RechnungStatus } from "@/api/schema";
import type { Leistung, Rechnung } from "@/api/types";
import { Badge } from "@/components/ui/badge";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { formatDate } from "@/lib/format";
import { paths } from "@/lib/navigation";

export type LeistungPaymentStatus = "Offen" | "Abgerechnet" | "Bezahlt";

export type KlientLeistungenSummary = {
  offen: { count: number; sum: number };
  abgerechnet: { count: number; sum: number };
  bezahlt: { count: number; sum: number };
};

export function resolveLeistungPaymentStatus(
  leistung: Leistung,
  rechnungenById: Map<string, Rechnung>,
): LeistungPaymentStatus {
  if (leistung.status === LeistungStatus.Offen) {
    return "Offen";
  }

  if (leistung.rechnungId) {
    const rechnung = rechnungenById.get(leistung.rechnungId);
    if (rechnung?.status === RechnungStatus.Bezahlt) {
      return "Bezahlt";
    }
  }

  return "Abgerechnet";
}

function summaryKeyForStatus(
  status: LeistungPaymentStatus,
): keyof KlientLeistungenSummary {
  switch (status) {
    case "Offen":
      return "offen";
    case "Bezahlt":
      return "bezahlt";
    default:
      return "abgerechnet";
  }
}

export function summarizeKlientLeistungen(
  leistungen: Leistung[],
  rechnungenById: Map<string, Rechnung>,
): KlientLeistungenSummary {
  const summary: KlientLeistungenSummary = {
    offen: { count: 0, sum: 0 },
    abgerechnet: { count: 0, sum: 0 },
    bezahlt: { count: 0, sum: 0 },
  };

  for (const leistung of leistungen) {
    const status = resolveLeistungPaymentStatus(leistung, rechnungenById);
    const bucket = summary[summaryKeyForStatus(status)];
    bucket.count += 1;
    bucket.sum += Number.parseFloat(leistung.betrag) || 0;
  }

  return summary;
}

function statusBadgeVariant(
  status: LeistungPaymentStatus,
): "default" | "success" | "warning" {
  switch (status) {
    case "Offen":
      return "warning";
    case "Bezahlt":
      return "success";
    default:
      return "default";
  }
}

function formatEuroSum(value: number): string {
  return value.toLocaleString("de-AT", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

type KlientLeistungenSummaryBadgesProps = {
  summary: KlientLeistungenSummary;
};

export function KlientLeistungenSummaryBadges({
  summary,
}: KlientLeistungenSummaryBadgesProps) {
  const hasAny =
    summary.offen.count > 0 ||
    summary.abgerechnet.count > 0 ||
    summary.bezahlt.count > 0;

  if (!hasAny) {
    return <span className="text-sm text-zinc-400">—</span>;
  }

  return (
    <div className="flex flex-wrap gap-1.5">
      {summary.offen.count > 0 ? (
        <Badge variant="warning">
          {summary.offen.count} offen · {formatEuroSum(summary.offen.sum)} €
        </Badge>
      ) : null}
      {summary.abgerechnet.count > 0 ? (
        <Badge>
          {summary.abgerechnet.count} abgerechnet ·{" "}
          {formatEuroSum(summary.abgerechnet.sum)} €
        </Badge>
      ) : null}
      {summary.bezahlt.count > 0 ? (
        <Badge variant="success">
          {summary.bezahlt.count} bezahlt · {formatEuroSum(summary.bezahlt.sum)}{" "}
          €
        </Badge>
      ) : null}
    </div>
  );
}

type KlientLeistungenSectionProps = {
  klientId: string;
  leistungen?: Leistung[];
  rechnungenById?: Map<string, Rechnung>;
  compact?: boolean;
};

export function KlientLeistungenSection({
  klientId,
  leistungen: leistungenProp,
  rechnungenById: rechnungenByIdProp,
  compact = false,
}: KlientLeistungenSectionProps) {
  const leistungenQuery = useAlleLeistungenQuery();
  const rechnungenQuery = useAlleRechnungenQuery();
  const klientRechnungenQuery = useRechnungenFürKlientQuery(
    leistungenProp && rechnungenByIdProp ? undefined : klientId,
  );

  const rechnungenById = useMemo(() => {
    if (rechnungenByIdProp) {
      return rechnungenByIdProp;
    }

    const entries =
      rechnungenQuery.data?.map(
        (rechnung) => [rechnung.id, rechnung] as const,
      ) ??
      klientRechnungenQuery.data?.map(
        (rechnung) => [rechnung.id, rechnung] as const,
      ) ??
      [];

    return new Map(entries);
  }, [klientRechnungenQuery.data, rechnungenByIdProp, rechnungenQuery.data]);

  const klientLeistungen = useMemo(() => {
    const all = leistungenProp ?? leistungenQuery.data ?? [];
    return all
      .filter((leistung) => leistung.klientId === klientId)
      .sort((a, b) => b.leistungsdatum.localeCompare(a.leistungsdatum));
  }, [klientId, leistungenProp, leistungenQuery.data]);

  const summary = useMemo(
    () => summarizeKlientLeistungen(klientLeistungen, rechnungenById),
    [klientLeistungen, rechnungenById],
  );

  const isLoading =
    (leistungenProp === undefined && leistungenQuery.isPending) ||
    (rechnungenByIdProp === undefined &&
      rechnungenQuery.isPending &&
      klientRechnungenQuery.isPending);

  if (isLoading) {
    return <p className="text-sm text-zinc-500">Lade Leistungen…</p>;
  }

  if (klientLeistungen.length === 0) {
    return (
      <div className="space-y-2">
        <p className="text-sm text-zinc-500">Keine Leistungen.</p>
        <Link
          href={paths.leistungForKlient(klientId)}
          className="text-sm font-medium text-emerald-700 hover:underline dark:text-emerald-300"
        >
          Leistung buchen
        </Link>
      </div>
    );
  }

  if (compact) {
    return (
      <div className="space-y-3">
        <KlientLeistungenSummaryBadges summary={summary} />
        <ul className="space-y-2">
          {klientLeistungen.slice(0, 5).map((leistung) => {
            const status = resolveLeistungPaymentStatus(
              leistung,
              rechnungenById,
            );
            return (
              <li
                key={leistung.id}
                className="flex flex-wrap items-center justify-between gap-2 rounded-lg border border-zinc-200 px-3 py-2 text-sm dark:border-zinc-800"
              >
                <div>
                  <p className="font-medium">{leistung.beschreibung}</p>
                  <p className="text-zinc-500">
                    {formatDate(leistung.leistungsdatum)} · {leistung.betrag} €
                  </p>
                </div>
                <Badge variant={statusBadgeVariant(status)}>{status}</Badge>
              </li>
            );
          })}
        </ul>
        {klientLeistungen.length > 5 ? (
          <p className="text-xs text-zinc-500">
            + {klientLeistungen.length - 5} weitere in der Akte
          </p>
        ) : null}
        <Link
          href={paths.klient(klientId)}
          className="text-sm font-medium text-emerald-700 hover:underline dark:text-emerald-300"
        >
          Alle Leistungen in der Akte
        </Link>
      </div>
    );
  }

  const offene = klientLeistungen.filter(
    (leistung) =>
      resolveLeistungPaymentStatus(leistung, rechnungenById) === "Offen",
  );
  const abgerechnete = klientLeistungen.filter((leistung) => {
    const status = resolveLeistungPaymentStatus(leistung, rechnungenById);
    return status === "Abgerechnet" || status === "Bezahlt";
  });

  return (
    <div className="space-y-4">
      <KlientLeistungenSummaryBadges summary={summary} />

      {offene.length > 0 ? (
        <LeistungenTable
          title="Offene Leistungen"
          leistungen={offene}
          rechnungenById={rechnungenById}
        />
      ) : null}

      {abgerechnete.length > 0 ? (
        <LeistungenTable
          title="Abgerechnete Leistungen"
          leistungen={abgerechnete}
          rechnungenById={rechnungenById}
          showPaymentStatus
        />
      ) : null}
    </div>
  );
}

function LeistungenTable({
  title,
  leistungen,
  rechnungenById,
  showPaymentStatus = false,
}: {
  title: string;
  leistungen: Leistung[];
  rechnungenById: Map<string, Rechnung>;
  showPaymentStatus?: boolean;
}) {
  return (
    <div className="space-y-2">
      <p className="text-xs font-semibold tracking-wide text-zinc-500 uppercase">
        {title}
      </p>
      <Table>
        <TableHead>
          <TableRow>
            <TableHeader>Datum</TableHeader>
            <TableHeader>Beschreibung</TableHeader>
            <TableHeader>Betrag</TableHeader>
            {showPaymentStatus ? <TableHeader>Status</TableHeader> : null}
          </TableRow>
        </TableHead>
        <TableBody>
          {leistungen.map((leistung) => {
            const status = resolveLeistungPaymentStatus(
              leistung,
              rechnungenById,
            );
            return (
              <TableRow key={leistung.id}>
                <TableCell>{formatDate(leistung.leistungsdatum)}</TableCell>
                <TableCell className="font-medium">
                  {leistung.beschreibung}
                </TableCell>
                <TableCell>{leistung.betrag} €</TableCell>
                {showPaymentStatus ? (
                  <TableCell>
                    <Badge variant={statusBadgeVariant(status)}>{status}</Badge>
                  </TableCell>
                ) : null}
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </div>
  );
}
