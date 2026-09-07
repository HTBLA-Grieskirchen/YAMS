"use client";

import { useMemo, useState } from "react";

import {
  useAlleBehandlungenQuery,
  useAlleProdukteQuery,
  useAlleSeminareQuery,
} from "@/api/hooks";
import { BehandlungCreateForm } from "@/components/katalog/behandlung-create-form";
import { ProduktCreateForm } from "@/components/katalog/produkt-create-form";
import { SeminarCreateForm } from "@/components/katalog/seminar-create-form";
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
import { cn } from "@/lib/cn";
import { matchesSearchQuery } from "@/lib/format";

type KatalogTab = "produkte" | "behandlungen" | "seminare";

export function KatalogOverview() {
  const [tab, setTab] = useState<KatalogTab>("produkte");
  const [filter, setFilter] = useState("");
  const produkteQuery = useAlleProdukteQuery();
  const behandlungenQuery = useAlleBehandlungenQuery();
  const seminareQuery = useAlleSeminareQuery();

  const produkte = useMemo(
    () =>
      (produkteQuery.data ?? []).filter((p) =>
        matchesSearchQuery(filter, [
          p.name,
          p.beschreibung,
          p.einzelpreis,
          p.mwst,
        ]),
      ),
    [filter, produkteQuery.data],
  );

  const behandlungen = useMemo(
    () =>
      (behandlungenQuery.data ?? []).filter((b) =>
        matchesSearchQuery(filter, [
          b.name,
          b.beschreibung,
          b.standardpreis,
          b.mwst,
        ]),
      ),
    [filter, behandlungenQuery.data],
  );

  const seminare = useMemo(
    () =>
      (seminareQuery.data ?? []).filter((s) =>
        matchesSearchQuery(filter, [
          s.titel,
          s.beschreibung,
          s.teilnahmegebührBasis,
          s.mwst,
        ]),
      ),
    [filter, seminareQuery.data],
  );

  const isLoading =
    tab === "produkte"
      ? produkteQuery.isPending
      : tab === "behandlungen"
        ? behandlungenQuery.isPending
        : seminareQuery.isPending;

  const error =
    tab === "produkte"
      ? produkteQuery.error
      : tab === "behandlungen"
        ? behandlungenQuery.error
        : seminareQuery.error;

  return (
    <div className="space-y-6 p-6">
      <div className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">Katalog</h1>
        <p className="text-sm text-zinc-600 dark:text-zinc-400">
          Produkte, Behandlungen und Seminar-Stammdaten anlegen und verwalten.
        </p>
      </div>

      <div className="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
        <div className="flex flex-wrap gap-2">
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
          <TabButton
            active={tab === "seminare"}
            onClick={() => setTab("seminare")}
          >
            Seminare
          </TabButton>
        </div>
        <SearchInput
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          placeholder="Katalog durchsuchen…"
        />
      </div>

      <div className="grid gap-6 xl:grid-cols-[minmax(0,1fr)_22rem]">
        <div className="space-y-4">
          {isLoading ? <p className="text-sm text-zinc-500">Lade…</p> : null}
          {error ? <Alert variant="error">{String(error)}</Alert> : null}

          {tab === "produkte" && !isLoading && !error ? (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader>Name</TableHeader>
                  <TableHeader>Beschreibung</TableHeader>
                  <TableHeader>Preis (netto)</TableHeader>
                  <TableHeader>MwSt</TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {produkte.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={4} className="text-zinc-500">
                      Keine Produkte.
                    </TableCell>
                  </TableRow>
                ) : (
                  produkte.map((p) => (
                    <TableRow key={p.id}>
                      <TableCell className="font-medium">{p.name}</TableCell>
                      <TableCell>{p.beschreibung}</TableCell>
                      <TableCell>{p.einzelpreis} €</TableCell>
                      <TableCell>{p.mwst}</TableCell>
                    </TableRow>
                  ))
                )}
              </TableBody>
            </Table>
          ) : null}

          {tab === "behandlungen" && !isLoading && !error ? (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader>Name</TableHeader>
                  <TableHeader>Beschreibung</TableHeader>
                  <TableHeader>Standardpreis</TableHeader>
                  <TableHeader>MwSt</TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {behandlungen.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={4} className="text-zinc-500">
                      Keine Behandlungen.
                    </TableCell>
                  </TableRow>
                ) : (
                  behandlungen.map((b) => (
                    <TableRow key={b.id}>
                      <TableCell className="font-medium">{b.name}</TableCell>
                      <TableCell>{b.beschreibung}</TableCell>
                      <TableCell>{b.standardpreis} €</TableCell>
                      <TableCell>{b.mwst}</TableCell>
                    </TableRow>
                  ))
                )}
              </TableBody>
            </Table>
          ) : null}

          {tab === "seminare" && !isLoading && !error ? (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader>Titel</TableHeader>
                  <TableHeader>Beschreibung</TableHeader>
                  <TableHeader>Gebühr (netto)</TableHeader>
                  <TableHeader>MwSt</TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {seminare.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={4} className="text-zinc-500">
                      Keine Seminare.
                    </TableCell>
                  </TableRow>
                ) : (
                  seminare.map((s) => (
                    <TableRow key={s.id}>
                      <TableCell className="font-medium">{s.titel}</TableCell>
                      <TableCell>{s.beschreibung}</TableCell>
                      <TableCell>{s.teilnahmegebührBasis} €</TableCell>
                      <TableCell>{s.mwst}</TableCell>
                    </TableRow>
                  ))
                )}
              </TableBody>
            </Table>
          ) : null}
        </div>

        <div>
          {tab === "produkte" ? <ProduktCreateForm /> : null}
          {tab === "behandlungen" ? <BehandlungCreateForm /> : null}
          {tab === "seminare" ? <SeminarCreateForm /> : null}
        </div>
      </div>
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
