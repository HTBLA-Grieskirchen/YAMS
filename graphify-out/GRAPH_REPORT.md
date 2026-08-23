# Graph Report - yams  (2026-08-23)

## Corpus Check
- 199 files · ~302,355 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1658 nodes · 3699 edges · 111 communities (89 shown, 22 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 111 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3bb621de`
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
- query
- RepositoryError
- ProduktId
- compilerOptions
- stores/index.tsx
- FakeDatastore
- NotificationStore
- relations/index.tsx
- libs/database/index.ts
- EventStore
- EmailAdresse
- rechnung_from_row
- Versioned<T>
- AddressTable.tsx
- live
- UnitOfWorkImpl
- YamsAppApi
- makeRecordForTable
- BehandlungRepository
- UseCase
- FakeUnitOfWork
- participation/index.tsx
- EmailAdresse
- addresses/index.tsx
- RechnungId
- KlientId
- RepositoryResult
- BehandlungId
- UnitOfWork
- LeftMenuLayout.tsx
- Clock
- .get_current_version
- YamsApiSpec
- SQLiteKlientRepository
- Leistung
- SQLiteUnitOfWork
- Versioned
- ClientStore
- ParticipationStore
- RelationStore
- leistung_from_row
- Preis
- Adresse
- use_cases/abrechnung.rs
- Ländercode
- DialogStore
- AnimalStore
- Klient
- StructuredError
- LeistungRepository
- DatabaseConnection
- String
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
- notification.ts
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
- ExecutionContext
- Rechnung
- .run
- KlientErstellen
- RechnungRepository
- commands.rs
- Behandlung
- Produkt
- KlientRepository
- TagesabschlussDurchfuehren

## God Nodes (most connected - your core abstractions)
1. `useStore()` - 52 edges
2. `Versioned` - 49 edges
3. `query()` - 42 edges
4. `FakeDatastore` - 33 edges
5. `E` - 31 edges
6. `KlientId` - 30 edges
7. `makeRecordForTable()` - 29 edges
8. `Record` - 28 edges
9. `Client` - 26 edges
10. `RepositoryError` - 24 edges

## Surprising Connections (you probably didn't know these)
- `Config` --references--> `String`  [EXTRACTED]
  backend/server/src/main.rs → crates/yams-api/src/errors/internal_error.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `create_animal()` --references--> `YamsAppApi`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/api/mod.rs
- `get_animals()` --references--> `YamsAppApi`  [EXTRACTED]
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

## Communities (111 total, 22 thin omitted)

### Community 0 - "E"
Cohesion: 0.06
Nodes (53): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+45 more)

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

### Community 9 - "query"
Cohesion: 0.18
Nodes (16): AnimalAddItem, AnimalComboBox, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow (+8 more)

### Community 10 - "RepositoryError"
Cohesion: 0.14
Nodes (20): Connection, RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, InstanceType, Arc (+12 more)

### Community 11 - "ProduktId"
Cohesion: 0.21
Nodes (13): NeuesProdukt, Produkt, ProduktId, Uuid, produkt_from_row(), Arc, Mutex, Option (+5 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "stores/index.tsx"
Cohesion: 0.07
Nodes (44): queryClient, EventsUsages, EventDetailItem, EventOverviewItem, askSubmitDeleteEvent(), submitDeleteEvent(), LanguagePicker, MainMenu (+36 more)

### Community 14 - "FakeDatastore"
Cohesion: 0.13
Nodes (15): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, Arc, Behandlung, Clone, Leistung (+7 more)

### Community 15 - "NotificationStore"
Cohesion: 0.27
Nodes (4): NotificationInfo, NotificationInfoType, NotificationEntry, NotificationStore

### Community 16 - "relations/index.tsx"
Cohesion: 0.22
Nodes (11): EventParticipants, participationSearchedClient(), SmallSearchField, deleteClientRelation(), relateClients(), updateClientRelation(), AddRelationDialog, ClientRelations (+3 more)

### Community 17 - "libs/database/index.ts"
Cohesion: 0.10
Nodes (17): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+9 more)

### Community 18 - "EventStore"
Cohesion: 0.12
Nodes (3): EventResponse, SeminarResponse, EventStore

### Community 19 - "EmailAdresse"
Cohesion: 0.27
Nodes (8): EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, AsRef, Error, S, Self, TryFrom

### Community 20 - "rechnung_from_row"
Cohesion: 0.23
Nodes (13): parse_preis(), load_positionen(), rechnung_from_row(), Arc, Mutex, Option, Rechnung, RepositoryResult (+5 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (9): Clone, Deref, Formatter, T, Target, Versioned<T>, DerefMut, PartialEq (+1 more)

### Community 22 - "AddressTable.tsx"
Cohesion: 0.12
Nodes (13): AddressEditTableRowContent, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons, EditState, initialEditState (+5 more)

### Community 23 - "live"
Cohesion: 0.35
Nodes (3): live(), ano(), no()

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "YamsAppApi"
Cohesion: 0.06
Nodes (46): Arc, Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, ResultReport (+38 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.13
Nodes (16): createSeminar(), ensureSeminar(), Address, AddressResponse, Client, DatabaseObject, DatabaseResponse, isRecord() (+8 more)

### Community 27 - "BehandlungRepository"
Cohesion: 0.13
Nodes (6): BehandlungRepository, HaustierRepository, ProduktRepository, Send, Sync, Debug

### Community 28 - "UseCase"
Cohesion: 0.18
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - "FakeUnitOfWork"
Cohesion: 0.28
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 30 - "participation/index.tsx"
Cohesion: 0.30
Nodes (10): clientSearched(), ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation() (+2 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "addresses/index.tsx"
Cohesion: 0.14
Nodes (12): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), AddressTableHeader (+4 more)

### Community 33 - "RechnungId"
Cohesion: 0.35
Nodes (9): Rechnung, RechnungFehler, RechnungId, Rechnungsposition, RechnungStatus, Leistung, NaiveDate, Uuid (+1 more)

### Community 34 - "KlientId"
Cohesion: 0.18
Nodes (15): Haustier, HaustierId, NeuesHaustier, NaiveDate, Uuid, Klient, KlientId, NeuerKlient (+7 more)

### Community 35 - "RepositoryResult"
Cohesion: 0.28
Nodes (6): FakeHaustiereRepository, FakeRechnungenRepository, Haustier, Rechnung, RepositoryResult, Vec

### Community 36 - "BehandlungId"
Cohesion: 0.21
Nodes (13): Behandlung, BehandlungId, NeueBehandlung, Uuid, behandlung_from_row(), Arc, Behandlung, Mutex (+5 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

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

### Community 43 - "Leistung"
Cohesion: 0.36
Nodes (8): Leistung, LeistungFehler, LeistungQuelle, LeistungStatus, NeueLeistung, NaiveDate, Option, Uuid

### Community 44 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "Versioned"
Cohesion: 0.26
Nodes (13): Versioned, haustier_from_row(), query_all_haustiere(), Arc, Haustier, Mutex, Option, RepositoryResult (+5 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.23
Nodes (12): LeistungId, leistung_from_row(), Arc, Leistung, Mutex, NaiveDate, Option, RepositoryResult (+4 more)

### Community 51 - "Preis"
Cohesion: 0.47
Nodes (5): Preis, PreisFehler, Decimal, Self, preis_to_str()

### Community 52 - "Adresse"
Cohesion: 0.26
Nodes (8): Adresse, domain::Adresse, Laendercode, Error, Example, From, Self, TryFrom

### Community 53 - "use_cases/abrechnung.rs"
Cohesion: 0.21
Nodes (13): BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler, ProduktErstellenFehler (+5 more)

### Community 54 - "Ländercode"
Cohesion: 0.27
Nodes (8): Adresse, LaendercodeValidierungsfehler, Ländercode, AsRef, Error, S, Self, TryFrom

### Community 55 - "DialogStore"
Cohesion: 0.27
Nodes (3): DialogInfo, DialogInfoType, DialogStore

### Community 56 - "AnimalStore"
Cohesion: 0.12
Nodes (3): AnimalResponse, RaceResponse, AnimalStore

### Community 57 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 61 - "String"
Cohesion: 0.29
Nodes (5): InternalServerError, From, Self, String, MobilnummerValidierungsfehler

### Community 62 - "base_app_builder"
Cohesion: 0.25
Nodes (5): base_app_builder(), AppBuilder, SetUowProvider, test_haustier(), test_klient()

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

### Community 72 - "notification.ts"
Cohesion: 0.21
Nodes (11): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+3 more)

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
Nodes (19): format_naive_date(), leistung_status_from_str(), leistung_status_to_str(), parse_haustier_id(), parse_klient_id(), parse_naive_date(), parse_rechnung_id(), parse_uuid() (+11 more)

### Community 100 - "ExecutionContext"
Cohesion: 0.23
Nodes (7): ExecutionContext, Arc, BehandlungErstellen, ProduktErstellen, Behandlung, Error, Produkt

### Community 101 - "Rechnung"
Cohesion: 0.46
Nodes (7): Rechnung, Rechnungsposition, RechnungStatus, NaiveDate, Uuid, Vec, schema_rechnung_from_domain()

### Community 102 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 103 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 105 - "commands.rs"
Cohesion: 0.53
Nodes (5): AnimalCreation, create_animal(), get_animals(), State, Vec

### Community 106 - "Behandlung"
Cohesion: 0.83
Nodes (3): Behandlung, Uuid, schema_behandlung_from_domain()

### Community 107 - "Produkt"
Cohesion: 0.83
Nodes (3): Produkt, Uuid, schema_produkt_from_domain()

### Community 110 - "TagesabschlussDurchfuehren"
Cohesion: 0.83
Nodes (3): Rechnung, Vec, TagesabschlussDurchfuehren

## Knowledge Gaps
- **184 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+179 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `ProduktId`, `EmailAdresse`, `YamsAppApi`, `UseCase`, `FakeUnitOfWork`, `EmailAdresse`, `RechnungId`, `KlientId`, `BehandlungId`, `YamsApiSpec`, `Leistung`, `Preis`, `Adresse`, `use_cases/abrechnung.rs`, `Ländercode`, `Klient`, `StructuredError`, `KlientErstellung`, `main`, `requests/abrechnung.rs`, `common.rs`, `ExecutionContext`, `Rechnung`, `KlientErstellen`, `commands.rs`, `Behandlung`, `Produkt`?**
  _High betweenness centrality (0.165) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `StructuredError`, `.run`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _184 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `E` be split into smaller, more focused modules?**
  _Cohesion score 0.05721003134796238 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.047619047619047616 - nodes in this community are weakly interconnected._