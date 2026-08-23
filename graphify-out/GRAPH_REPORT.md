# Graph Report - yams  (2026-08-23)

## Corpus Check
- 205 files · ~304,670 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1759 nodes · 3940 edges · 116 communities (92 shown, 24 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 116 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `def91582`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- E
- arc_up
- devDependencies
- biome.json
- yams-api
- Klient
- YAMSFrontendConfig
- EventForm.tsx
- openapi_service
- schema/leistung.rs
- RepositoryError
- query
- compilerOptions
- useStore
- Versioned
- participation/index.tsx
- RelationStore
- libs/database/index.ts
- EventStore
- String
- SQLiteRechnungRepository
- Versioned<T>
- SQLiteConnection
- stores/index.tsx
- UnitOfWorkImpl
- use_cases/abrechnung.rs
- makeRecordForTable
- BehandlungRepository
- UseCase
- .run
- relations/index.tsx
- EmailAdresse
- addresses/index.tsx
- RechnungId
- KlientId
- ExecutionContext
- SQLiteBehandlungRepository
- UnitOfWork
- YamsAppApi
- Clock
- .get_current_version
- YamsApiSpec
- SQLiteKlientRepository
- LeistungId
- SQLiteUnitOfWork
- HaustierId
- ClientStore
- KlientErstellen
- Migration
- leistung_from_row
- Preis
- Adresse
- LeftMenuLayout.tsx
- Ländercode
- dialog.ts
- AnimalStore
- Rechnungsposition
- StructuredError
- pages/index.tsx
- DatabaseConnection
- base_app_builder
- AddressStore
- KlientErstellung
- Store
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- src/api/schema.d.ts
- yams-core
- main
- Abrechnung — Domain-Spezifikation
- _app.tsx
- base_app_builder
- BackendClient
- diffx-finish-review
- index.d.ts
- requests/abrechnung.rs
- Next.js
- postcss.config.mjs
- Built on SurrealDB
- YAMS Banner SVG
- YAMS Logo SVG
- common.rs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- AddressTable.tsx
- Rechnung
- LeistungRepository
- TagesabschlussErstellung
- RechnungRepository
- ProduktRepository
- RechnungIn<S>
- Klient
- RechnungOffen
- event/index.tsx
- KlientRepository
- ParticipationStore
- layout.tsx
- ProduktErstellen

## God Nodes (most connected - your core abstractions)
1. `useStore()` - 52 edges
2. `Versioned` - 49 edges
3. `query()` - 42 edges
4. `KlientId` - 38 edges
5. `FakeDatastore` - 33 edges
6. `Preis` - 32 edges
7. `E` - 31 edges
8. `makeRecordForTable()` - 29 edges
9. `Record` - 28 edges
10. `Client` - 26 edges

## Surprising Connections (you probably didn't know these)
- `Config` --references--> `String`  [EXTRACTED]
  backend/server/src/main.rs → crates/yams-api/src/errors/internal_error.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  crates/yams-api/src/bin/export_spec.rs → crates/yams-api/src/spec.rs
- `alle_haustiere()` --references--> `String`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/errors/internal_error.rs
- `haustier_erstellen()` --references--> `String`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/errors/internal_error.rs

## Import Cycles
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **CI Release Pipeline** — github_workflows_publish_release_workflow, github_workflows_smoke_tests_workflow, github_workflows_unit_tests_workflow, github_workflows_tauri_action, github_workflows_mise_action [EXTRACTED 1.00]
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **Frontend Backend Communication Bridge** — specs_backend_hexagonal_backend_client, specs_backend_hexagonal_http_adapter, specs_backend_hexagonal_tauri_adapter, specs_backend_hexagonal_tanstack_query, specs_frontend_migration_yamsconfig [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (116 total, 24 thin omitted)

### Community 0 - "E"
Cohesion: 0.05
Nodes (58): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+50 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (41): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+33 more)

### Community 2 - "devDependencies"
Cohesion: 0.05
Nodes (41): babel-plugin-react-compiler, @biomejs/biome, dependencies, next, react, react-dom, devDependencies, babel-plugin-react-compiler (+33 more)

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
Nodes (28): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+20 more)

### Community 8 - "openapi_service"
Cohesion: 0.14
Nodes (14): openapi_service(), C, From, Into, Item, Report, Self, T (+6 more)

### Community 9 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (13): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungStatus, NaiveDate, Option (+5 more)

### Community 10 - "RepositoryError"
Cohesion: 0.26
Nodes (10): RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, AsRef, Path, ResultReport (+2 more)

### Community 11 - "query"
Cohesion: 0.20
Nodes (15): AnimalAddItem, AnimalComboBox, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow (+7 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "useStore"
Cohesion: 0.13
Nodes (21): EventsUsages, EventDetailItem, EventOverviewItem, askSubmitDeleteEvent(), useSubmissionState(), EditClient(), ClientDetail, ClientOverview (+13 more)

### Community 14 - "Versioned"
Cohesion: 0.05
Nodes (54): Versioned, Behandlung, BehandlungId, NeueBehandlung, Decimal, Uuid, NeuesProdukt, Produkt (+46 more)

### Community 15 - "participation/index.tsx"
Cohesion: 0.21
Nodes (13): EventParticipants, participationSearchedClient(), ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation() (+5 more)

### Community 17 - "libs/database/index.ts"
Cohesion: 0.10
Nodes (17): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+9 more)

### Community 18 - "EventStore"
Cohesion: 0.12
Nodes (3): EventResponse, SeminarResponse, EventStore

### Community 19 - "String"
Cohesion: 0.24
Nodes (11): From, String, EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, MobilnummerValidierungsfehler, AsRef, Error (+3 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.22
Nodes (15): geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate, Option (+7 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (9): Clone, Deref, Formatter, T, Target, Versioned<T>, DerefMut, PartialEq (+1 more)

### Community 22 - "SQLiteConnection"
Cohesion: 0.23
Nodes (10): Connection, InstanceType, Arc, Deref, Mutex, Target, SQLiteConnection, Database (+2 more)

### Community 23 - "stores/index.tsx"
Cohesion: 0.32
Nodes (4): live(), StoreContext, ano(), no()

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "use_cases/abrechnung.rs"
Cohesion: 0.20
Nodes (13): BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler, ProduktErstellenFehler (+5 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.17
Nodes (13): createSeminar(), ensureSeminar(), Address, Animal, DatabaseObject, DatabaseResponse, isRecord(), makeRecord() (+5 more)

### Community 27 - "BehandlungRepository"
Cohesion: 0.20
Nodes (4): BehandlungRepository, HaustierRepository, Send, Sync

### Community 28 - "UseCase"
Cohesion: 0.18
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "relations/index.tsx"
Cohesion: 0.22
Nodes (12): clientSearched(), SmallSearchField, deleteClientRelation(), relateClients(), updateClientRelation(), Client, ClientRelation, AddRelationDialog (+4 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "addresses/index.tsx"
Cohesion: 0.14
Nodes (12): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), AddressTableHeader (+4 more)

### Community 33 - "RechnungId"
Cohesion: 0.14
Nodes (16): LeistungAbgerechnet, Bezahlt, Offen, Rechnung, RechnungBezahlt, RechnungFehler, RechnungId, RechnungIn (+8 more)

### Community 34 - "KlientId"
Cohesion: 0.33
Nodes (8): Klient, KlientId, NeuerKlient, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Uuid

### Community 35 - "ExecutionContext"
Cohesion: 0.29
Nodes (5): ExecutionContext, Arc, LeistungOffen, LeistungAusBehandlungBuchen, Error

### Community 36 - "SQLiteBehandlungRepository"
Cohesion: 0.47
Nodes (5): Arc, Mutex, Option, Transaction, SQLiteBehandlungRepository

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "YamsAppApi"
Cohesion: 0.05
Nodes (47): Arc, Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, ResultReport (+39 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "YamsApiSpec"
Cohesion: 0.18
Nodes (12): Behandlung, Haustier, Klient, Leistung, Path, Produkt, Rechnung, Uuid (+4 more)

### Community 42 - "SQLiteKlientRepository"
Cohesion: 0.26
Nodes (9): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 43 - "LeistungId"
Cohesion: 0.13
Nodes (16): Abgerechnet, Leistung, LeistungId, LeistungIn, LeistungIn<S>, LeistungQuelle, NeueLeistung, Offen (+8 more)

### Community 44 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "HaustierId"
Cohesion: 0.17
Nodes (18): Haustier, HaustierId, NeuesHaustier, NaiveDate, Uuid, parse_haustier_id(), haustier_from_row(), query_all_haustiere() (+10 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "Migration"
Cohesion: 0.38
Nodes (6): column_exists(), Migration, rename_column_if_exists(), Error, Option, Transaction

### Community 50 - "leistung_from_row"
Cohesion: 0.17
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 51 - "Preis"
Cohesion: 0.20
Nodes (6): Add, Preis, PreisFehler, Decimal, Output, Self

### Community 52 - "Adresse"
Cohesion: 0.26
Nodes (8): Adresse, domain::Adresse, Ländercode, Error, Example, From, Self, TryFrom

### Community 53 - "LeftMenuLayout.tsx"
Cohesion: 0.27
Nodes (3): MenuEntryData, setupStore, Layout

### Community 54 - "Ländercode"
Cohesion: 0.38
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 55 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 56 - "AnimalStore"
Cohesion: 0.12
Nodes (3): AnimalResponse, RaceResponse, AnimalStore

### Community 57 - "Rechnungsposition"
Cohesion: 0.43
Nodes (3): position_from_leistung(), Rechnungsposition, Decimal

### Community 58 - "StructuredError"
Cohesion: 0.19
Nodes (9): InternalServerError, Self, into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError (+1 more)

### Community 59 - "pages/index.tsx"
Cohesion: 0.25
Nodes (4): Data, Home, nextConfig, !.next

### Community 62 - "base_app_builder"
Cohesion: 0.17
Nodes (12): base_app_builder(), AppBuilder, SetUowProvider, AbrechnungSetup, Arc, Klient, NaiveDate, setup_abrechnung_fixture() (+4 more)

### Community 64 - "KlientErstellung"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellung, Adresse, Error, NaiveDate, Self, TryFrom

### Community 66 - "Integration Test Workflow"
Cohesion: 0.53
Nodes (6): mise, mise-action, Release Workflow, Integration Test Workflow, tauri-action, Unit Test Workflow

### Community 67 - "frontend-legacy/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "src/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 69 - "yams-core"
Cohesion: 0.60
Nodes (6): molting, yams, yams-api, yams-core, yams-persistence, yams-server

### Community 70 - "main"
Cohesion: 0.50
Nodes (4): BackendServerError, Config, main(), Report

### Community 71 - "Abrechnung — Domain-Spezifikation"
Cohesion: 0.22
Nodes (8): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte

### Community 72 - "_app.tsx"
Cohesion: 0.06
Nodes (32): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+24 more)

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
Cohesion: 0.17
Nodes (21): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+13 more)

### Community 89 - "common.rs"
Cohesion: 0.22
Nodes (16): behandlung_from_row(), Row, decimal_to_str(), parse_decimal(), parse_klient_id(), parse_preis(), parse_rechnung_id(), parse_uuid() (+8 more)

### Community 100 - "AddressTable.tsx"
Cohesion: 0.15
Nodes (11): AddressEditTableRowContent, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons, EditState, initialEditState (+3 more)

### Community 101 - "Rechnung"
Cohesion: 0.30
Nodes (13): Rechnung, Rechnungsposition, RechnungStatus, NaiveDate, Option, S, Uuid, Vec (+5 more)

### Community 103 - "TagesabschlussErstellung"
Cohesion: 0.67
Nodes (3): From, TagesabschlussDurchführen, TagesabschlussErstellung

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - "RechnungOffen"
Cohesion: 0.60
Nodes (3): RechnungOffen, Vec, TagesabschlussDurchführen

### Community 111 - "event/index.tsx"
Cohesion: 0.60
Nodes (3): submitDeleteEvent(), deleteEvent(), SubmissionState

### Community 114 - "layout.tsx"
Cohesion: 0.40
Nodes (3): geistMono, geistSans, metadata

## Knowledge Gaps
- **185 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+180 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `schema/leistung.rs`, `Versioned`, `SQLiteRechnungRepository`, `use_cases/abrechnung.rs`, `UseCase`, `EmailAdresse`, `KlientId`, `YamsAppApi`, `YamsApiSpec`, `LeistungId`, `HaustierId`, `KlientErstellen`, `leistung_from_row`, `Adresse`, `Ländercode`, `Rechnungsposition`, `StructuredError`, `KlientErstellung`, `main`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `Klient`, `ProduktErstellen`?**
  _High betweenness centrality (0.167) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `StructuredError`, `.run`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.044) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _185 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `E` be split into smaller, more focused modules?**
  _Cohesion score 0.05025773195876289 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._