# Graph Report - yams  (2026-09-03)

## Corpus Check
- 272 files · ~336,376 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3006 nodes · 7305 edges · 154 communities (124 shown, 21 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 227 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e6a5fe99`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .add_dyn
- arc_up
- devDependencies
- layout.tsx
- yams-api
- Klient
- YAMSFrontendConfig
- EventForm.tsx
- String
- termin_from_parts
- yams-filesystemstore/src/lib.rs
- UpMigration
- compilerOptions
- notification.ts
- TypicalJsonResponse
- Behandlung
- produkt_from_row
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- RepositoryResult
- stores/index.tsx
- tests.rs
- LeistungOffen
- makeRecordForTable
- query
- relations/index.tsx
- .begin
- paths.ts
- EmailAdresse
- AddressTable.tsx
- RechnungId
- Klient
- NeueLeistung
- PdfDokument
- in_memory_object_store.rs
- ResultReport
- FixedClock
- .get_current_version
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- dialog.ts
- FakeDatastore
- NotificationType
- RepoStorage
- SeminarTerminId
- leistung_from_row
- Klientbericht
- base_app_builder
- Seminar
- HaustierErstellung
- Produkt
- Ratio
- api/client.ts
- StructuredError
- repos.rs
- DatabaseConnection
- StreamBinaryResponse
- base_app_builder
- schema/seminar.rs
- Preis
- HaustierErstellen
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- src/api/schema.d.ts
- yams-core
- leistung-form.tsx
- Seminar — Domain-Spezifikation
- Adresse
- base_app_builder
- BackendClient
- diffx-finish-review
- index.d.ts
- Menge
- Next.js
- postcss.config.mjs
- api/types.ts
- Built on SurrealDB
- YAMS Banner SVG
- YAMS Logo SVG
- yams-typstreports/src/lib.rs
- common.rs
- use_cases/seminar.rs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- SQLiteInstance
- Clock
- UnitOfWork
- hooks/index.ts
- instrumented.rs
- useStore
- .behandlung_erstellen
- StatusCode
- Klient
- .produkt_erstellen
- page.tsx
- InstrumentedUnitOfWork
- KlientId
- api/index.ts
- HttpYamsApi
- KlientErstellen
- ports/object_store.rs
- .finish
- KlientErstellung
- Rechnung
- teilnahme_pdf_laden
- v0004_leistungen_quelle_mwst.rs
- src/errors.rs
- SQLiteHaustierRepository
- SeminarId
- ObjectStore
- SQLiteUnitOfWork
- Migration
- Versioned
- YamsApiSpec
- openapi_service
- validation_error.rs
- LeistungId
- .haustier_by_id
- service/pdf.rs
- bad_request
- StreamBody
- LeistungIn<S>
- InternalServerError
- .tagesabschluss_durchführen
- SQLiteBehandlungRepository
- .clone
- .seminar_umsatz_prognose
- FakeUnitOfWork
- .seminar_umsatz_prognose
- preis.rs
- document_text
- RechnungRepository
- SeminarRepository
- SeminarTerminRepository
- FakeSeminareRepository
- parse_datetime
- BehandlungErstellen

## God Nodes (most connected - your core abstractions)
1. `KlientId` - 74 edges
2. `Versioned` - 70 edges
3. `Preis` - 56 edges
4. `Ratio` - 56 edges
5. `useStore()` - 52 edges
6. `YamsAppApi` - 45 edges
7. `query()` - 42 edges
8. `FakeDatastore` - 41 edges
9. `SeminarTerminId` - 38 edges
10. `E` - 34 edges

## Surprising Connections (you probably didn't know these)
- `Config` --references--> `String`  [EXTRACTED]
  backend/server/src/main.rs → crates/yams-api/src/errors/internal_error.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `TestError` --implements--> `Error`  [EXTRACTED]
  crates/molting/src/tests/common/mod.rs → frontend-legacy/libs/notification.ts
- `main()` --calls--> `openapi_service()`  [INFERRED]
  crates/yams-api/src/bin/export_spec.rs → crates/yams-api/src/spec.rs
- `YAMSBackendConfig` --references--> `String`  [EXTRACTED]
  frontend/src-tauri/src/config.rs → crates/yams-api/src/errors/internal_error.rs

## Import Cycles
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **CI Release Pipeline** — github_workflows_publish_release_workflow, github_workflows_smoke_tests_workflow, github_workflows_unit_tests_workflow, github_workflows_tauri_action, github_workflows_mise_action [EXTRACTED 1.00]
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **Frontend Backend Communication Bridge** — specs_backend_hexagonal_backend_client, specs_backend_hexagonal_http_adapter, specs_backend_hexagonal_tauri_adapter, specs_backend_hexagonal_tanstack_query, specs_frontend_migration_yamsconfig [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (154 total, 21 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (40): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+32 more)

### Community 2 - "devDependencies"
Cohesion: 0.04
Nodes (47): babel-plugin-react-compiler, @biomejs/biome, dependencies, next, openapi-fetch, react, react-dom, @tanstack/react-query (+39 more)

### Community 3 - "layout.tsx"
Cohesion: 0.05
Nodes (37): source, assist, actions, next, react, files, ignoreUnknown, includes (+29 more)

### Community 4 - "yams-api"
Cohesion: 0.06
Nodes (37): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+29 more)

### Community 5 - "Klient"
Cohesion: 0.07
Nodes (37): Energetik Logo (legacy), Backend ER Model, Adresse, Behandlung, Behandlungsart, BehandlungsTermin, Buchungsart, Energetik Sabine Petschl (+29 more)

### Community 6 - "YAMSFrontendConfig"
Cohesion: 0.14
Nodes (16): dev_var_set(), project_dirs(), Arc, Default, From, Option, Self, Send (+8 more)

### Community 7 - "EventForm.tsx"
Cohesion: 0.13
Nodes (29): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+21 more)

### Community 8 - "String"
Cohesion: 0.15
Nodes (36): YamsAppApi, From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen() (+28 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.19
Nodes (23): insert_params(), leistung_id_for(), load_buchungen(), load_termin(), ort_from_columns(), replace_buchungen(), Arc, FxHashMap (+15 more)

### Community 10 - "yams-filesystemstore/src/lib.rs"
Cohesion: 0.14
Nodes (26): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+18 more)

### Community 11 - "UpMigration"
Cohesion: 0.15
Nodes (9): Send, Sync, UpMigration, Migration, Option, Transaction, Migration, Option (+1 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "notification.ts"
Cohesion: 0.18
Nodes (8): NotificationActions, NotificationBehaviour, NotificationContent, NotificationInfo, NotificationInfoType, TODO: Add possibility to also display notification on host system if in Tauri, NotificationEntry, NotificationStore

### Community 14 - "TypicalJsonResponse"
Cohesion: 0.12
Nodes (14): Behandlung, BehandlungErstellung, HaustierErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung (+6 more)

### Community 15 - "Behandlung"
Cohesion: 0.11
Nodes (13): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+5 more)

### Community 16 - "produkt_from_row"
Cohesion: 0.27
Nodes (9): produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row, Transaction (+1 more)

### Community 17 - "apply_up_migrations"
Cohesion: 0.25
Nodes (8): apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Iterator, Option, DoubleEndedIterator

### Community 18 - "E"
Cohesion: 0.29
Nodes (12): AppliableMigration, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>> (+4 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (11): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+3 more)

### Community 20 - "parse_position_from_row"
Cohesion: 0.20
Nodes (15): geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate, Option (+7 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.12
Nodes (11): Clone, Deref, Option, Self, T, Target, UnitOfWork<'a>, Versioned<T> (+3 more)

### Community 22 - "RepositoryResult"
Cohesion: 0.20
Nodes (8): FakeHaustiereRepository, FakeSeminarTermineRepository, Haustier, NaiveDate, Rechnung, RepositoryResult, SeminarTermin, Vec

### Community 23 - "stores/index.tsx"
Cohesion: 0.04
Nodes (24): CompatibilityResult, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented, TODO: Remove once sync is implemented (+16 more)

### Community 24 - "tests.rs"
Cohesion: 0.21
Nodes (33): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+25 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.17
Nodes (14): LeistungOffen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler (+6 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.08
Nodes (19): Address, Animal, AnimalResponse, Client, ClientResponse, DatabaseObject, DatabaseResponse, isRecord() (+11 more)

### Community 27 - "query"
Cohesion: 0.19
Nodes (12): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAddress(), deleteAnimal(), patchAnimal(), query(), deleteRace() (+4 more)

### Community 28 - "relations/index.tsx"
Cohesion: 0.11
Nodes (23): clientSearched(), EventParticipants, participationSearchedClient(), SmallSearchField, ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent() (+15 more)

### Community 29 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 30 - "paths.ts"
Cohesion: 0.11
Nodes (12): AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, MenuEntryData, LiveRefresher, ClientOverview (+4 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.23
Nodes (8): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Example, From, Self, TryFrom

### Community 32 - "AddressTable.tsx"
Cohesion: 0.12
Nodes (17): useAddresses(), AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton (+9 more)

### Community 33 - "RechnungId"
Cohesion: 0.12
Nodes (29): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+21 more)

### Community 34 - "Klient"
Cohesion: 0.09
Nodes (21): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, neu(), NeuerKlient, Adresse (+13 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.10
Nodes (22): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, LeistungQuelle (+14 more)

### Community 36 - "PdfDokument"
Cohesion: 0.14
Nodes (12): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, PdfDokument, FakePdfRenderer, Arc (+4 more)

### Community 37 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 38 - "ResultReport"
Cohesion: 0.10
Nodes (22): ObjectStoreError, ObjectStream, Rechnung, RepositoryError, ResultReport, Self, Seminar, SeminarTermin (+14 more)

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - ".get_current_version"
Cohesion: 0.21
Nodes (9): Box, Future, Option, Output, Pin, Send, Transaction, SQLiteConnection (+1 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.16
Nodes (20): buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Option, Report, Self (+12 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.24
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (15): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungQuelleSeminar, LeistungStatus, Decimal (+7 more)

### Community 44 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 45 - "FakeDatastore"
Cohesion: 0.12
Nodes (16): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository, Arc, Behandlung, Clone (+8 more)

### Community 46 - "NotificationType"
Cohesion: 0.14
Nodes (15): ErrorReportExt, Result<T, E>, C, Report, Send, Sync, T, ThreadSafeError (+7 more)

### Community 47 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedBehandlungRepository, InstrumentedHaustierRepository, InstrumentedKlientRepository, InstrumentedLeistungRepository (+4 more)

### Community 48 - "SeminarTerminId"
Cohesion: 0.16
Nodes (18): abgehalten_use_case(), ExecutionContext, ExecutionSource, Arc, SeminarTerminId, UseCase, Option, Seminar (+10 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.16
Nodes (17): format_naive_date(), parse_naive_date(), parse_rechnung_id(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung (+9 more)

### Community 51 - "Klientbericht"
Cohesion: 0.17
Nodes (15): Klientbericht, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse, NaiveDate (+7 more)

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (41): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+33 more)

### Community 53 - "Seminar"
Cohesion: 0.14
Nodes (12): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+4 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.36
Nodes (6): HaustierErstellen, HaustierErstellung, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.13
Nodes (9): NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport, Self (+1 more)

### Community 56 - "Ratio"
Cohesion: 0.22
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 57 - "api/client.ts"
Cohesion: 0.21
Nodes (6): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useBackend()

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "repos.rs"
Cohesion: 0.08
Nodes (10): BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository, ProduktRepository, RepositoryError, Option, Send (+2 more)

### Community 60 - "DatabaseConnection"
Cohesion: 0.14
Nodes (7): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection, DatabaseError

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.31
Nodes (9): C, From, ObjectStream, Report, StatusCode, T, StreamBinaryResponse, TypicalJsonResponse<T> (+1 more)

### Community 62 - "base_app_builder"
Cohesion: 0.05
Nodes (74): App, Box, Context, F, ResultReport, base_app_builder(), AppBuilder, SetUowProvider (+66 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.16
Nodes (24): BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, From, NaiveDate, Option (+16 more)

### Community 64 - "Preis"
Cohesion: 0.12
Nodes (8): Add, Preis, Output, Self, position_from_leistung(), RechnungIn<S>, Rechnungsposition, Mul

### Community 65 - "HaustierErstellen"
Cohesion: 0.23
Nodes (10): HaustierErstellen, HaustierErstellenFehler, Context, Haustier, NaiveDate, Report, ResultReport, Vec (+2 more)

### Community 66 - "Integration Test Workflow"
Cohesion: 0.53
Nodes (6): mise, mise-action, Release Workflow, Integration Test Workflow, tauri-action, Unit Test Workflow

### Community 67 - "frontend-legacy/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "src/api/schema.d.ts"
Cohesion: 0.13
Nodes (14): components, $defs, leistungQuelle_LeistungQuelleBehandlungTypValues, leistungQuelle_LeistungQuelleManuellTypValues, leistungQuelle_LeistungQuelleProduktTypValues, leistungQuelle_LeistungQuelleSeminarTypValues, leistungStatusValues, operations (+6 more)

### Community 69 - "yams-core"
Cohesion: 0.44
Nodes (9): molting, yams, yams-api, yams-core, yams-fakes, yams-filesystemstore, yams-persistence, yams-server (+1 more)

### Community 70 - "leistung-form.tsx"
Cohesion: 0.12
Nodes (35): Alert(), AlertProps, AlertVariant, variantClasses, Button(), ButtonProps, ButtonSize, ButtonVariant (+27 more)

### Community 71 - "Seminar — Domain-Spezifikation"
Cohesion: 0.10
Nodes (18): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+10 more)

### Community 72 - "Adresse"
Cohesion: 0.29
Nodes (7): Adresse, domain::Adresse, Ländercode, Example, From, Self, TryFrom

### Community 73 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 74 - "BackendClient"
Cohesion: 0.50
Nodes (4): BackendClient, MobX, TanStack Query, yamsconfig.json

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 77 - "Menge"
Cohesion: 0.10
Nodes (28): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+20 more)

### Community 80 - "api/types.ts"
Cohesion: 0.10
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.15
Nodes (28): PdfRenderError, adresse_dict(), compile_paged(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict() (+20 more)

### Community 89 - "common.rs"
Cohesion: 0.20
Nodes (20): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis(), parse_ratio(), parse_uuid() (+12 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.14
Nodes (21): buchung_umsatz(), BuchungUmsatz, NaiveDate, Report, Self, Vec, SeminarBuchungAnlegenFehler, SeminarBuchungStornierenFehler (+13 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, InstanceType, Arc, AsRef, Connection, Deref, Mutex, Path (+9 more)

### Community 101 - "Clock"
Cohesion: 0.20
Nodes (7): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync

### Community 102 - "UnitOfWork"
Cohesion: 0.16
Nodes (13): Arc, LockedUnitOfWorkImpl, Box, RepositoryResult, Send, Sync, UnitOfWork, UnitOfWorkImpl (+5 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.17
Nodes (20): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+12 more)

### Community 104 - "instrumented.rs"
Cohesion: 0.22
Nodes (9): InstrumentedObjectStore, InstrumentedPdfRenderer, pdf_dokument_kind(), Arc, ObjectStoreError, ObjectStream, Option, ResultReport (+1 more)

### Community 105 - "useStore"
Cohesion: 0.07
Nodes (44): queryClient, EventsUsages, EventDetailItem, EventOverviewItem, askSubmitDeleteEvent(), submitDeleteEvent(), LanguagePicker, MainMenu (+36 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.31
Nodes (7): BehandlungErstellenFehler, Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "StatusCode"
Cohesion: 0.06
Nodes (34): BehandlungErstellenFehler, HaustierErstellenFehler, HttpStatusMapping, KlientErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler, LeistungManuellErfassenFehler, mapped() (+26 more)

### Community 108 - "Klient"
Cohesion: 0.20
Nodes (12): Klient, KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate (+4 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.28
Nodes (7): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain(), ProduktErstellenFehler

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 112 - "InstrumentedUnitOfWork"
Cohesion: 0.36
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 113 - "KlientId"
Cohesion: 0.16
Nodes (14): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+6 more)

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 116 - "KlientErstellen"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Klient, Mobilnummer, NaiveDate

### Community 117 - "ports/object_store.rs"
Cohesion: 0.53
Nodes (5): collect_object(), ObjectStoreError, once_stream(), ObjectStream, Vec

### Community 118 - ".finish"
Cohesion: 0.28
Nodes (6): C, F, Formatter, Future, Output, Report

### Community 119 - "KlientErstellung"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Self, TryFrom

### Community 120 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 121 - "teilnahme_pdf_laden"
Cohesion: 0.36
Nodes (8): rechnung_object_key(), rechnung_pdf_laden(), ObjectStoreError, ObjectStream, Option, ResultReport, teilnahme_object_key(), teilnahme_pdf_laden()

### Community 122 - "v0004_leistungen_quelle_mwst.rs"
Cohesion: 0.19
Nodes (17): adds_quelle_mwst_when_neither_column_exists(), align_column(), already_aligned_schema_is_noop(), apply(), column_exists(), columns(), memory(), merge_drops_quelle_mwst_prozentsatz_so_insert_matches_repo() (+9 more)

### Community 123 - "src/errors.rs"
Cohesion: 1.00
Nodes (3): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), RepositoryError

### Community 124 - "SQLiteHaustierRepository"
Cohesion: 0.26
Nodes (10): query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult, Transaction (+2 more)

### Community 125 - "SeminarId"
Cohesion: 0.26
Nodes (10): SeminarId, Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction (+2 more)

### Community 126 - "ObjectStore"
Cohesion: 0.19
Nodes (9): ExecutionContext<'a>, RepositoryResult, Self, ObjectStore, Send, Sync, PdfRenderer, Send (+1 more)

### Community 127 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (4): Migration, Option, Transaction, table_exists()

### Community 129 - "Versioned"
Cohesion: 0.22
Nodes (11): Versioned, Leistung, klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult (+3 more)

### Community 130 - "YamsApiSpec"
Cohesion: 0.32
Nodes (5): Path, SeminarTermin, SeminarUmsatzVorschau, Uuid, YamsApiSpec

### Community 131 - "openapi_service"
Cohesion: 0.14
Nodes (14): BackendServerError, catch_panic(), Config, main(), Report, init_tracing(), CatchPanic, openapi_service() (+6 more)

### Community 133 - "LeistungId"
Cohesion: 0.05
Nodes (60): LeistungId, Abgehalten, Abgesagt, absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys() (+52 more)

### Community 134 - ".haustier_by_id"
Cohesion: 0.29
Nodes (7): Haustier, HaustierErstellung, Haustier, NaiveDate, Uuid, schema_haustier_from_domain(), HaustierErstellenFehler

### Community 135 - "service/pdf.rs"
Cohesion: 0.20
Nodes (17): klient_bericht(), mit_objekt_rollback(), nach_pdf_persistieren(), objekt_löschen_best_effort(), pdfs_rendern_und_ablegen(), rechnungsdokument(), Klient, Report (+9 more)

### Community 136 - "bad_request"
Cohesion: 0.14
Nodes (12): bad_request(), Arc, C, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Report (+4 more)

### Community 137 - "StreamBody"
Cohesion: 0.22
Nodes (6): Self, StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 140 - ".tagesabschluss_durchführen"
Cohesion: 0.33
Nodes (4): Haustier, Rechnung, TagesabschlussErstellung, Vec

### Community 141 - "SQLiteBehandlungRepository"
Cohesion: 0.47
Nodes (5): Arc, Mutex, Option, Transaction, SQLiteBehandlungRepository

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "FakeUnitOfWork"
Cohesion: 0.30
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 145 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, SeminarUmsatzPrognoseBisDatumFehler

### Community 146 - "preis.rs"
Cohesion: 0.29
Nodes (8): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 147 - "document_text"
Cohesion: 0.50
Nodes (4): collect_frame_text(), document_text(), Frame, PagedDocument

### Community 152 - "parse_datetime"
Cohesion: 0.67
Nodes (4): format_datetime(), parse_datetime(), DateTime, Utc

## Knowledge Gaps
- **246 isolated node(s):** `molting`, `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler` (+241 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 751 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **21 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `openapi_service`, `LeistungId`, `.haustier_by_id`, `service/pdf.rs`, `YAMSFrontendConfig`, `termin_from_parts`, `InternalServerError`, `TypicalJsonResponse`, `Behandlung`, `FakeUnitOfWork`, `domain/kontakt.rs`, `parse_position_from_row`, `document_text`, `parse_datetime`, `BehandlungErstellen`, `LeistungOffen`, `tests.rs`, `EmailAdresse`, `Klient`, `NeueLeistung`, `in_memory_object_store.rs`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `SeminarTerminId`, `leistung_from_row`, `Klientbericht`, `base_app_builder`, `Seminar`, `HaustierErstellung`, `Produkt`, `StructuredError`, `base_app_builder`, `schema/seminar.rs`, `Preis`, `HaustierErstellen`, `Adresse`, `Menge`, `common.rs`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `KlientId`, `KlientErstellen`, `KlientErstellung`, `Rechnung`, `teilnahme_pdf_laden`, `v0004_leistungen_quelle_mwst.rs`?**
  _High betweenness centrality (0.509) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`, `NotificationType`?**
  _High betweenness centrality (0.310) - this node is a cross-community bridge._
- **Why does `Error` connect `NotificationType` to `arc_up`?**
  _High betweenness centrality (0.301) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `ValidationError`, `RechnungFehler` to the rest of the system?**
  _246 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05853174603174603 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._