"use client";

import { type FormEvent, useEffect, useMemo, useState } from "react";

import {
  useAlleBehandlungenQuery,
  useAlleKlientenQuery,
  useAlleProdukteQuery,
  useLeistungAusBehandlungBuchenMutation,
  useLeistungAusProduktBuchenMutation,
} from "@/api/hooks";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { todayIsoDate } from "@/lib/dates";

type LeistungQuelle = "produkt" | "behandlung";

type LeistungBuchenFormProps = {
  initialKlientId?: string | null;
};

export function LeistungBuchenForm({
  initialKlientId = null,
}: LeistungBuchenFormProps) {
  const klientenQuery = useAlleKlientenQuery();
  const produkteQuery = useAlleProdukteQuery();
  const behandlungenQuery = useAlleBehandlungenQuery();
  const produktMutation = useLeistungAusProduktBuchenMutation();
  const behandlungMutation = useLeistungAusBehandlungBuchenMutation();

  const [klientId, setKlientId] = useState(initialKlientId ?? "");
  const [haustierId, setHaustierId] = useState("");
  const [quelle, setQuelle] = useState<LeistungQuelle>("produkt");
  const [produktId, setProduktId] = useState("");
  const [behandlungId, setBehandlungId] = useState("");
  const [leistungsdatum, setLeistungsdatum] = useState(todayIsoDate());
  const [menge, setMenge] = useState("1");
  const [success, setSuccess] = useState<string | null>(null);

  useEffect(() => {
    if (initialKlientId) {
      setKlientId(initialKlientId);
    }
  }, [initialKlientId]);

  const klient = useMemo(
    () => (klientenQuery.data ?? []).find((k) => k.id === klientId),
    [klientId, klientenQuery.data],
  );

  const haustiere = klient?.haustiere ?? [];
  const produkte = produkteQuery.data ?? [];
  const behandlungen = behandlungenQuery.data ?? [];

  useEffect(() => {
    if (produkte.length > 0 && !produktId) {
      setProduktId(produkte[0]?.id ?? "");
    }
  }, [produkte, produktId]);

  useEffect(() => {
    if (behandlungen.length > 0 && !behandlungId) {
      setBehandlungId(behandlungen[0]?.id ?? "");
    }
  }, [behandlungen, behandlungId]);

  useEffect(() => {
    if (haustierId && !haustiere.some((h) => h.id === haustierId)) {
      setHaustierId("");
    }
  }, [haustierId, haustiere]);

  const mutation = quelle === "produkt" ? produktMutation : behandlungMutation;
  const isLoading =
    klientenQuery.isPending ||
    produkteQuery.isPending ||
    behandlungenQuery.isPending;

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!klientId) return;
    setSuccess(null);

    const haustierRef = haustierId || undefined;

    if (quelle === "produkt" && produktId) {
      const leistung = await produktMutation.mutateAsync({
        klientId,
        produktId,
        haustierId: haustierRef,
        menge,
        leistungsdatum,
      });
      setSuccess(
        `Leistung gebucht: ${leistung.beschreibung} (${leistung.betrag} €)`,
      );
      return;
    }

    if (quelle === "behandlung" && behandlungId) {
      const leistung = await behandlungMutation.mutateAsync({
        klientId,
        behandlungId,
        haustierId: haustierRef,
        leistungsdatum,
      });
      setSuccess(
        `Leistung gebucht: ${leistung.beschreibung} (${leistung.betrag} €)`,
      );
    }
  }

  return (
    <form className="space-y-4" onSubmit={handleSubmit}>
      <div className="grid gap-4 sm:grid-cols-2">
        <Field label="Klient">
          <Select
            value={klientId}
            onChange={(e) => setKlientId(e.target.value)}
            required
          >
            <option value="">— wählen —</option>
            {(klientenQuery.data ?? []).map((k) => (
              <option key={k.id} value={k.id}>
                {k.nachname}, {k.vorname} (KNr {k.kundennummer})
              </option>
            ))}
          </Select>
        </Field>

        <Field label="Haustier (optional)">
          <Select
            value={haustierId}
            onChange={(e) => setHaustierId(e.target.value)}
            disabled={!klient || haustiere.length === 0}
          >
            <option value="">— keins —</option>
            {haustiere.map((h) => (
              <option key={h.id} value={h.id}>
                {h.name} ({h.tierart})
              </option>
            ))}
          </Select>
        </Field>

        <Field label="Art">
          <Select
            value={quelle}
            onChange={(e) => setQuelle(e.target.value as LeistungQuelle)}
          >
            <option value="produkt">Produkt</option>
            <option value="behandlung">Behandlung</option>
          </Select>
        </Field>

        <Field label="Leistungsdatum">
          <Input
            type="date"
            value={leistungsdatum}
            onChange={(e) => setLeistungsdatum(e.target.value)}
            required
          />
        </Field>

        {quelle === "produkt" ? (
          <>
            <Field label="Produkt">
              <Select
                value={produktId}
                onChange={(e) => setProduktId(e.target.value)}
                required
                disabled={produkte.length === 0}
              >
                {produkte.map((p) => (
                  <option key={p.id} value={p.id}>
                    {p.name} ({p.einzelpreis} €)
                  </option>
                ))}
              </Select>
            </Field>
            <Field label="Menge">
              <Input
                value={menge}
                onChange={(e) => setMenge(e.target.value)}
                required
              />
            </Field>
          </>
        ) : (
          <Field label="Behandlung" className="sm:col-span-2">
            <Select
              value={behandlungId}
              onChange={(e) => setBehandlungId(e.target.value)}
              required
              disabled={behandlungen.length === 0}
            >
              {behandlungen.map((b) => (
                <option key={b.id} value={b.id}>
                  {b.name} ({b.standardpreis} €)
                </option>
              ))}
            </Select>
          </Field>
        )}
      </div>

      {isLoading ? (
        <p className="text-sm text-zinc-500">Lade Stammdaten…</p>
      ) : null}

      {quelle === "produkt" && produkte.length === 0 ? (
        <Alert variant="info">Zuerst ein Produkt im Katalog anlegen.</Alert>
      ) : null}
      {quelle === "behandlung" && behandlungen.length === 0 ? (
        <Alert variant="info">Zuerst eine Behandlung im Katalog anlegen.</Alert>
      ) : null}
      {!klientId ? (
        <Alert variant="info">Bitte einen Klienten wählen.</Alert>
      ) : null}

      {success ? <Alert variant="success">{success}</Alert> : null}
      {mutation.error ? (
        <Alert variant="error">{String(mutation.error)}</Alert>
      ) : null}

      <Button
        type="submit"
        disabled={
          !klientId ||
          isLoading ||
          mutation.isPending ||
          (quelle === "produkt" && produkte.length === 0) ||
          (quelle === "behandlung" && behandlungen.length === 0)
        }
      >
        {mutation.isPending ? "Buchen…" : "Leistung buchen"}
      </Button>
    </form>
  );
}
