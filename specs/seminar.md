# Seminar — Domain-Spezifikation

Ubiquitous Language: **Deutsch** in Code, API-JSON und SQLite-Spalten (UTF-8).

## Aggregates

| Typ | Name | Rolle |
|-----|------|-------|
| Aggregate (Root) | `Seminar` | Vorlage: Titel, Beschreibung, `teilnahmegebühr_basis`, `mwst`, optional `standarddauer` |
| Aggregate (Root) | `SeminarTerminIn<S>` | Konkreter Termin; enthält Entities `SeminarBuchung` |
| Entity | `SeminarBuchung` | Eigene `SeminarBuchungId`; **kein** eigenes Repository |

Repository-Grenze: nur `Seminar` und `SeminarTermin`. Buchungen werden mit dem Termin geladen und gespeichert.

## Type-State `SeminarTermin`

| Zustand | Bedeutung |
|---------|-----------|
| `Geplant` | Buchungen offen; Ort/Zeitraum änderbar |
| `Abgehalten { abgehalten_am, leistungen }` | Bestätigte Buchungen haben `LeistungOffen`; Mapping `HashMap<SeminarBuchungId, LeistungId>` |
| `Abgesagt { abgesagt_am, grund }` | Archiviert, keine Leistungen |

Enum `SeminarTermin` rekonstruiert persistierten Zustand via `from_parts`. **Kein hard delete.**

`SeminarTerminIn<Abgehalten>::leistung_fuer_buchung` ist der type-safe Lookup.

## Entity `SeminarBuchung`

| Feld | Typ |
|------|-----|
| `id` | `SeminarBuchungId` |
| `klient_id` | `KlientId` |
| `rabatt` | `Ratio` (Default `zero`) |
| Status | `Bestätigt` \| `Storniert { storniert_am }` |

Kein `leistung_id` auf der Entity — Zuordnung nur im `Abgehalten`-State.

## Wertobjekte

| Typ | Invarianten |
|-----|-------------|
| `Zeitraum` | `ende > beginn` (`DateTime<Utc>`); Konstruktion als `Result<Self, ZeitraumFehler>`, kein `Report` |
| `Ratio` | `0..=1`; Rabatt und MwSt |
| `Preis::nach_rabatt` | `basis * (1 - rabatt)`; infallible, bei 100% → `0` |
| `SeminarOrt` | optional `ort_name` und/oder `adresse` |

## Invarianten

1. Buchung nur bei Termin `Geplant`.
2. `max_teilnehmer` (falls `Some`) zählt nur bestätigte Buchungen.
3. Ein Klient höchstens eine Buchung pro Termin (auch stornierte blockieren keine neue? — **stornierte zählen nicht**; neuer Versuch nach Storno ist erlaubt).
4. Stornieren nur `Bestätigt` auf `Geplant`.
5. Absagen und `aktualisieren` nur `Geplant`.
6. `als_abgehalten` nur `Geplant`; Mapping muss genau die bestätigten Buchungen abdecken. Wiederholung = Fehler (Termin nicht mehr `Geplant`).
7. Stornierte Buchungen erzeugen keine Leistung.

## Abgehalten-Ablauf

1. Termin muss `Geplant` sein.
2. Pro bestätigter Buchung: `LeistungOffen` mit `LeistungQuelle::Seminar` (`basis` + `rabatt`; Betrag JIT).
3. `leistungsdatum` = `zeitraum.ende` als Datum.
4. Transition `Geplant → Abgehalten` mit vollständigem Mapping.
5. `TagesabschlussDurchführen` übernimmt die offenen Leistungen — kein eigener Rechnungs-Use-Case.

## Use Cases

| Use Case | Zweck |
|----------|-------|
| `SeminarErstellen` | Vorlage anlegen |
| `SeminarTerminPlanen` | Termin aus Vorlage, leere Buchungen; liefert `SeminarTerminGeplant` |
| `SeminarTerminAktualisieren` | Ort/Zeitraum/max TN — nur `Geplant`; Benachrichtigung TODO |
| `SeminarBuchungAnlegen` | Aggregate-Methode + `update` |
| `SeminarBuchungStornieren` | `Bestätigt → Storniert` |
| `SeminarTerminAbsagen` | `Geplant → Abgesagt` |
| `SeminarTerminAlsAbgehaltenMarkieren` | Leistungen persistieren + Statuswechsel |
| `SeminarUmsatzVorschau` | Dry-run Umsatz eines Termins |
| `SeminarUmsatzPrognoseBisDatum` | Alle noch nicht vollständig abgerechneten Termine bis Stichtag |

## Prognose

- **Geplant**: Summe bestätigter Buchungen (`basis.nach_rabatt(rabatt)`).
- **Abgehalten**: Summe zugehöriger `LeistungOffen` (Tagesabschluss ausstehend).
- **Abgesagt**: ausgeschlossen.

## Persistence

Tabellen `seminar`, `seminar_termin`, `seminar_buchung` (Child, inkl. optionaler `leistung_id` für das Abgehalten-Mapping). Kein `SeminarBuchungRepository`.
