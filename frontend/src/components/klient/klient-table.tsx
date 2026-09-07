"use client";

import Link from "next/link";
import {
  CheckCircle2,
  ChevronDown,
  ChevronUp,
  Mail,
  MoreVertical,
  Phone,
  TriangleAlert,
} from "lucide-react";
import { useMemo, useState } from "react";

import { useAlleKlientenQuery } from "@/api/hooks";
import type { Klient } from "@/api/types";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { SearchInput } from "@/components/ui/search-input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { formatDate, matchesSearchQuery } from "@/lib/format";
import { paths } from "@/lib/navigation";

export function KlientTable() {
  const klientenQuery = useAlleKlientenQuery();
  const [filter, setFilter] = useState("");

  const filtered = useMemo(() => {
    const klienten = klientenQuery.data ?? [];
    return klienten.filter((klient) =>
      matchesSearchQuery(filter, [
        klient.vorname,
        klient.nachname,
        klient.email,
        klient.mobilnummer,
        klient.kundennummer,
        klient.adresse.stadt,
        klient.adresse.postleitzahl,
      ]),
    );
  }, [filter, klientenQuery.data]);

  if (klientenQuery.isPending) {
    return <p className="text-sm text-zinc-500">Lade Klienten…</p>;
  }

  if (klientenQuery.error) {
    return <Alert variant="error">{String(klientenQuery.error)}</Alert>;
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          {filtered.length} Klient(en)
        </p>
        <SearchInput
          value={filter}
          onChange={(event) => setFilter(event.target.value)}
          placeholder="Suchen…"
        />
      </div>

      {filtered.length === 0 ? (
        <Alert variant="info">Keine Klienten gefunden.</Alert>
      ) : (
        <Table>
          <TableHead>
            <TableRow>
              <TableHeader className="w-10" />
              <TableHeader>Nachname</TableHeader>
              <TableHeader>Vorname</TableHeader>
              <TableHeader>Geburtstag</TableHeader>
              <TableHeader className="w-12" />
              <TableHeader className="w-12" />
            </TableRow>
          </TableHead>
          <TableBody>
            {filtered.map((klient) => (
              <KlientTableRow key={klient.id} klient={klient} />
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
}

function KlientTableRow({ klient }: { klient: Klient }) {
  const [expanded, setExpanded] = useState(false);

  return (
    <>
      <TableRow>
        <TableCell>
          {klient.einwilligung ? (
            <CheckCircle2 className="size-5 text-emerald-600" />
          ) : (
            <TriangleAlert className="size-5 text-red-600" />
          )}
        </TableCell>
        <TableCell className="font-medium">{klient.nachname}</TableCell>
        <TableCell>{klient.vorname}</TableCell>
        <TableCell>{formatDate(klient.geburtstag)}</TableCell>
        <TableCell>
          <button
            type="button"
            onClick={() => setExpanded((value) => !value)}
            className="rounded-md p-1 text-zinc-400 hover:bg-zinc-100 hover:text-zinc-700 dark:hover:bg-zinc-900"
            aria-expanded={expanded}
          >
            {expanded ? (
              <ChevronUp className="size-5" />
            ) : (
              <ChevronDown className="size-5" />
            )}
          </button>
        </TableCell>
        <TableCell>
          <details className="relative">
            <summary className="flex size-8 cursor-pointer list-none items-center justify-center rounded-md hover:bg-zinc-100 dark:hover:bg-zinc-900 [&::-webkit-details-marker]:hidden">
              <MoreVertical className="size-4" />
            </summary>
            <div className="absolute right-0 z-20 mt-1 w-44 rounded-lg border border-zinc-200 bg-white p-1 shadow-lg dark:border-zinc-700 dark:bg-zinc-900">
              <Link
                href={paths.klient(klient.id)}
                className="block rounded-md px-3 py-2 text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800"
              >
                Akte öffnen
              </Link>
              <Link
                href={paths.leistungForKlient(klient.id)}
                className="block rounded-md px-3 py-2 text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800"
              >
                Leistung buchen
              </Link>
            </div>
          </details>
        </TableCell>
      </TableRow>

      {expanded ? (
        <TableRow className="bg-zinc-50 dark:bg-zinc-900/40">
          <TableCell colSpan={6}>
            <div className="max-w-2xl space-y-4 rounded-xl border border-zinc-200 bg-white p-4 dark:border-zinc-800 dark:bg-zinc-950">
              <p className="text-xs font-semibold tracking-wide text-zinc-500 uppercase">
                Weitere Informationen
              </p>
              <div className="space-y-2 text-sm">
                <p className="flex items-center gap-2">
                  <Mail className="size-4 text-zinc-400" />
                  <a
                    href={`mailto:${klient.email}`}
                    className="text-emerald-700 underline-offset-2 hover:underline dark:text-emerald-300"
                  >
                    {klient.email}
                  </a>
                </p>
                <p className="flex items-center gap-2">
                  <Phone className="size-4 text-zinc-400" />
                  {klient.mobilnummer}
                </p>
                <p className="text-zinc-600 dark:text-zinc-400">
                  {klient.adresse.straßeUndHausnummer},{" "}
                  {klient.adresse.postleitzahl} {klient.adresse.stadt},{" "}
                  {klient.adresse.ländercode}
                </p>
              </div>

              {klient.einwilligung ? (
                <Badge variant="success">Einwilligung erteilt</Badge>
              ) : (
                <Badge variant="error">Keine Einwilligung</Badge>
              )}

              <div>
                <p className="mb-2 text-xs font-semibold tracking-wide text-zinc-500 uppercase">
                  Haustiere
                </p>
                {klient.haustiere.length === 0 ? (
                  <p className="text-sm text-zinc-500">Keine Haustiere.</p>
                ) : (
                  <ul className="space-y-2">
                    {klient.haustiere.map((haustier) => (
                      <li
                        key={haustier.id}
                        className="rounded-lg border border-zinc-200 px-3 py-2 text-sm dark:border-zinc-800"
                      >
                        {haustier.name} ({haustier.tierart}) · Geb.{" "}
                        {formatDate(haustier.geburtstag)}
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            </div>
          </TableCell>
        </TableRow>
      ) : null}
    </>
  );
}
