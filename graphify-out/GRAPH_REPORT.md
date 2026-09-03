# Graph Report - yams  (2026-09-03)

## Corpus Check
- 271 files · ~335,206 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2992 nodes · 7295 edges · 155 communities (122 shown, 24 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 227 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `33b0a74d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .add_dyn
- arc_up
- devDependencies
- biome.json
- yams-api
- Klient
- YAMSFrontendConfig
- EventForm.tsx
- String
- termin_from_parts
- yams-filesystemstore/src/lib.rs
- UpMigration
- compilerOptions
- EventStore
- .leistung_aus_behandlung_buchen
- Behandlung
- ProduktId
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- RepositoryResult
- libs/database/index.ts
- tests.rs
- LeistungOffen
- makeRecordForTable
- query
- useStore
- .begin
- Client
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
- domain/seminar_termin.rs
- SeminarTerminId
- SQLiteLeistungRepository
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
- Json
- base_app_builder
- schema/seminar.rs
- Preis
- teilnahme_dokument
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
- requests/abrechnung.rs
- Next.js
- postcss.config.mjs
- api/types.ts
- Built on SurrealDB
- YAMS Banner SVG
- YAMS Logo SVG
- yams-typstreports/src/lib.rs
- common.rs
- ExecutionContext
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- SQLiteInstance
- SystemClock
- UnitOfWork
- hooks/index.ts
- instrumented.rs
- stores/index.tsx
- .behandlung_erstellen
- StatusCode
- Klient
- .produkt_erstellen
- page.tsx
- InstrumentedUnitOfWork
- KlientId
- api/index.ts
- HttpYamsApi
- openapi_service
- ports/object_store.rs
- .finish
- KlientErstellung
- Rechnung
- quelle_to_db
- v0004_leistungen_quelle_mwst.rs
- src/errors.rs
- SQLiteHaustierRepository
- seminar_from_row
- PdfRenderer
- SQLiteUnitOfWork
- Migration
- Versioned
- YamsApiSpec
- main
- validation_error.rs
- LeistungId
- .haustier_erstellen
- ObjectStore
- bad_request
- StreamBody
- RechnungIn<S>
- InternalServerError
- SeminarId
- Zeitraum
- FakeKlientenRepository
- .seminar_umsatz_prognose
- FakeUnitOfWork
- LeistungRepository
- preis.rs
- document_text
- RechnungRepository
- SeminarRepository
- SeminarTerminRepository
- FakeSeminareRepository
- .behandlung_erstellen
- .klient_erstellen
- .produkt_erstellen

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

## Communities (155 total, 24 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (40): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+32 more)

### Community 2 - "devDependencies"
Cohesion: 0.04
Nodes (47): babel-plugin-react-compiler, @biomejs/biome, dependencies, next, openapi-fetch, react, react-dom, @tanstack/react-query (+39 more)

### Community 3 - "biome.json"
Cohesion: 0.07
Nodes (29): source, assist, actions, next, react, files, ignoreUnknown, includes (+21 more)

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
Cohesion: 0.11
Nodes (35): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, clientSearched(), AddClientForm (+27 more)

### Community 8 - "String"
Cohesion: 0.15
Nodes (36): YamsAppApi, From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen() (+28 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.16
Nodes (27): format_datetime(), parse_datetime(), DateTime, Utc, insert_params(), leistung_id_for(), load_buchungen(), load_termin() (+19 more)

### Community 10 - "yams-filesystemstore/src/lib.rs"
Cohesion: 0.14
Nodes (26): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+18 more)

### Community 11 - "UpMigration"
Cohesion: 0.15
Nodes (9): Send, Sync, UpMigration, Migration, Option, Transaction, Migration, Option (+1 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "EventStore"
Cohesion: 0.06
Nodes (6): EventResponse, AddressStore, AnimalStore, EventStore, Store, SettingsStore

### Community 14 - ".leistung_aus_behandlung_buchen"
Cohesion: 0.29
Nodes (4): Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung

### Community 15 - "Behandlung"
Cohesion: 0.10
Nodes (18): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+10 more)

### Community 16 - "ProduktId"
Cohesion: 0.27
Nodes (10): ProduktId, produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row (+2 more)

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
Cohesion: 0.19
Nodes (16): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (10): Clone, Deref, Option, Self, T, Target, Versioned<T>, Ordering (+2 more)

### Community 22 - "RepositoryResult"
Cohesion: 0.14
Nodes (11): FakeHaustiereRepository, FakeLeistungenRepository, FakeRechnungenRepository, FakeSeminarTermineRepository, Haustier, Leistung, NaiveDate, Rechnung (+3 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.06
Nodes (22): deleteEventParticipation(), CompatibilityResult, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented (+14 more)

### Community 24 - "tests.rs"
Cohesion: 0.21
Nodes (33): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+25 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.18
Nodes (15): LeistungOffen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler (+7 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.10
Nodes (18): AnimalComboBox, createSeminar(), ensureSeminar(), Address, AddressResponse, Animal, AnimalResponse, DatabaseObject (+10 more)

### Community 27 - "query"
Cohesion: 0.17
Nodes (18): AnimalAddItem, AnimalRow, deleteAddress(), deleteAnimal(), patchAnimal(), deleteClientRelation(), relateClients(), updateClientRelation() (+10 more)

### Community 28 - "useStore"
Cohesion: 0.12
Nodes (25): EventsUsages, EventDetailItem, EventOverviewItem, EventParticipants, participationSearchedClient(), askSubmitDeleteEvent(), submitDeleteEvent(), SmallSearchField (+17 more)

### Community 29 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 30 - "Client"
Cohesion: 0.09
Nodes (17): AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, MenuEntryData, LiveRefresher, Client (+9 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.23
Nodes (8): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Example, From, Self, TryFrom

### Community 32 - "AddressTable.tsx"
Cohesion: 0.12
Nodes (17): useAddresses(), AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton (+9 more)

### Community 33 - "RechnungId"
Cohesion: 0.11
Nodes (30): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+22 more)

### Community 34 - "Klient"
Cohesion: 0.09
Nodes (21): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, neu(), NeuerKlient, Adresse (+13 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.09
Nodes (21): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, LeistungIn<S> (+13 more)

### Community 36 - "PdfDokument"
Cohesion: 0.14
Nodes (12): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, PdfDokument, FakePdfRenderer, Arc (+4 more)

### Community 37 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 38 - "ResultReport"
Cohesion: 0.09
Nodes (23): Haustier, ObjectStoreError, ObjectStream, Rechnung, RepositoryError, ResultReport, Self, Seminar (+15 more)

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - ".get_current_version"
Cohesion: 0.21
Nodes (9): Box, Future, Option, Output, Pin, Send, Transaction, SQLiteConnection (+1 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.16
Nodes (21): abgehalten_use_case(), buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Option, Report (+13 more)

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
Cohesion: 0.14
Nodes (14): FakeBehandlungenRepository, FakeDatastore, FakeProdukteRepository, Arc, Behandlung, Clone, Default, FxHashMap (+6 more)

### Community 46 - "NotificationType"
Cohesion: 0.14
Nodes (15): ErrorReportExt, Result<T, E>, C, Report, Send, Sync, T, ThreadSafeError (+7 more)

### Community 47 - "domain/seminar_termin.rs"
Cohesion: 0.30
Nodes (18): absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient() (+10 more)

### Community 48 - "SeminarTerminId"
Cohesion: 0.10
Nodes (13): seminar_betrag_full_rabatt_is_zero(), seminar_betrag_uses_nach_rabatt(), Item, Iterator, Self, Seminar, Uuid, SeminarBuchung (+5 more)

### Community 50 - "SQLiteLeistungRepository"
Cohesion: 0.21
Nodes (11): leistung_offen_from_row(), Arc, Leistung, Mutex, NaiveDate, Option, RepositoryResult, Row (+3 more)

### Community 51 - "Klientbericht"
Cohesion: 0.17
Nodes (15): Klientbericht, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse, NaiveDate (+7 more)

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (41): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+33 more)

### Community 53 - "Seminar"
Cohesion: 0.15
Nodes (12): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+4 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.36
Nodes (6): HaustierErstellen, HaustierErstellung, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.12
Nodes (8): NeuesProdukt, preis(), Produkt, ProduktFehler, Into, ResultReport, Self, Uuid

### Community 56 - "Ratio"
Cohesion: 0.18
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 57 - "api/client.ts"
Cohesion: 0.21
Nodes (6): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useBackend()

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "repos.rs"
Cohesion: 0.10
Nodes (10): BehandlungRepository, HaustierRepository, KlientRepository, ProduktRepository, RepositoryError, Option, Send, Sync (+2 more)

### Community 60 - "DatabaseConnection"
Cohesion: 0.14
Nodes (7): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection, DatabaseError

### Community 61 - "Json"
Cohesion: 0.27
Nodes (10): C, From, ObjectStream, Report, StatusCode, T, StreamBinaryResponse, TypicalJsonResponse<T> (+2 more)

### Community 62 - "base_app_builder"
Cohesion: 0.05
Nodes (75): App, Arc, Box, Context, F, ResultReport, base_app_builder(), AppBuilder (+67 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.13
Nodes (27): NaiveDate, SeminarUmsatzPrognose, BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, From (+19 more)

### Community 64 - "Preis"
Cohesion: 0.11
Nodes (9): Add, Menge, MengeFehler, Decimal, Self, Preis, Output, Self (+1 more)

### Community 65 - "teilnahme_dokument"
Cohesion: 0.43
Nodes (7): klient_bericht(), rechnungsdokument(), Klient, S, Seminar, SeminarBuchung, teilnahme_dokument()

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
Cohesion: 0.11
Nodes (36): Alert(), AlertProps, AlertVariant, variantClasses, Button(), ButtonProps, ButtonSize, ButtonVariant (+28 more)

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

### Community 77 - "requests/abrechnung.rs"
Cohesion: 0.16
Nodes (24): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+16 more)

### Community 80 - "api/types.ts"
Cohesion: 0.10
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.15
Nodes (28): PdfRenderError, adresse_dict(), compile_paged(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict() (+20 more)

### Community 89 - "common.rs"
Cohesion: 0.27
Nodes (17): format_naive_date(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_naive_date(), parse_preis(), parse_ratio() (+9 more)

### Community 92 - "ExecutionContext"
Cohesion: 0.05
Nodes (57): ExecutionContext, ExecutionSource, Arc, UseCase, BehandlungErstellen, ProduktErstellen, Behandlung, Produkt (+49 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, InstanceType, Arc, AsRef, Connection, Deref, Mutex, Path (+9 more)

### Community 101 - "SystemClock"
Cohesion: 0.33
Nodes (4): DateTime, NaiveDate, Utc, SystemClock

### Community 102 - "UnitOfWork"
Cohesion: 0.24
Nodes (9): LockedUnitOfWorkImpl, Box, RepositoryResult, Send, Sync, UnitOfWork, UnitOfWorkImpl, UnitOfWorkProvider (+1 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.19
Nodes (19): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+11 more)

### Community 104 - "instrumented.rs"
Cohesion: 0.22
Nodes (9): InstrumentedObjectStore, InstrumentedPdfRenderer, pdf_dokument_kind(), Arc, ObjectStoreError, ObjectStream, Option, ResultReport (+1 more)

### Community 105 - "stores/index.tsx"
Cohesion: 0.06
Nodes (33): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+25 more)

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
Cohesion: 0.23
Nodes (4): InstrumentedUnitOfWork, Box, RepositoryResult, Self

### Community 113 - "KlientId"
Cohesion: 0.16
Nodes (14): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+6 more)

### Community 114 - "api/index.ts"
Cohesion: 0.14
Nodes (19): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+11 more)

### Community 116 - "openapi_service"
Cohesion: 0.25
Nodes (7): openapi_service(), Into, Item, Self, IntoIterator, OpenApiService, ServerObject

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

### Community 121 - "quelle_to_db"
Cohesion: 0.29
Nodes (6): menge_to_str(), quelle_to_db(), QuelleDbColumns, ratio_to_str(), Option, LeistungQuelle

### Community 122 - "v0004_leistungen_quelle_mwst.rs"
Cohesion: 0.19
Nodes (17): adds_quelle_mwst_when_neither_column_exists(), align_column(), already_aligned_schema_is_noop(), apply(), column_exists(), columns(), memory(), merge_drops_quelle_mwst_prozentsatz_so_insert_matches_repo() (+9 more)

### Community 123 - "src/errors.rs"
Cohesion: 1.00
Nodes (3): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), RepositoryError

### Community 124 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 125 - "seminar_from_row"
Cohesion: 0.27
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, seminar_from_row() (+1 more)

### Community 126 - "PdfRenderer"
Cohesion: 0.16
Nodes (9): ExecutionContext<'a>, RepositoryResult, Self, Clock, Send, Sync, PdfRenderer, Send (+1 more)

### Community 127 - "SQLiteUnitOfWork"
Cohesion: 0.19
Nodes (11): Arc, Box, Mutex, Option, RepositoryError, RepositoryResult, ResultReport, Self (+3 more)

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (4): Migration, Option, Transaction, table_exists()

### Community 129 - "Versioned"
Cohesion: 0.28
Nodes (10): Versioned, klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row (+2 more)

### Community 130 - "YamsApiSpec"
Cohesion: 0.19
Nodes (12): Haustier, HaustierErstellung, Path, Rechnung, Seminar, SeminarTermin, SeminarUmsatzVorschau, TagesabschlussErstellung (+4 more)

### Community 131 - "main"
Cohesion: 0.24
Nodes (8): BackendServerError, catch_panic(), Config, main(), Report, init_tracing(), CatchPanic, PanicHandler

### Community 133 - "LeistungId"
Cohesion: 0.16
Nodes (14): LeistungId, Abgehalten, Abgesagt, DateTime, FxHashMap, Into, ResultReport, Utc (+6 more)

### Community 134 - ".haustier_erstellen"
Cohesion: 0.36
Nodes (6): HaustierErstellung, Haustier, NaiveDate, Uuid, schema_haustier_from_domain(), HaustierErstellenFehler

### Community 135 - "ObjectStore"
Cohesion: 0.20
Nodes (19): ObjectStore, Send, Sync, mit_objekt_rollback(), nach_pdf_persistieren(), objekt_löschen_best_effort(), pdfs_rendern_und_ablegen(), rechnung_key_uses_uuid() (+11 more)

### Community 136 - "bad_request"
Cohesion: 0.14
Nodes (12): bad_request(), Arc, C, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Report (+4 more)

### Community 137 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 140 - "SeminarId"
Cohesion: 0.16
Nodes (10): SeminarId, NeuerSeminarTermin, Adresse, From, Option, S, Vec, SeminarOrt (+2 more)

### Community 141 - "Zeitraum"
Cohesion: 0.22
Nodes (10): DateTime, Display, Formatter, Self, Utc, utc(), Zeitraum, zeitraum_accepts_ende_after_beginn() (+2 more)

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "FakeUnitOfWork"
Cohesion: 0.30
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 146 - "preis.rs"
Cohesion: 0.29
Nodes (8): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 147 - "document_text"
Cohesion: 0.50
Nodes (4): collect_frame_text(), document_text(), Frame, PagedDocument

## Knowledge Gaps
- **246 isolated node(s):** `molting`, `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler` (+241 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 741 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `main`, `LeistungId`, `.haustier_erstellen`, `ObjectStore`, `YAMSFrontendConfig`, `termin_from_parts`, `InternalServerError`, `SeminarId`, `Behandlung`, `FakeUnitOfWork`, `domain/kontakt.rs`, `parse_position_from_row`, `document_text`, `tests.rs`, `LeistungOffen`, `EmailAdresse`, `RechnungId`, `Klient`, `NeueLeistung`, `in_memory_object_store.rs`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `Klientbericht`, `base_app_builder`, `Seminar`, `HaustierErstellung`, `Produkt`, `StructuredError`, `Json`, `base_app_builder`, `schema/seminar.rs`, `Preis`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `ExecutionContext`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `KlientId`, `KlientErstellung`, `Rechnung`, `quelle_to_db`, `v0004_leistungen_quelle_mwst.rs`?**
  _High betweenness centrality (0.525) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`, `NotificationType`?**
  _High betweenness centrality (0.311) - this node is a cross-community bridge._
- **Why does `Error` connect `NotificationType` to `arc_up`?**
  _High betweenness centrality (0.310) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `ValidationError`, `RechnungFehler` to the rest of the system?**
  _246 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05853174603174603 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._