# Graph Report - yams  (2026-09-03)

## Corpus Check
- 270 files · ~333,354 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2928 nodes · 7102 edges · 161 communities (130 shown, 22 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 221 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `aca638ff`
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
- relations/index.tsx
- TypicalJsonResponse
- Behandlung
- FakeDatastore
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- bad_request
- libs/database/index.ts
- tests.rs
- LeistungOffen
- makeRecordForTable
- AddressTable.tsx
- HaustierErstellen
- UnitOfWorkImpl
- Rechnungsposition
- EmailAdresse
- api/client.ts
- RechnungId
- Klient
- HaustierId
- PdfDokument
- repos.rs
- ResultReport
- FixedClock
- .get_current_version
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- dialog.ts
- RepositoryResult
- NotificationType
- KlientErstellen
- base_app_builder
- leistung_from_row
- Klientbericht
- base_app_builder
- Seminar
- HaustierErstellung
- Produkt
- Ratio
- LeistungIn<S>
- StructuredError
- .as_impl
- DatabaseConnection
- StreamBinaryResponse
- cases/seminar.rs
- schema/seminar.rs
- Preis
- domain/leistung.rs
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
- .from_parts
- stores/index.tsx
- .behandlung_erstellen
- StatusCode
- Klient
- .produkt_erstellen
- page.tsx
- SQLiteHaustierRepository
- NeuesHaustier
- api/index.ts
- HttpYamsApi
- query
- InMemoryObjectStore
- notification.ts
- KlientErstellung
- Rechnung
- cases/abrechnung.rs
- FakeObjectStore
- src/errors.rs
- FakeUnitOfWork
- seminar_from_row
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
- FakeKlientenRepository
- StreamBody
- !.next
- InternalServerError
- App
- SeminarTerminId
- ports/object_store.rs
- .seminar_umsatz_prognose
- KlientId
- KlientRepository
- RechnungRepository
- document_text
- ProduktId
- FakeSeminareRepository
- preis.rs
- parse_datetime
- behandlung_from_row
- Rechnung
- layout.tsx
- BehandlungErstellen
- teilnahme_dokument
- .tagesabschluss_durchführen
- HaustierRepository
- ProduktRepository
- wal_connection_race.rs

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
10. `LeistungId` - 34 edges

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

## Communities (161 total, 22 thin omitted)

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
Cohesion: 0.14
Nodes (27): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+19 more)

### Community 8 - "String"
Cohesion: 0.15
Nodes (36): YamsAppApi, From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen() (+28 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.19
Nodes (23): insert_params(), leistung_id_for(), load_buchungen(), load_termin(), ort_from_columns(), replace_buchungen(), Arc, FxHashMap (+15 more)

### Community 10 - "yams-filesystemstore/src/lib.rs"
Cohesion: 0.14
Nodes (23): FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing(), put_get_roundtrip(), rejects_parent_segment(), Arc (+15 more)

### Community 11 - "UpMigration"
Cohesion: 0.11
Nodes (13): Send, Sync, UpMigration, Migration, Option, Transaction, Migration, Option (+5 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "relations/index.tsx"
Cohesion: 0.19
Nodes (13): clientSearched(), EventParticipants, participationSearchedClient(), SmallSearchField, deleteClientRelation(), relateClients(), updateClientRelation(), ClientDetail (+5 more)

### Community 14 - "TypicalJsonResponse"
Cohesion: 0.12
Nodes (14): Behandlung, BehandlungErstellung, HaustierErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung (+6 more)

### Community 15 - "Behandlung"
Cohesion: 0.15
Nodes (9): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+1 more)

### Community 16 - "FakeDatastore"
Cohesion: 0.14
Nodes (14): FakeBehandlungenRepository, FakeDatastore, FakeProdukteRepository, Arc, Behandlung, Clone, Default, FxHashMap (+6 more)

### Community 17 - "apply_up_migrations"
Cohesion: 0.21
Nodes (11): apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationUp, MigrationError, MigrationTarget, Box, Item (+3 more)

### Community 18 - "E"
Cohesion: 0.35
Nodes (9): AppliableMigration, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration, T (+1 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (11): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+3 more)

### Community 20 - "parse_position_from_row"
Cohesion: 0.22
Nodes (15): geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate, Option (+7 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (10): Clone, Deref, Option, Self, T, Target, Versioned<T>, Ordering (+2 more)

### Community 22 - "bad_request"
Cohesion: 0.14
Nodes (12): bad_request(), Arc, C, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Report (+4 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.06
Nodes (19): CompatibilityResult, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented, TODO: Remove once sync is implemented (+11 more)

### Community 24 - "tests.rs"
Cohesion: 0.21
Nodes (33): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+25 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.16
Nodes (15): LeistungOffen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler (+7 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.08
Nodes (20): createSeminar(), ensureSeminar(), Address, AddressResponse, Client, DatabaseObject, DatabaseResponse, isRecord() (+12 more)

### Community 27 - "AddressTable.tsx"
Cohesion: 0.08
Nodes (36): AddressEditTableRowContent, AddressTableHeader, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons, EditState (+28 more)

### Community 28 - "HaustierErstellen"
Cohesion: 0.23
Nodes (10): HaustierErstellen, HaustierErstellenFehler, Context, Haustier, NaiveDate, Report, ResultReport, Vec (+2 more)

### Community 29 - "UnitOfWorkImpl"
Cohesion: 0.16
Nodes (13): main(), Box, RepositoryResult, Unimplemented, ExecutionSource, LockedUnitOfWorkImpl, Send, Sync (+5 more)

### Community 30 - "Rechnungsposition"
Cohesion: 0.16
Nodes (6): Menge, MengeFehler, Decimal, Self, position_from_leistung(), Rechnungsposition

### Community 31 - "EmailAdresse"
Cohesion: 0.23
Nodes (8): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Example, From, Self, TryFrom

### Community 32 - "api/client.ts"
Cohesion: 0.18
Nodes (8): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), Addresses

### Community 33 - "RechnungId"
Cohesion: 0.23
Nodes (16): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), leistung_offen(), mwst_19(), position(), rechnung_offen_rejects_empty_positionen(), RechnungFehler (+8 more)

### Community 34 - "Klient"
Cohesion: 0.13
Nodes (7): Klient, NeuerKlient, Adresse, EmailAdresse, Into, Mobilnummer, NaiveDate

### Community 35 - "HaustierId"
Cohesion: 0.18
Nodes (12): HaustierId, LeistungFehler, LeistungIn, LeistungQuelle, mark_abgerechnet_sets_rechnung_id(), NeueLeistung, Into, NaiveDate (+4 more)

### Community 36 - "PdfDokument"
Cohesion: 0.14
Nodes (12): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, PdfDokument, FakePdfRenderer, Arc (+4 more)

### Community 37 - "repos.rs"
Cohesion: 0.22
Nodes (4): RepositoryError, Option, SeminarTerminRepository, Debug

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
Cohesion: 0.08
Nodes (9): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore, EventStore, Store (+1 more)

### Community 45 - "RepositoryResult"
Cohesion: 0.14
Nodes (11): FakeHaustiereRepository, FakeLeistungenRepository, FakeRechnungenRepository, FakeSeminarTermineRepository, Haustier, Leistung, NaiveDate, Rechnung (+3 more)

### Community 46 - "NotificationType"
Cohesion: 0.14
Nodes (15): ErrorReportExt, Result<T, E>, C, Report, Send, Sync, T, ThreadSafeError (+7 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Klient, Mobilnummer, NaiveDate

### Community 48 - "base_app_builder"
Cohesion: 0.16
Nodes (12): base_app_builder(), AppBuilder, SetUowProvider, test_tagesabschluss_empty_day_returns_no_rechnungen(), test_haustier(), mwst_19(), test_behandlung_erstellen(), test_behandlung_erstellen_rejects_empty_name() (+4 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.17
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 51 - "Klientbericht"
Cohesion: 0.17
Nodes (15): Klientbericht, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse, NaiveDate (+7 more)

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (41): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+33 more)

### Community 53 - "Seminar"
Cohesion: 0.13
Nodes (12): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+4 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.36
Nodes (6): HaustierErstellen, HaustierErstellung, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.12
Nodes (8): NeuesProdukt, preis(), Produkt, ProduktFehler, Into, ResultReport, Self, Uuid

### Community 56 - "Ratio"
Cohesion: 0.24
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - ".as_impl"
Cohesion: 0.14
Nodes (6): BehandlungRepository, LeistungRepository, Send, Sync, SeminarRepository, UnitOfWork<'a>

### Community 60 - "DatabaseConnection"
Cohesion: 0.14
Nodes (7): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection, DatabaseError

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.31
Nodes (9): C, From, ObjectStream, Report, StatusCode, T, StreamBinaryResponse, TypicalJsonResponse<T> (+1 more)

### Community 62 - "cases/seminar.rs"
Cohesion: 0.21
Nodes (28): abgehalten_maps_every_confirmed_buchung(), abgehalten_schreibt_teilnahme_pdf_für_bestätigte_buchungen(), absage_blockiert_abgehalten(), adresse(), aktualisieren_blockiert_nach_abgehalten(), aktualisieren_nur_geplant(), assert_offen_mwst(), kapazität_und_doppelbuchung() (+20 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.13
Nodes (27): NaiveDate, SeminarUmsatzPrognose, BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, From (+19 more)

### Community 64 - "Preis"
Cohesion: 0.20
Nodes (5): Add, Preis, Output, Self, Mul

### Community 65 - "domain/leistung.rs"
Cohesion: 0.15
Nodes (11): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, Offen, produkt_betrag_multiplies_menge(), From (+3 more)

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
Cohesion: 0.19
Nodes (19): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis(), parse_ratio(), parse_rechnung_id() (+11 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.15
Nodes (20): buchung_umsatz(), BuchungUmsatz, NaiveDate, Report, Self, Vec, SeminarBuchungAnlegenFehler, SeminarBuchungStornierenFehler (+12 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "Clock"
Cohesion: 0.22
Nodes (7): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync

### Community 102 - "UnitOfWork"
Cohesion: 0.18
Nodes (10): Box, C, F, Formatter, Future, Output, Report, RepositoryResult (+2 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.17
Nodes (20): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+12 more)

### Community 104 - ".from_parts"
Cohesion: 0.15
Nodes (9): Bezahlt, Offen, RechnungIn, RechnungIn<S>, NaiveDate, Option, S, Vec (+1 more)

### Community 105 - "stores/index.tsx"
Cohesion: 0.06
Nodes (42): queryClient, AddressTableRow, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow (+34 more)

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

### Community 112 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 113 - "NeuesHaustier"
Cohesion: 0.13
Nodes (12): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, neues(), NeuesHaustier, Into (+4 more)

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 116 - "query"
Cohesion: 0.12
Nodes (12): AnimalAddItem, AnimalComboBox, deleteAddress(), patchAnimal(), query(), deleteRace(), patchRace(), Animal (+4 more)

### Community 117 - "InMemoryObjectStore"
Cohesion: 0.23
Nodes (12): get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap, Mutex, ObjectStoreError, ObjectStream (+4 more)

### Community 118 - "notification.ts"
Cohesion: 0.13
Nodes (14): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+6 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Self, TryFrom

### Community 120 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 121 - "cases/abrechnung.rs"
Cohesion: 0.27
Nodes (15): AbrechnungSetup, app_with_pdf_fakes(), menge(), mwst_19(), Arc, Klient, NaiveDate, setup_abrechnung_fixture() (+7 more)

### Community 122 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 123 - "src/errors.rs"
Cohesion: 1.00
Nodes (3): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), RepositoryError

### Community 124 - "FakeUnitOfWork"
Cohesion: 0.30
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 125 - "seminar_from_row"
Cohesion: 0.27
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, seminar_from_row() (+1 more)

### Community 126 - "ObjectStore"
Cohesion: 0.16
Nodes (10): ExecutionContext<'a>, Arc, RepositoryResult, Self, ObjectStore, Send, Sync, PdfRenderer (+2 more)

### Community 127 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (4): Migration, Option, Transaction, table_exists()

### Community 129 - "Versioned"
Cohesion: 0.28
Nodes (10): Versioned, klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row (+2 more)

### Community 130 - "YamsApiSpec"
Cohesion: 0.32
Nodes (5): Path, SeminarTermin, SeminarUmsatzVorschau, Uuid, YamsApiSpec

### Community 131 - "openapi_service"
Cohesion: 0.14
Nodes (14): BackendServerError, catch_panic(), Config, main(), Report, init_tracing(), CatchPanic, openapi_service() (+6 more)

### Community 133 - "LeistungId"
Cohesion: 0.05
Nodes (61): LeistungId, SeminarId, Abgehalten, Abgesagt, absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only() (+53 more)

### Community 134 - ".haustier_by_id"
Cohesion: 0.29
Nodes (7): Haustier, HaustierErstellung, Haustier, NaiveDate, Uuid, schema_haustier_from_domain(), HaustierErstellenFehler

### Community 135 - "service/pdf.rs"
Cohesion: 0.31
Nodes (10): rechnung_key_uses_uuid(), rechnung_object_key(), rechnung_pdf_laden(), ObjectStoreError, ObjectStream, Option, ResultReport, teilnahme_key_nests_termin_and_buchung() (+2 more)

### Community 137 - "StreamBody"
Cohesion: 0.22
Nodes (6): Self, StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 138 - "!.next"
Cohesion: 0.33
Nodes (3): Data, nextConfig, !.next

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 140 - "App"
Cohesion: 0.21
Nodes (10): App, Arc, Box, Context, F, ResultReport, app_with_pdf_fakes(), Arc (+2 more)

### Community 141 - "SeminarTerminId"
Cohesion: 0.17
Nodes (16): abgehalten_use_case(), ExecutionContext, SeminarTerminId, UseCase, Option, Seminar, SeminarOrt, SeminarTermin (+8 more)

### Community 142 - "ports/object_store.rs"
Cohesion: 0.53
Nodes (5): collect_object(), ObjectStoreError, once_stream(), ObjectStream, Vec

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "KlientId"
Cohesion: 0.33
Nodes (11): klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, KlientId, neu(), AsRef, ResultReport (+3 more)

### Community 147 - "document_text"
Cohesion: 0.50
Nodes (4): collect_frame_text(), document_text(), Frame, PagedDocument

### Community 148 - "ProduktId"
Cohesion: 0.27
Nodes (10): ProduktId, produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row (+2 more)

### Community 150 - "preis.rs"
Cohesion: 0.29
Nodes (8): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 151 - "parse_datetime"
Cohesion: 0.67
Nodes (4): format_datetime(), parse_datetime(), DateTime, Utc

### Community 152 - "behandlung_from_row"
Cohesion: 0.27
Nodes (9): behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 153 - "Rechnung"
Cohesion: 0.22
Nodes (6): Rechnung, RechnungBezahlt, RechnungOffen, From, Vec, TagesabschlussDurchführen

### Community 154 - "layout.tsx"
Cohesion: 0.31
Nodes (5): geistMono, geistSans, metadata, Providers(), createQueryClient()

### Community 156 - "teilnahme_dokument"
Cohesion: 0.38
Nodes (7): klient_bericht(), rechnungsdokument(), Klient, S, Seminar, SeminarBuchung, teilnahme_dokument()

### Community 157 - ".tagesabschluss_durchführen"
Cohesion: 0.33
Nodes (4): Haustier, Rechnung, TagesabschlussErstellung, Vec

### Community 160 - "wal_connection_race.rs"
Cohesion: 0.70
Nodes (4): app(), neuer_klient(), parallel_execute_fn_survives_wal_connection_init(), Arc

## Knowledge Gaps
- **246 isolated node(s):** `molting`, `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler` (+241 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 738 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `openapi_service`, `LeistungId`, `.haustier_by_id`, `service/pdf.rs`, `YAMSFrontendConfig`, `termin_from_parts`, `InternalServerError`, `SeminarTerminId`, `TypicalJsonResponse`, `Behandlung`, `KlientId`, `domain/kontakt.rs`, `parse_position_from_row`, `document_text`, `parse_datetime`, `tests.rs`, `LeistungOffen`, `BehandlungErstellen`, `HaustierErstellen`, `Rechnungsposition`, `EmailAdresse`, `Klient`, `HaustierId`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `KlientErstellen`, `leistung_from_row`, `Klientbericht`, `base_app_builder`, `Seminar`, `HaustierErstellung`, `Produkt`, `StructuredError`, `schema/seminar.rs`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `NeuesHaustier`, `InMemoryObjectStore`, `KlientErstellung`, `Rechnung`, `FakeObjectStore`, `FakeUnitOfWork`?**
  _High betweenness centrality (0.502) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`, `NotificationType`?**
  _High betweenness centrality (0.308) - this node is a cross-community bridge._
- **Why does `Error` connect `NotificationType` to `arc_up`?**
  _High betweenness centrality (0.303) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `ValidationError`, `RechnungFehler` to the rest of the system?**
  _246 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05853174603174603 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._