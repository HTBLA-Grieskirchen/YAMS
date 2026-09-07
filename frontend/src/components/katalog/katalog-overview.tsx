"use client";

import { useMemo, useState } from "react";

import {
  useAlleBehandlungenQuery,
  useAlleProdukteQuery,
} from "@/api/hooks";
import { Alert } from "@/components/ui/alert";
import { SearchInput } from "@/components/ui/search-input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { matchesSearchQuery } from "@/lib/format";
import { cn } from "@/lib/cn";

type KatalogTab = "produkte" | "behandlungen";

export function KatalogOverview() {
  const [tab, setTab] = useState<KatalogTab>("produkte");
  const produkteQuery = useAlleProdukteQuery();
  const behandlungenQuery = useAlleBehandlungenQuery();
  const [filter, setFilter] = useState("");

  const produkte = useMemo(() => {
    return (produkteQuery.data ?? []).filter((produkt) =>
      matchesSearchQuery(filter, [
        produkt.name,
        produkt.einzelpreis,
        produkt.mwst,
      ]),
    );
  }, [filter, produkteQuery.data]);

  const behandlungen = useMemo(() => {
    return (behandlungenQuery.data ?? []).filter((behandlung) =>
      matchesSearchQuery(filter, [
        behandlung.name,
        behandlung.standardpreis,
        behandlung.mwst,
      ]),
    );
  }, [filter, behandlungenQuery.data]);

  const isLoading =
    tab === "produkte" ? produkteQuery.isPending : behandlungenQuery.isPending;
  const error =
    tab === "produkte" ? produkteQuery.error : behandlungenQuery.error;

  return (
    <div className="space-y-4 p-6">
      <div className="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
        <div className="flex gap-2">
          <TabButton
            active={tab === "produkte"}
            onClick={() => setTab("produkte")}
          >
            Produkte
          </TabButton>
          <TabButton
            active={tab === "behandlungen"}
            onClick={() => setTab("behandlungen")}
          >
            Behandlungen
          </TabButton>
        </div>
        <SearchInput
          value={filter}
          onChange={(event) => setFilter(event.target.value)}
          placeholder="Katalog durchsuchen…"
        />
      </div>

      {isLoading ? <p className="text-sm text-zinc-500">Lade Katalog…</p> : null}
      {error ? <Alert variant="error">{String(error)}</Alert> : null}

      {!isLoading && !error && tab === "produkte" ? (
        <Table>
          <TableHead>
            <TableRow>
              <TableHeader>Name</TableHeader>
              <TableHeader>Einzelpreis (netto)</TableHeader>
              <TableHeader>MwSt</TableHeader>
            </TableRow>
          </TableHead>
          <TableBody>
            {produkte.length === 0 ? (
              <TableRow>
                <TableCell colSpan={3} className="text-zinc-500">
                  Keine Produkte.
                </TableCell>
              </TableRow>
            ) : (
              produkte.map((produkt) => (
                <TableRow key={produkt.id}>
                  <TableCell className="font-medium">{produkt.name}</TableCell>
                  <TableCell>{produkt.einzelpreis} €</TableCell>
                  <TableCell>{produkt.mwst}</TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      ) : null}

      {!isLoading && !error && tab === "behandlungen" ? (
        <Table>
          <TableHead>
            <TableRow>
              <TableHeader>Name</TableHeader>
              <TableHeader>Standardpreis</TableHeader>
              <TableHeader>MwSt</TableHeader>
            </TableRow>
          </TableHead>
          <TableBody>
            {behandlungen.length === 0 ? (
              <TableRow>
                <TableCell colSpan={3} className="text-zinc-500">
                  Keine Behandlungen.
                </TableCell>
              </TableRow>
            ) : (
              behandlungen.map((behandlung) => (
                <TableRow key={behandlung.id}>
                  <TableCell className="font-medium">
                    {behandlung.name}
                  </TableCell>
                  <TableCell>{behandlung.standardpreis} €</TableCell>
                  <TableCell>{behandlung.mwst}</TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      ) : null}
    </div>
  );
}

function TabButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "rounded-lg px-4 py-2 text-sm font-medium transition-colors",
        active
          ? "bg-emerald-600 text-white"
          : "bg-white text-zinc-700 hover:bg-zinc-100 dark:bg-zinc-900 dark:text-zinc-200 dark:hover:bg-zinc-800",
      )}
    >
      {children}
    </button>
  );
}
