"use client";

import {
  CheckCircle2,
  ChevronDown,
  ChevronUp,
  Mail,
  MoreVertical,
  PawPrint,
  Phone,
  TriangleAlert,
} from "lucide-react";
import Link from "next/link";
import { useEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import {
  useAlleKlientenQuery,
  useAlleLeistungenQuery,
  useAlleRechnungenQuery,
} from "@/api/hooks";
import type { Klient, Leistung, Rechnung } from "@/api/types";
import { HaustierCreateForm } from "@/components/klient/haustier-create-form";
import {
  KlientLeistungenSection,
  KlientLeistungenSummaryBadges,
  summarizeKlientLeistungen,
} from "@/components/klient/klient-leistungen";
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
  const leistungenQuery = useAlleLeistungenQuery();
  const rechnungenQuery = useAlleRechnungenQuery();
  const [filter, setFilter] = useState("");

  const rechnungenById = useMemo(() => {
    return new Map(
      (rechnungenQuery.data ?? []).map((rechnung) => [rechnung.id, rechnung]),
    );
  }, [rechnungenQuery.data]);

  const leistungenByKlientId = useMemo(() => {
    const grouped = new Map<string, Leistung[]>();
    for (const leistung of leistungenQuery.data ?? []) {
      const existing = grouped.get(leistung.klientId) ?? [];
      existing.push(leistung);
      grouped.set(leistung.klientId, existing);
    }
    return grouped;
  }, [leistungenQuery.data]);

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

  if (
    klientenQuery.isPending ||
    leistungenQuery.isPending ||
    rechnungenQuery.isPending
  ) {
    return <p className="text-sm text-zinc-500">Lade Klienten…</p>;
  }

  if (klientenQuery.error || leistungenQuery.error || rechnungenQuery.error) {
    return (
      <Alert variant="error">
        {String(
          klientenQuery.error ?? leistungenQuery.error ?? rechnungenQuery.error,
        )}
      </Alert>
    );
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
              <TableHeader>Leistungen</TableHeader>
              <TableHeader className="w-12" />
              <TableHeader className="w-12" />
            </TableRow>
          </TableHead>
          <TableBody>
            {filtered.map((klient) => (
              <KlientTableRow
                key={klient.id}
                klient={klient}
                leistungen={leistungenByKlientId.get(klient.id) ?? []}
                rechnungenById={rechnungenById}
              />
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
}

function KlientTableRow({
  klient,
  leistungen,
  rechnungenById,
}: {
  klient: Klient;
  leistungen: Leistung[];
  rechnungenById: Map<string, Rechnung>;
}) {
  const [expanded, setExpanded] = useState(false);
  const [showHaustierForm, setShowHaustierForm] = useState(false);

  const leistungenSummary = useMemo(
    () => summarizeKlientLeistungen(leistungen, rechnungenById),
    [leistungen, rechnungenById],
  );

  function openHaustierForm() {
    setExpanded(true);
    setShowHaustierForm(true);
  }

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
          <KlientLeistungenSummaryBadges summary={leistungenSummary} />
        </TableCell>
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
        <TableCell className="relative">
          <RowActionsMenu
            onHaustierAnlegen={openHaustierForm}
            klientId={klient.id}
          />
        </TableCell>
      </TableRow>

      {expanded ? (
        <TableRow className="bg-zinc-50 dark:bg-zinc-900/40">
          <TableCell colSpan={7} className="overflow-visible">
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
                <div className="mb-2 flex items-center justify-between gap-2">
                  <p className="text-xs font-semibold tracking-wide text-zinc-500 uppercase">
                    Haustiere
                  </p>
                  {!showHaustierForm ? (
                    <button
                      type="button"
                      onClick={() => setShowHaustierForm(true)}
                      className="inline-flex items-center gap-1 text-xs font-medium text-emerald-700 hover:underline dark:text-emerald-300"
                    >
                      <PawPrint className="size-3.5" />
                      Haustier anlegen
                    </button>
                  ) : null}
                </div>
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

              <div>
                <p className="mb-2 text-xs font-semibold tracking-wide text-zinc-500 uppercase">
                  Leistungen
                </p>
                <KlientLeistungenSection
                  klientId={klient.id}
                  leistungen={leistungen}
                  rechnungenById={rechnungenById}
                  compact
                />
              </div>

              {showHaustierForm ? (
                <HaustierCreateForm klientId={klient.id} embedded />
              ) : null}
            </div>
          </TableCell>
        </TableRow>
      ) : null}
    </>
  );
}

type RowActionsMenuProps = {
  klientId: string;
  onHaustierAnlegen: () => void;
};

function RowActionsMenu({ klientId, onHaustierAnlegen }: RowActionsMenuProps) {
  const [open, setOpen] = useState(false);
  const [menuStyle, setMenuStyle] = useState({ top: 0, left: 0 });
  const buttonRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!open) return;

    function closeOnScroll() {
      setOpen(false);
    }

    window.addEventListener("scroll", closeOnScroll, true);
    window.addEventListener("resize", closeOnScroll);
    return () => {
      window.removeEventListener("scroll", closeOnScroll, true);
      window.removeEventListener("resize", closeOnScroll);
    };
  }, [open]);

  function toggleMenu() {
    if (!open && buttonRef.current) {
      const rect = buttonRef.current.getBoundingClientRect();
      setMenuStyle({
        top: rect.bottom + 4,
        left: Math.max(8, rect.right - 176),
      });
    }
    setOpen((value) => !value);
  }

  function close() {
    setOpen(false);
  }

  return (
    <>
      <button
        ref={buttonRef}
        type="button"
        aria-expanded={open}
        aria-haspopup="menu"
        onClick={toggleMenu}
        className="flex size-8 items-center justify-center rounded-md hover:bg-zinc-100 dark:hover:bg-zinc-900"
      >
        <MoreVertical className="size-4" />
      </button>

      {open && typeof document !== "undefined"
        ? createPortal(
            <>
              <button
                type="button"
                aria-label="Menü schließen"
                className="fixed inset-0 z-40 cursor-default"
                onClick={close}
              />
              <div
                role="menu"
                className="fixed z-50 w-44 rounded-lg border border-zinc-200 bg-white p-1 shadow-lg dark:border-zinc-700 dark:bg-zinc-900"
                style={{ top: menuStyle.top, left: menuStyle.left }}
              >
                <Link
                  href={paths.klient(klientId)}
                  role="menuitem"
                  className="block rounded-md px-3 py-2 text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800"
                  onClick={close}
                >
                  Akte öffnen
                </Link>
                <button
                  type="button"
                  role="menuitem"
                  className="block w-full rounded-md px-3 py-2 text-left text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800"
                  onClick={() => {
                    close();
                    onHaustierAnlegen();
                  }}
                >
                  Haustier anlegen
                </button>
                <Link
                  href={paths.leistungForKlient(klientId)}
                  role="menuitem"
                  className="block rounded-md px-3 py-2 text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800"
                  onClick={close}
                >
                  Leistung buchen
                </Link>
              </div>
            </>,
            document.body,
          )
        : null}
    </>
  );
}
