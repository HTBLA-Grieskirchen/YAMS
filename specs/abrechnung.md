# Abrechnung — Domain-Spezifikation

Ubiquitous Language: **Deutsch** in Code, API-JSON und SQLite-Spalten (UTF-8, z. B. `Ländercode`).

## Aggregates (Type-State)

| Aggregate | Zustände | Beschreibung |
|-----------|----------|--------------|
| `LeistungIn<S>` | `Offen` → `Abgerechnet` | Type-state; enum `Leistung` sums states |
| `RechnungIn<S>` | `Offen` → `Bezahlt` | Type-state; enum `Rechnung` sums states |
| `Klient`, `Haustier`, `Produkt`, `Behandlung`, `Seminar` | — | Stammdaten / Katalog |
| `SeminarTerminIn<S>` | `Geplant` → `Abgehalten` \| `Abgesagt` | siehe [`seminar.md`](seminar.md) |

Repository-Grenze: enum `Leistung` / `Rechnung` rekonstruiert persistierten Zustand via `from_parts`.

## Wertobjekte

| Typ | Invarianten |
|-----|-------------|
| `Preis` | `rust_decimal`, nicht negativ; `Add` und `&Preis * &Menge` / `&Preis * &Ratio` erhalten die Invariante; `nach_rabatt(&Ratio)` ist infallible (`basis * (1 - rabatt)`, 100% → `0`) |
| `Ratio` | `0..=1` (100% = `1`); generischer Anteil (MwSt, Rabatt) |
| `Menge` | nicht negativ, einheitenlos; Produktmenge und Rechnungs-`stückzahl` |
| `EmailAdresse`, `Mobilnummer` | Validierung bei Konstruktion (`Result`, kein `Report`) |
| `Ländercode` | Enum `AT` \| `DE` \| `CH` |
| `LeistungQuelle` | `Produkt { … }` \| `Behandlung { … }` \| `Manuell { … }` \| `Seminar { termin_id, buchung_id, teilnahmegebühr_basis, rabatt, mwst }` — `betrag()` = `basis.nach_rabatt(rabatt)` (JIT, nicht persistiert) |
| `Rechnungsposition` | `einzelpreis`, `stückzahl`, `mwst` (`Ratio`) → Getter `gesamtpreis_netto`, `mwst_betrag`, `gesamtpreis_brutto` |

`Leistung::betrag()` und `Rechnung::gesamtbetrag_brutto()` sind berechnete Getter, keine gespeicherten Felder.

## Use Cases

| Use Case | Zweck |
|----------|-------|
| `ProduktErstellen` / `BehandlungErstellen` | Katalog anlegen |
| `LeistungAusProduktBuchen` | Leistung mit Preis-Snapshot (Produkt × Menge) |
| `LeistungAusBehandlungBuchen` | Leistung mit Standard- oder Override-Preis |
| `LeistungManuellErfassen` | Sonstige Leistung mit manuellem Preis |
| `TagesabschlussDurchführen` | Offene Leistungen eines Tages → Rechnungen pro Klient |

## Invarianten

1. Nur `LeistungOffen` (`LeistungIn<Offen>`) fließt in Tagesabschluss ein.
2. `RechnungOffen::aus_leistungen` markiert Leistungen in-place als `Abgerechnet` — kein separates Repository-`mark_abgerechnet`.
3. Repositories sind dumb: `update` persistiert mutierte Domain-Entitäten.
4. `Rechnung` v1 nur via `TagesabschlussDurchführen`.
5. `Haustier.klient_id` Pflicht; keine `haustier_ids` auf `Klient`.

## Tagesabschluss-Ablauf

1. `abschlussdatum` = Eingabe oder `Clock::today()`
2. `LeistungOffen` für Datum laden
3. Nach `klient_id` gruppieren
4. Pro Gruppe: `RechnungOffen::aus_leistungen(&mut leistungen)` → Rechnung + `LeistungAbgerechnet` persistieren via `update`
5. `checkpoint()` zwischen Klient-Gruppen

## Stub-Umfang

`Produkt` und `Behandlung` sind Katalog-Stubs — kein Lager, keine klinische Dokumentation.

## Geplant (nicht in diesem Slice)

`Beziehung`, `Rasse`, PDF-Export, Zahlungsabwicklung. Seminare: siehe [`seminar.md`](seminar.md).
