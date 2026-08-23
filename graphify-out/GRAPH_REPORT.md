# Graph Report - yams  (2026-08-24)

## Corpus Check
- 211 files · ~307,797 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1890 nodes · 4252 edges · 106 communities (88 shown, 18 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 117 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `78193a4c`
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
- model/event.ts
- RepositoryError
- UpMigration
- compilerOptions
- _app.tsx
- ProduktId
- participation/index.tsx
- App
- apply_up_migrations
- E
- EmailAdresse
- SQLiteRechnungRepository
- Versioned<T>
- BehandlungId
- libs/database/index.ts
- UnitOfWorkImpl
- KlientId
- makeRecordForTable
- KlientRepository
- UseCase
- .run
- query
- EmailAdresse
- addresses/index.tsx
- Rechnungsposition
- NeuerKlient
- LeistungId
- SQLiteConnection
- UnitOfWork
- YamsAppApi
- Clock
- .get_current_version
- YamsApiSpec
- Ländercode
- schema/leistung.rs
- SQLiteUnitOfWork
- Versioned
- .contextualize_with
- KlientErstellen
- ExecutionContext
- leistung_from_row
- Preis
- Adresse
- LeftMenuLayout.tsx
- HaustierErstellung
- DialogStore
- AnimalStore
- BehandlungRepository
- StructuredError
- HaustierRepository
- DatabaseConnection
- base_app_builder
- AddressStore
- Haustier
- SettingsStore
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- src/api/schema.d.ts
- yams-core
- Abrechnung — Domain-Spezifikation
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
- common.rs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- stores/index.tsx
- Rechnung
- LeistungRepository
- RechnungRepository
- Behandlung
- Klient
- Produkt

## God Nodes (most connected - your core abstractions)
1. `useStore()` - 52 edges
2. `Versioned` - 49 edges
3. `query()` - 42 edges
4. `KlientId` - 38 edges
5. `FakeDatastore` - 33 edges
6. `E` - 32 edges
7. `Preis` - 32 edges
8. `YamsAppApi` - 30 edges
9. `makeRecordForTable()` - 29 edges
10. `Record` - 28 edges

## Surprising Connections (you probably didn't know these)
- `Config` --references--> `String`  [EXTRACTED]
  backend/server/src/main.rs → crates/yams-api/src/errors/internal_error.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `alle_haustiere()` --references--> `YamsAppApi`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/api/mod.rs
- `behandlung_erstellen()` --references--> `YamsAppApi`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/api/mod.rs
- `haustier_by_id()` --references--> `YamsAppApi`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/api/mod.rs

## Import Cycles
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **CI Release Pipeline** — github_workflows_publish_release_workflow, github_workflows_smoke_tests_workflow, github_workflows_unit_tests_workflow, github_workflows_tauri_action, github_workflows_mise_action [EXTRACTED 1.00]
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **Frontend Backend Communication Bridge** — specs_backend_hexagonal_backend_client, specs_backend_hexagonal_http_adapter, specs_backend_hexagonal_tauri_adapter, specs_backend_hexagonal_tanstack_query, specs_frontend_migration_yamsconfig [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (106 total, 18 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (41): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+33 more)

### Community 2 - "devDependencies"
Cohesion: 0.04
Nodes (45): babel-plugin-react-compiler, @biomejs/biome, dependencies, next, openapi-fetch, react, react-dom, @tauri-apps/api (+37 more)

### Community 3 - "biome.json"
Cohesion: 0.06
Nodes (32): source, assist, actions, next, react, files, ignoreUnknown, includes (+24 more)

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

### Community 8 - "String"
Cohesion: 0.14
Nodes (33): From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen(), leistung_aus_behandlung_buchen() (+25 more)

### Community 9 - "model/event.ts"
Cohesion: 0.11
Nodes (8): submitDeleteEvent(), createEvent(), deleteEvent(), updateEvent(), Event, EventResponse, Seminar, EventStore

### Community 10 - "RepositoryError"
Cohesion: 0.26
Nodes (10): RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, AsRef, Path, ResultReport (+2 more)

### Community 11 - "UpMigration"
Cohesion: 0.15
Nodes (12): Send, Sync, UpMigration, Migration, Error, Option, Transaction, Migration (+4 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "_app.tsx"
Cohesion: 0.06
Nodes (33): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+25 more)

### Community 14 - "ProduktId"
Cohesion: 0.20
Nodes (14): NeuesProdukt, Produkt, ProduktId, Decimal, Uuid, produkt_from_row(), Arc, Mutex (+6 more)

### Community 15 - "participation/index.tsx"
Cohesion: 0.19
Nodes (13): EventParticipants, participationSearchedClient(), ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation() (+5 more)

### Community 16 - "App"
Cohesion: 0.16
Nodes (13): Self, App, Arc, Box, F, O, ResultReport, T (+5 more)

### Community 17 - "apply_up_migrations"
Cohesion: 0.25
Nodes (8): apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Option, DoubleEndedIterator, Iterator

### Community 18 - "E"
Cohesion: 0.29
Nodes (12): AppliableMigration, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>> (+4 more)

### Community 19 - "EmailAdresse"
Cohesion: 0.25
Nodes (9): EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, MobilnummerValidierungsfehler, AsRef, Error, S, Self (+1 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.19
Nodes (16): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "BehandlungId"
Cohesion: 0.20
Nodes (14): Behandlung, BehandlungId, NeueBehandlung, Decimal, Uuid, behandlung_from_row(), Arc, Behandlung (+6 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.07
Nodes (20): CompatibilityResult, DatabaseError, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented (+12 more)

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "KlientId"
Cohesion: 0.11
Nodes (25): Haustier, HaustierId, NeuesHaustier, NaiveDate, Uuid, KlientId, LeistungIn, LeistungIn<S> (+17 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.09
Nodes (26): AnimalAddItem, AnimalComboBox, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow (+18 more)

### Community 28 - "UseCase"
Cohesion: 0.18
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "query"
Cohesion: 0.22
Nodes (14): clientSearched(), SmallSearchField, deleteAddress(), deleteClientRelation(), relateClients(), updateClientRelation(), query(), deleteRace() (+6 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "addresses/index.tsx"
Cohesion: 0.14
Nodes (12): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), AddressTableHeader (+4 more)

### Community 33 - "Rechnungsposition"
Cohesion: 0.14
Nodes (17): Bezahlt, Offen, position_from_leistung(), Rechnung, RechnungBezahlt, RechnungFehler, RechnungIn, Rechnungsposition (+9 more)

### Community 34 - "NeuerKlient"
Cohesion: 0.43
Nodes (7): Klient, NeuerKlient, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Uuid

### Community 35 - "LeistungId"
Cohesion: 0.13
Nodes (15): Abgerechnet, Leistung, LeistungAbgerechnet, LeistungId, LeistungOffen, LeistungQuelle, Offen, Decimal (+7 more)

### Community 36 - "SQLiteConnection"
Cohesion: 0.23
Nodes (10): Connection, InstanceType, Arc, Deref, Mutex, Target, SQLiteConnection, Database (+2 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "YamsAppApi"
Cohesion: 0.12
Nodes (20): Arc, Behandlung, BehandlungErstellung, Haustier, HaustierErstellung, Klient, KlientErstellung, Leistung (+12 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "YamsApiSpec"
Cohesion: 0.05
Nodes (45): BackendServerError, Config, main(), Report, KlientErstellen, KlientErstellung, Adresse, Error (+37 more)

### Community 42 - "Ländercode"
Cohesion: 0.38
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (14): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungStatus, Decimal, NaiveDate (+6 more)

### Community 44 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "Versioned"
Cohesion: 0.05
Nodes (54): Versioned, FakeBehandlungenRepository, FakeDatastore, FakeHaustiereRepository, FakeKlientenRepository, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository (+46 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.21
Nodes (10): ErrorReportExt, Result<T, E>, C, Error, Report, Send, Sync, T (+2 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "ExecutionContext"
Cohesion: 0.16
Nodes (10): ExecutionContext, Arc, RechnungOffen, BehandlungErstellen, ProduktErstellen, Behandlung, Error, Produkt (+2 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.22
Nodes (12): leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex, NaiveDate, Option, RepositoryResult (+4 more)

### Community 51 - "Preis"
Cohesion: 0.15
Nodes (7): Add, Preis, PreisFehler, Decimal, Output, Self, RechnungIn<S>

### Community 52 - "Adresse"
Cohesion: 0.26
Nodes (8): Adresse, domain::Adresse, Ländercode, Error, Example, From, Self, TryFrom

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "DialogStore"
Cohesion: 0.27
Nodes (3): DialogInfo, DialogInfoType, DialogStore

### Community 56 - "AnimalStore"
Cohesion: 0.12
Nodes (3): AnimalResponse, RaceResponse, AnimalStore

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "HaustierRepository"
Cohesion: 0.16
Nodes (5): HaustierRepository, ProduktRepository, Send, Sync, Debug

### Community 60 - "DatabaseConnection"
Cohesion: 0.16
Nodes (6): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection

### Community 62 - "base_app_builder"
Cohesion: 0.16
Nodes (13): base_app_builder(), AppBuilder, SetUowProvider, AbrechnungSetup, Arc, Klient, NaiveDate, setup_abrechnung_fixture() (+5 more)

### Community 63 - "AddressStore"
Cohesion: 0.15
Nodes (3): AddressResponse, AddressStore, Store

### Community 64 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 66 - "Integration Test Workflow"
Cohesion: 0.53
Nodes (6): mise, mise-action, Release Workflow, Integration Test Workflow, tauri-action, Unit Test Workflow

### Community 67 - "frontend-legacy/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "src/api/schema.d.ts"
Cohesion: 0.15
Nodes (12): components, $defs, FlattenedDeepRequired, leistungQuelle_LeistungQuelleBehandlungTypValues, leistungQuelle_LeistungQuelleManuellTypValues, leistungQuelle_LeistungQuelleProduktTypValues, leistungStatusValues, operations (+4 more)

### Community 69 - "yams-core"
Cohesion: 0.48
Nodes (7): molting, yams, yams-api, yams-core, yams-fakes, yams-persistence, yams-server

### Community 71 - "Abrechnung — Domain-Spezifikation"
Cohesion: 0.22
Nodes (8): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte

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
Cohesion: 0.11
Nodes (26): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+18 more)

### Community 80 - "api/types.ts"
Cohesion: 0.06
Nodes (43): ApiError, HttpYamsApi, JsonClient, unwrap(), createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi() (+35 more)

### Community 89 - "common.rs"
Cohesion: 0.20
Nodes (18): decimal_to_str(), format_naive_date(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_naive_date(), parse_preis(), parse_rechnung_id() (+10 more)

### Community 100 - "stores/index.tsx"
Cohesion: 0.09
Nodes (34): AddressEditTableRowContent, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons, EditState, EventsUsages (+26 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 106 - "Behandlung"
Cohesion: 0.70
Nodes (4): Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - "Produkt"
Cohesion: 0.60
Nodes (4): Produkt, Decimal, Uuid, schema_produkt_from_domain()

## Knowledge Gaps
- **199 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+194 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `ProduktId`, `EmailAdresse`, `SQLiteRechnungRepository`, `BehandlungId`, `KlientId`, `UseCase`, `EmailAdresse`, `Rechnungsposition`, `NeuerKlient`, `LeistungId`, `YamsApiSpec`, `Ländercode`, `schema/leistung.rs`, `Versioned`, `KlientErstellen`, `ExecutionContext`, `Adresse`, `HaustierErstellung`, `StructuredError`, `Haustier`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `Behandlung`, `Klient`, `Produkt`?**
  _High betweenness centrality (0.158) - this node is a cross-community bridge._
- **Why does `!.next` connect `biome.json` to `api/types.ts`, `stores/index.tsx`?**
  _High betweenness centrality (0.044) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `.add_dyn`, `String`, `.contextualize_with`, `apply_up_migrations`, `StructuredError`, `.run`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _199 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.043478260869565216 - nodes in this community are weakly interconnected._