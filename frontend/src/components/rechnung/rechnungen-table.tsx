"use client";

import Link from "next/link";

import { RechnungStatus } from "@/api/schema";
import type { Klient, Rechnung } from "@/api/types";
import { RechnungDownloadButton } from "@/components/rechnung/rechnung-download-button";
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

type RechnungenTableProps = {
  rechnungen: Rechnung[];
  klientenById?: Map<string, Klient>;
  showKlient?: boolean;
};

function statusVariant(
  status: Rechnung["status"],
): "default" | "success" | "warning" {
  if (status === RechnungStatus.Bezahlt) {
    return "success";
  }
  return "warning";
}

export function RechnungenTable({
  rechnungen,
  klientenById,
  showKlient = false,
}: RechnungenTableProps) {
  if (rechnungen.length === 0) {
    return <p className="text-sm text-zinc-500">Keine Rechnungen.</p>;
  }

  return (
    <Table>
      <TableHead>
        <TableRow>
          <TableHeader>Nr.</TableHeader>
          <TableHeader>Datum</TableHeader>
          {showKlient ? <TableHeader>Klient</TableHeader> : null}
          <TableHeader>Betrag</TableHeader>
          <TableHeader>Status</TableHeader>
          <TableHeader className="w-24" />
        </TableRow>
      </TableHead>
      <TableBody>
        {rechnungen.map((rechnung) => {
          const klient = klientenById?.get(rechnung.klientId);
          return (
            <TableRow key={rechnung.id}>
              <TableCell className="font-medium">
                #{rechnung.rechnungsnummer}
              </TableCell>
              <TableCell>{formatDate(rechnung.rechnungsdatum)}</TableCell>
              {showKlient ? (
                <TableCell>
                  {klient ? (
                    <Link
                      href={paths.klient(klient.id)}
                      className="text-emerald-700 hover:underline dark:text-emerald-300"
                    >
                      {klient.vorname} {klient.nachname}
                    </Link>
                  ) : (
                    rechnung.klientId.slice(0, 8)
                  )}
                </TableCell>
              ) : null}
              <TableCell>{rechnung.gesamtbetragBrutto} €</TableCell>
              <TableCell>
                <Badge variant={statusVariant(rechnung.status)}>
                  {rechnung.status}
                </Badge>
              </TableCell>
              <TableCell>
                <RechnungDownloadButton
                  rechnungId={rechnung.id}
                  rechnungsnummer={rechnung.rechnungsnummer}
                />
              </TableCell>
            </TableRow>
          );
        })}
      </TableBody>
    </Table>
  );
}
