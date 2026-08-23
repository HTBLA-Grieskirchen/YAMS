# Graph Report - yams  (2026-08-23)

## Corpus Check
- 203 files · ~303,929 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1727 nodes · 3880 edges · 112 communities (88 shown, 24 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 114 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0b53b98a`
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
- ProduktId
- compilerOptions
- stores/index.tsx
- FakeDatastore
- events/[id]/index.tsx
- RelationStore
- libs/database/index.ts
- EventStore
- String
- GeladeneRechnung
- Versioned<T>
- SQLiteConnection
- live
- UnitOfWorkImpl
- YamsAppApi
- makeRecordForTable
- HaustierRepository
- UseCase
- FakeUnitOfWork
- query
- EmailAdresse
- AddressTable.tsx
- Rechnungsposition
- KlientId
- RepositoryResult
- BehandlungId
- UnitOfWork
- RechnungId
- Clock
- .get_current_version
- YamsApiSpec
- SQLiteKlientRepository
- LeistungId
- SQLiteUnitOfWork
- Versioned
- ClientStore
- ExecutionContext
- BehandlungRepository
- leistung_from_row
- Preis
- Adresse
- use_cases/abrechnung.rs
- Ländercode
- DialogStore
- AnimalStore
- LeistungRepository
- StructuredError
- FakeKlientenRepository
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
- Notifications.tsx
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
- notification.ts
- Rechnung
- .run
- KlientErstellen
- RechnungRepository
- Behandlung
- BehandlungErstellen
- Klient
- ProduktErstellen
- ParticipationStore

## God Nodes (most connected - your core abstractions)
1. `useStore()` - 52 edges
2. `Versioned` - 49 edges
3. `query()` - 42 edges
4. `KlientId` - 38 edges
5. `FakeDatastore` - 33 edges
6. `E` - 31 edges
7. `Preis` - 31 edges
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

## Communities (112 total, 24 thin omitted)

### Community 0 - "E"
Cohesion: 0.06
Nodes (54): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+46 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (41): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+33 more)

### Community 2 - "devDependencies"
Cohesion: 0.05
Nodes (41): babel-plugin-react-compiler, @biomejs/biome, dependencies, next, react, react-dom, devDependencies, babel-plugin-react-compiler (+33 more)

### Community 3 - "biome.json"
Cohesion: 0.05
Nodes (35): source, assist, actions, next, react, files, ignoreUnknown, includes (+27 more)

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
Cohesion: 0.12
Nodes (30): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+22 more)

### Community 8 - "openapi_service"
Cohesion: 0.14
Nodes (14): openapi_service(), C, From, Into, Item, Report, Self, T (+6 more)

### Community 9 - "schema/leistung.rs"
Cohesion: 0.33
Nodes (12): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungStatus, NaiveDate, Option (+4 more)

### Community 10 - "RepositoryError"
Cohesion: 0.26
Nodes (10): RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, AsRef, Path, ResultReport (+2 more)

### Community 11 - "ProduktId"
Cohesion: 0.21
Nodes (13): NeuesProdukt, Produkt, ProduktId, Uuid, produkt_from_row(), Arc, Mutex, Option (+5 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "stores/index.tsx"
Cohesion: 0.06
Nodes (40): queryClient, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, MenuEntryData (+32 more)

### Community 14 - "FakeDatastore"
Cohesion: 0.13
Nodes (14): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, Arc, Behandlung, Clone, Mutex (+6 more)

### Community 15 - "events/[id]/index.tsx"
Cohesion: 0.21
Nodes (13): EventsUsages, EventDetailItem, EventOverviewItem, askSubmitDeleteEvent(), submitDeleteEvent(), MainNavbar, NavbarMenuEntry, deleteEvent() (+5 more)

### Community 17 - "libs/database/index.ts"
Cohesion: 0.10
Nodes (17): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+9 more)

### Community 18 - "EventStore"
Cohesion: 0.12
Nodes (3): EventResponse, SeminarResponse, EventStore

### Community 19 - "String"
Cohesion: 0.24
Nodes (11): From, String, EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, MobilnummerValidierungsfehler, AsRef, Error (+3 more)

### Community 20 - "GeladeneRechnung"
Cohesion: 0.19
Nodes (12): GeladeneRechnung, geladene_rechnung_from_parts(), RechnungRowData, Arc, Mutex, NaiveDate, Option, RepositoryResult (+4 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (9): Clone, Deref, Formatter, T, Target, Versioned<T>, DerefMut, PartialEq (+1 more)

### Community 22 - "SQLiteConnection"
Cohesion: 0.23
Nodes (10): Connection, InstanceType, Arc, Deref, Mutex, Target, SQLiteConnection, Database (+2 more)

### Community 23 - "live"
Cohesion: 0.35
Nodes (3): live(), ano(), no()

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "YamsAppApi"
Cohesion: 0.06
Nodes (44): Arc, Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, ResultReport (+36 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.12
Nodes (22): AnimalAddItem, AnimalComboBox, patchAnimal(), Result, patchRace(), createSeminar(), ensureSeminar(), Address (+14 more)

### Community 27 - "HaustierRepository"
Cohesion: 0.13
Nodes (6): HaustierRepository, KlientRepository, ProduktRepository, Send, Sync, Debug

### Community 28 - "UseCase"
Cohesion: 0.18
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - "FakeUnitOfWork"
Cohesion: 0.28
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 30 - "query"
Cohesion: 0.13
Nodes (26): clientSearched(), EventParticipants, participationSearchedClient(), SmallSearchField, ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent() (+18 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "AddressTable.tsx"
Cohesion: 0.09
Nodes (22): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), AddressEditTableRowContent (+14 more)

### Community 33 - "Rechnungsposition"
Cohesion: 0.16
Nodes (13): position_from_leistung(), Rechnung, RechnungBezahltMarker, RechnungFehler, RechnungOffenMarker, Rechnungsposition, Decimal, NaiveDate (+5 more)

### Community 34 - "KlientId"
Cohesion: 0.26
Nodes (13): Haustier, HaustierId, NeuesHaustier, NaiveDate, Uuid, Klient, KlientId, NeuerKlient (+5 more)

### Community 35 - "RepositoryResult"
Cohesion: 0.30
Nodes (5): FakeHaustiereRepository, FakeRechnungenRepository, Haustier, RepositoryResult, Vec

### Community 36 - "BehandlungId"
Cohesion: 0.21
Nodes (13): Behandlung, BehandlungId, NeueBehandlung, Uuid, behandlung_from_row(), Arc, Behandlung, Mutex (+5 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "RechnungId"
Cohesion: 0.18
Nodes (4): LeistungAbgerechnet, Rechnung<S>, RechnungId, parse_rechnung_id()

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
Cohesion: 0.11
Nodes (17): Abgerechnet, GeladeneLeistung, Leistung, Leistung<S>, LeistungFehler, LeistungId, LeistungQuelle, NeueLeistung (+9 more)

### Community 44 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "Versioned"
Cohesion: 0.26
Nodes (13): Versioned, haustier_from_row(), query_all_haustiere(), Arc, Haustier, Mutex, Option, RepositoryResult (+5 more)

### Community 47 - "ExecutionContext"
Cohesion: 0.29
Nodes (5): ExecutionContext, Arc, LeistungOffen, LeistungAusBehandlungBuchen, Error

### Community 50 - "leistung_from_row"
Cohesion: 0.24
Nodes (11): leistung_from_row(), leistung_offen_from_row(), Arc, Mutex, NaiveDate, Option, RepositoryResult, Row (+3 more)

### Community 51 - "Preis"
Cohesion: 0.33
Nodes (4): Preis, PreisFehler, Decimal, Self

### Community 52 - "Adresse"
Cohesion: 0.26
Nodes (8): Adresse, domain::Adresse, Laendercode, Error, Example, From, Self, TryFrom

### Community 53 - "use_cases/abrechnung.rs"
Cohesion: 0.18
Nodes (14): RechnungOffen, BehandlungErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler, ProduktErstellenFehler (+6 more)

### Community 54 - "Ländercode"
Cohesion: 0.38
Nodes (4): Adresse, LaendercodeValidierungsfehler, Ländercode, Self

### Community 55 - "DialogStore"
Cohesion: 0.27
Nodes (3): DialogInfo, DialogInfoType, DialogStore

### Community 56 - "AnimalStore"
Cohesion: 0.12
Nodes (3): AnimalResponse, RaceResponse, AnimalStore

### Community 58 - "StructuredError"
Cohesion: 0.19
Nodes (9): InternalServerError, Self, into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError (+1 more)

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
Nodes (8): Abrechnung — Domain-Spezifikation, Aggregates, Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte

### Community 72 - "Notifications.tsx"
Cohesion: 0.15
Nodes (11): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationBehaviour, NotificationInfo, NotificationInfoType (+3 more)

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
Cohesion: 0.14
Nodes (24): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+16 more)

### Community 89 - "common.rs"
Cohesion: 0.16
Nodes (22): decimal_to_str(), format_naive_date(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_naive_date(), parse_preis(), parse_uuid() (+14 more)

### Community 100 - "notification.ts"
Cohesion: 0.33
Nodes (4): NotificationActions, NotificationContent, TODO: Add possibility to also display notification on host system if in Tauri, NotificationEntry

### Community 101 - "Rechnung"
Cohesion: 0.31
Nodes (12): Rechnung, Rechnungsposition, RechnungStatus, NaiveDate, S, Uuid, Vec, schema_position_from_domain() (+4 more)

### Community 102 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 103 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 106 - "Behandlung"
Cohesion: 0.83
Nodes (3): Behandlung, Uuid, schema_behandlung_from_domain()

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

## Knowledge Gaps
- **188 isolated node(s):** `molting`, `UnitOfWork<'a>`, `LeistungFehler`, `RechnungOffenMarker`, `RechnungBezahltMarker` (+183 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `schema/leistung.rs`, `ProduktId`, `GeladeneRechnung`, `YamsAppApi`, `UseCase`, `FakeUnitOfWork`, `EmailAdresse`, `Rechnungsposition`, `KlientId`, `BehandlungId`, `YamsApiSpec`, `LeistungId`, `Adresse`, `use_cases/abrechnung.rs`, `Ländercode`, `StructuredError`, `KlientErstellung`, `main`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `KlientErstellen`, `Behandlung`, `BehandlungErstellen`, `Klient`, `ProduktErstellen`?**
  _High betweenness centrality (0.168) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `StructuredError`, `.run`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_fuer_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `LeistungFehler` to the rest of the system?**
  _188 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `E` be split into smaller, more focused modules?**
  _Cohesion score 0.05694586312563841 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._