# YAMS Hexagonal Backend Specification

> **Hinweis:** Domain-Sprache ist **Deutsch** (UTF-8). Dieses Dokument kann hinter dem Code liegen — Implementation in `crates/` und [`abrechnung.md`](abrechnung.md) ist maßgeblich.

## Core Domain (`yams-core`)

- **Entities**: `Klient`, `Haustier`, `Adresse`, `Produkt`, `Behandlung`, `Leistung`, `Rechnung`, `Seminar`, `SeminarTermin` (siehe [`abrechnung.md`](abrechnung.md), [`seminar.md`](seminar.md)).
- **Geplant**: `Beziehung`, `Rasse`.
- **Value Objects**: `Preis`, `Ratio`, `Zeitraum`, `EmailAdresse`, `Mobilnummer`, `Ländercode`, Status-Enums.
- **Ports**: Repository pro Aggregate + `Clock`.
- **Use Cases**: Ein Use Case pro Geschäftsvorgang (`KlientErstellen`, `TagesabschlussDurchführen`, …).
- **App**: Composition Root; alle Mutationen via `App::execute`.
- **Error Handling**: `thiserror` in Domain/Use Cases, `error_stack::Report` an Grenzen.

## API Layer (`yams-api`)

- **Purpose**: `YamsAppApi`, schema DTOs, OpenAPI — framework-agnostisch.
- **Naming**: Deutsche Feldnamen, JSON `camelCase` (z. B. `vorName`, `ländercode`).
- **Spec Export**: Binary `export_spec` druckt OpenAPI nach stdout.

## Persistence (`yams-persistence`)

- **Driver**: `libsql` (SQLite).
- **Schema**: SQL-Migrationen via `molting` in `migrations/`.
- **Mappers**: Manuelles Parsen/Speichern, kein ORM.

## Standalone Server (`yams-server`)

- **Framework**: `poem-openapi`.
- **Routes**: `/klient`, `/haustier`, `/produkt`, `/behandlung`, `/leistung`, `/rechnung`, `/tagesabschluss`, `/seminar`, `/seminar-termin`, `/seminar-prognose`.

## Tauri Embedded (`frontend/src-tauri`)

- **Adapters**: Tauri commands → `YamsAppApi`.
- **Features**: `yams-api` mit `serde` only (kein poem).

## Communication Bridge (Frontend)

- **BackendClient** in TypeScript.
- **HttpAdapter** / **TauriAdapter**.
- **TanStack Query** für State.
