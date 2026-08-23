# Graph Report - yams  (2026-08-24)

## Corpus Check
- 204 files · ~304,751 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1750 nodes · 3930 edges · 113 communities (87 shown, 26 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 116 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `dc54e358`
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
- stores/index.tsx
- Versioned
- participation/index.tsx
- RelationStore
- libs/database/index.ts
- App
- EmailAdresse
- SQLiteRechnungRepository
- Versioned<T>
- BehandlungId
- live
- UnitOfWorkImpl
- use_cases/abrechnung.rs
- makeRecordForTable
- KlientRepository
- UseCase
- .run
- relations/index.tsx
- EmailAdresse
- addresses/index.tsx
- Rechnungsposition
- KlientId
- LeistungQuelle
- UnitOfWork
- YamsAppApi
- Clock
- .get_current_version
- YamsApiSpec
- HaustierErstellung
- LeistungId
- SQLiteUnitOfWork
- HaustierId
- ClientStore
- KlientErstellen
- ExecutionContext
- SQLiteLeistungRepository
- Preis
- String
- LeftMenuLayout.tsx
- RechnungIn<S>
- DialogStore
- AnimalStore
- BehandlungRepository
- StructuredError
- HaustierRepository
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
- parse_preis
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
- requests/abrechnung.rs
- RechnungRepository
- Behandlung
- Haustier
- Klient
- Produkt
- ProduktErstellen
- ParticipationStore

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
- `alle_haustiere()` --references--> `YamsAppApi`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/api/mod.rs
- `haustier_erstellen()` --references--> `YamsAppApi`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/api/mod.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  crates/yams-api/src/bin/export_spec.rs → crates/yams-api/src/spec.rs

## Import Cycles
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **CI Release Pipeline** — github_workflows_publish_release_workflow, github_workflows_smoke_tests_workflow, github_workflows_unit_tests_workflow, github_workflows_tauri_action, github_workflows_mise_action [EXTRACTED 1.00]
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **Frontend Backend Communication Bridge** — specs_backend_hexagonal_backend_client, specs_backend_hexagonal_http_adapter, specs_backend_hexagonal_tauri_adapter, specs_backend_hexagonal_tanstack_query, specs_frontend_migration_yamsconfig [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (113 total, 26 thin omitted)

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
Cohesion: 0.32
Nodes (13): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungStatus, Decimal, NaiveDate (+5 more)

### Community 10 - "RepositoryError"
Cohesion: 0.14
Nodes (20): Connection, RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, InstanceType, Arc (+12 more)

### Community 11 - "query"
Cohesion: 0.21
Nodes (11): AnimalAddItem, AnimalRow, deleteAddress(), deleteAnimal(), patchAnimal(), query(), deleteRace(), patchRace() (+3 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "stores/index.tsx"
Cohesion: 0.08
Nodes (32): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+24 more)

### Community 14 - "Versioned"
Cohesion: 0.05
Nodes (56): Versioned, NeuesProdukt, Produkt, ProduktId, Decimal, Uuid, FakeBehandlungenRepository, FakeDatastore (+48 more)

### Community 15 - "participation/index.tsx"
Cohesion: 0.19
Nodes (14): EventParticipants, participationSearchedClient(), SmallSearchField, ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation() (+6 more)

### Community 17 - "libs/database/index.ts"
Cohesion: 0.10
Nodes (17): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+9 more)

### Community 18 - "App"
Cohesion: 0.16
Nodes (13): Self, App, Arc, Box, F, O, ResultReport, T (+5 more)

### Community 19 - "EmailAdresse"
Cohesion: 0.25
Nodes (9): EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, MobilnummerValidierungsfehler, AsRef, Error, S, Self (+1 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.18
Nodes (15): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), RechnungRowData, Arc, Mutex, NaiveDate, Option (+7 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "BehandlungId"
Cohesion: 0.20
Nodes (14): Behandlung, BehandlungId, NeueBehandlung, Decimal, Uuid, behandlung_from_row(), Arc, Behandlung (+6 more)

### Community 23 - "live"
Cohesion: 0.36
Nodes (3): live(), ano(), no()

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "use_cases/abrechnung.rs"
Cohesion: 0.16
Nodes (16): BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler, ProduktErstellenFehler (+8 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.09
Nodes (13): createSeminar(), ensureSeminar(), Address, DatabaseObject, DatabaseResponse, isRecord(), makeRecord(), makeRecordForTable() (+5 more)

### Community 28 - "UseCase"
Cohesion: 0.18
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "relations/index.tsx"
Cohesion: 0.14
Nodes (19): AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, clientSearched(), deleteClientRelation(), relateClients() (+11 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "addresses/index.tsx"
Cohesion: 0.17
Nodes (9): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), Addresses (+1 more)

### Community 33 - "Rechnungsposition"
Cohesion: 0.14
Nodes (18): Bezahlt, Offen, position_from_leistung(), Rechnung, RechnungBezahlt, RechnungFehler, RechnungIn, RechnungOffen (+10 more)

### Community 34 - "KlientId"
Cohesion: 0.33
Nodes (8): Klient, KlientId, NeuerKlient, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Uuid

### Community 35 - "LeistungQuelle"
Cohesion: 0.17
Nodes (8): LeistungIn, LeistungIn<S>, LeistungQuelle, NeueLeistung, Decimal, NaiveDate, Option, S

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "YamsAppApi"
Cohesion: 0.18
Nodes (13): Arc, Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, ResultReport (+5 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "YamsApiSpec"
Cohesion: 0.18
Nodes (12): Behandlung, Haustier, Klient, Leistung, Path, Produkt, Rechnung, Uuid (+4 more)

### Community 42 - "HaustierErstellung"
Cohesion: 0.21
Nodes (12): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid, alle_haustiere() (+4 more)

### Community 43 - "LeistungId"
Cohesion: 0.17
Nodes (12): Abgerechnet, Leistung, LeistungAbgerechnet, LeistungId, Offen, From, Self, Uuid (+4 more)

### Community 44 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "HaustierId"
Cohesion: 0.18
Nodes (17): Haustier, HaustierId, NeuesHaustier, NaiveDate, Uuid, haustier_from_row(), query_all_haustiere(), Arc (+9 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "ExecutionContext"
Cohesion: 0.29
Nodes (5): ExecutionContext, Arc, LeistungOffen, LeistungAusBehandlungBuchen, Error

### Community 50 - "SQLiteLeistungRepository"
Cohesion: 0.21
Nodes (11): leistung_offen_from_row(), Arc, Leistung, Mutex, NaiveDate, Option, RepositoryResult, Row (+3 more)

### Community 51 - "Preis"
Cohesion: 0.24
Nodes (6): Add, Preis, PreisFehler, Decimal, Output, Self

### Community 52 - "String"
Cohesion: 0.13
Nodes (15): From, Self, String, Adresse, domain::Adresse, Ländercode, Error, Example (+7 more)

### Community 55 - "DialogStore"
Cohesion: 0.27
Nodes (3): DialogInfo, DialogInfoType, DialogStore

### Community 58 - "StructuredError"
Cohesion: 0.19
Nodes (9): InternalServerError, into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, ValidationError (+1 more)

### Community 59 - "HaustierRepository"
Cohesion: 0.16
Nodes (5): HaustierRepository, ProduktRepository, Send, Sync, Debug

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

### Community 72 - "Notifications.tsx"
Cohesion: 0.14
Nodes (12): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationBehaviour, NotificationInfo, NotificationInfoType (+4 more)

### Community 73 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 74 - "BackendClient"
Cohesion: 0.50
Nodes (4): BackendClient, MobX, TanStack Query, yamsconfig.json

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 77 - "parse_preis"
Cohesion: 0.21
Nodes (12): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusProduktBuchen, LeistungManuellErfassen, parse_preis(), ProduktErstellen, ProduktErstellung (+4 more)

### Community 89 - "common.rs"
Cohesion: 0.19
Nodes (20): decimal_to_str(), format_naive_date(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_naive_date(), parse_preis(), parse_rechnung_id() (+12 more)

### Community 100 - "AddressTable.tsx"
Cohesion: 0.09
Nodes (30): AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons (+22 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 103 - "requests/abrechnung.rs"
Cohesion: 0.38
Nodes (10): LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Decimal, From, NaiveDate, Option, Uuid (+2 more)

### Community 106 - "Behandlung"
Cohesion: 0.70
Nodes (4): Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - "Produkt"
Cohesion: 0.60
Nodes (4): Produkt, Decimal, Uuid, schema_produkt_from_domain()

## Knowledge Gaps
- **185 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+180 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **26 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `schema/leistung.rs`, `Versioned`, `EmailAdresse`, `SQLiteRechnungRepository`, `BehandlungId`, `use_cases/abrechnung.rs`, `UseCase`, `EmailAdresse`, `Rechnungsposition`, `KlientId`, `LeistungQuelle`, `YamsApiSpec`, `HaustierErstellung`, `LeistungId`, `HaustierId`, `KlientErstellen`, `StructuredError`, `KlientErstellung`, `main`, `parse_preis`, `common.rs`, `Rechnung`, `requests/abrechnung.rs`, `Behandlung`, `Haustier`, `Klient`, `Produkt`, `ProduktErstellen`?**
  _High betweenness centrality (0.168) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `StructuredError`, `.run`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `KlientId` connect `KlientId` to `common.rs`, `Rechnungsposition`, `LeistungQuelle`, `YamsAppApi`, `HaustierErstellung`, `LeistungId`, `HaustierId`, `parse_preis`, `Versioned`, `ExecutionContext`, `SQLiteRechnungRepository`, `RechnungIn<S>`, `use_cases/abrechnung.rs`, `UseCase`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _185 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `E` be split into smaller, more focused modules?**
  _Cohesion score 0.05694586312563841 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._