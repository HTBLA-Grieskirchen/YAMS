# Abrechnung — Domain-Spezifikation

Ubiquitous Language: **Deutsch** in Code, API-JSON und SQLite-Spalten (UTF-8, z. B. `Ländercode`).

## Aggregates

| Aggregate | Beschreibung |
|-----------|--------------|
| `Klient` | Kunde / Patientenverwaltung |
| `Haustier` | Tier eines Klienten (`klient_id` Pflicht) |
| `Adresse` | Wertobjekt, in `Klient` eingebettet |
| `Produkt` | Katalog-Eintrag (Name, Beschreibung, Einzelpreis) |
| `Behandlung` | Katalog-Eintrag (Name, Beschreibung, Standardpreis) |
| `Leistung` | Offener Posten / abrechenbare Position |
| `Rechnung` | Rechnung mit `Rechnungspositionen` |

## Wertobjekte

| Typ | Invarianten |
|-----|-------------|
| `Preis` | `rust_decimal`, nicht negativ |
| `EmailAdresse` | Validierung bei Konstruktion |
| `Mobilnummer` | Validierung bei Konstruktion |
| `Ländercode` | ISO 3166-1 alpha-2 |
| `LeistungStatus` | `Offen` \| `Abgerechnet` |
| `RechnungStatus` | `Offen` \| `Bezahlt` |
| `LeistungQuelle` | `Produkt(ProduktId)` \| `Behandlung(BehandlungId)` \| `Manuell` |

## Use Cases

| Use Case | Zweck |
|----------|-------|
| `KlientErstellen` | Neuen Klient anlegen |
| `HaustierErstellen` | Haustier für Klient anlegen |
| `ProduktErstellen` | Katalog-Produkt anlegen |
| `BehandlungErstellen` | Katalog-Behandlung anlegen |
| `LeistungAusProduktBuchen` | Leistung aus Produkt × Menge |
| `LeistungAusBehandlungBuchen` | Leistung aus Behandlung |
| `LeistungManuellErfassen` | Leistung mit Betrag + Beschreibung |
| `TagesabschlussDurchfuehren` | Offene Leistungen eines Tages → Rechnungen pro Klient |

## Invarianten

1. Nur `Leistung` mit Status `Offen` kann in Tagesabschluss einfließen.
2. `Abgerechnet` Leistungen sind unveränderlich (`rechnung_id` gesetzt).
3. `Rechnung` wird in v1 nur via `TagesabschlussDurchfuehren` erzeugt (kein ad-hoc Anlegen).
4. `Haustier` gehört genau einem `Klient` (`klient_id` Pflicht); keine `haustier_ids` auf `Klient`.
5. `Bezahlt` auf Rechnung wird v1 manuell gesetzt (kein Zahlungsgateway).

## Tagesabschluss-Ablauf

1. `abschlussdatum` = Eingabe oder `Clock::today()`
2. Alle `Offen` Leistungen mit `leistungsdatum = abschlussdatum` laden
3. Nach `klient_id` gruppieren
4. Pro Gruppe: `Rechnung::aus_leistungen` → persistieren → Leistungen `Abgerechnet` markieren
5. `checkpoint()` zwischen Klient-Gruppen (wie `VieleHaustiereErstellen`)

## Stub-Umfang

`Produkt` und `Behandlung` sind Katalog-Stubs — kein Lager, keine klinische Dokumentation.

## Geplant (nicht in diesem Slice)

`Veranstaltung`, `Teilnahme`, `Beziehung`, `Rasse`, PDF-Export, Zahlungsabwicklung.
