# Abrechnung — Domain-Spezifikation

Ubiquitous Language: **Deutsch** in Code, API-JSON und SQLite-Spalten (UTF-8, z. B. `Ländercode`).

## Aggregates (Type-State)

| Aggregate | Zustände | Beschreibung |
|-----------|----------|--------------|
| `Leistung<S>` | `Offen` → `Abgerechnet` | Offener Posten; nur `LeistungOffen::mark_abgerechnet` transitioniert |
| `Rechnung<S>` | `Offen` → `Bezahlt` | Rechnung mit `Rechnungspositionen` |
| `Klient`, `Haustier`, `Produkt`, `Behandlung` | — | Stammdaten / Katalog |

Repository-Grenze: `GeladeneLeistung` / `GeladeneRechnung` rekonstruieren persistierten Zustand.

## Wertobjekte

| Typ | Invarianten |
|-----|-------------|
| `Preis` | `rust_decimal`, nicht negativ; `add` / `multiply` erhalten Invariante |
| `EmailAdresse`, `Mobilnummer` | Validierung bei Konstruktion (`Result`, kein `Report`) |
| `Ländercode` | Enum `AT` \| `DE` \| `CH` |
| `LeistungQuelle` | `Produkt { produkt_id, menge, einzelpreis }` \| `Behandlung { behandlung_id, preis }` \| `Manuell { preis }` — Preis-Snapshot zum Buchungszeitpunkt |
| `Rechnungsposition` | `einzelpreis`, `stückzahl`, `mwst_prozentsatz` → Getter `gesamtpreis_netto`, `mwst_betrag`, `gesamtpreis_brutto` |

`Leistung::betrag()` und `Rechnung::gesamtbetrag_brutto()` sind berechnete Getter, keine gespeicherten Felder.

## Use Cases

| Use Case | Zweck |
|----------|-------|
| `ProduktErstellen` / `BehandlungErstellen` | Katalog anlegen |
| `LeistungAusProduktBuchen` | Leistung mit Preis-Snapshot (Produkt × Menge) |
| `LeistungAusBehandlungBuchen` | Leistung mit Standard- oder Override-Preis |
| `LeistungManuellErfassen` | Sonstige Leistung mit manuellem Preis |
| `TagesabschlussDurchfuehren` | Offene Leistungen eines Tages → Rechnungen pro Klient |

## Invarianten

1. Nur `Leistung<Offen>` fließt in Tagesabschluss ein (type-state, kein Laufzeit-Status-Check).
2. `RechnungOffen::aus_leistungen` markiert Leistungen in-place als `Abgerechnet` — kein separates Repository-`mark_abgerechnet`.
3. Repositories sind dumb: `update` persistiert mutierte Domain-Entitäten.
4. `Rechnung` v1 nur via `TagesabschlussDurchfuehren`.
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

`Veranstaltung`, `Teilnahme`, `Beziehung`, `Rasse`, PDF-Export, Zahlungsabwicklung.
