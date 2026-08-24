# Graph Report - yams  (2026-08-24)

## Corpus Check
- 235 files · ~311,775 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1988 nodes · 4578 edges · 125 communities (100 shown, 25 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 118 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3592185e`
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
- EventStore
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
- live
- UnitOfWorkImpl
- use_cases/abrechnung.rs
- stores/index.tsx
- KlientRepository
- UseCase
- .run
- relations/index.tsx
- EmailAdresse
- api/client.ts
- .aus_leistungen
- KlientId
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
- FakeDatastore
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
- Store
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- src/api/schema.d.ts
- yams-core
- leistung-form.tsx
- Abrechnung — Domain-Spezifikation
- makeRecordForTable
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
- Versioned
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- useStore
- Rechnung
- LeistungRepository
- hooks/index.ts
- RechnungRepository
- Behandlung
- libs/database/index.ts
- Klient
- Produkt
- page.tsx
- SQLiteHaustierRepository
- LeistungQuelle
- api/index.ts
- HttpYamsApi
- .new
- SQLiteKlientRepository
- notification.ts
- ClientStore
- RelationStore
- NotificationStore
- Rechnungsposition
- EventParticipationResponse
- ProduktErstellen

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

## Communities (125 total, 25 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (41): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+33 more)

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
Cohesion: 0.10
Nodes (35): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+27 more)

### Community 8 - "String"
Cohesion: 0.14
Nodes (33): From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen(), leistung_aus_behandlung_buchen() (+25 more)

### Community 9 - "EventStore"
Cohesion: 0.12
Nodes (3): EventResponse, SeminarResponse, EventStore

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
Cohesion: 0.10
Nodes (17): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+9 more)

### Community 14 - "ProduktId"
Cohesion: 0.20
Nodes (14): NeuesProdukt, Produkt, ProduktId, Decimal, Uuid, produkt_from_row(), Arc, Mutex (+6 more)

### Community 15 - "participation/index.tsx"
Cohesion: 0.25
Nodes (12): clientSearched(), ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation() (+4 more)

### Community 16 - "App"
Cohesion: 0.16
Nodes (13): Self, App, Arc, Box, F, O, ResultReport, T (+5 more)

### Community 17 - "apply_up_migrations"
Cohesion: 0.21
Nodes (11): apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationUp, MigrationError, MigrationTarget, Box, Item (+3 more)

### Community 18 - "E"
Cohesion: 0.35
Nodes (9): AppliableMigration, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration, T (+1 more)

### Community 19 - "EmailAdresse"
Cohesion: 0.25
Nodes (9): EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, MobilnummerValidierungsfehler, AsRef, Error, S, Self (+1 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.18
Nodes (17): parse_klient_id(), preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex (+9 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "BehandlungId"
Cohesion: 0.20
Nodes (14): Behandlung, BehandlungId, NeueBehandlung, Decimal, Uuid, behandlung_from_row(), Arc, Behandlung (+6 more)

### Community 23 - "live"
Cohesion: 0.23
Nodes (4): live(), ParticipationStore, ano(), no()

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "use_cases/abrechnung.rs"
Cohesion: 0.16
Nodes (16): BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler, ProduktErstellenFehler (+8 more)

### Community 26 - "stores/index.tsx"
Cohesion: 0.14
Nodes (18): AnimalAddItem, AnimalComboBox, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow (+10 more)

### Community 28 - "UseCase"
Cohesion: 0.18
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "relations/index.tsx"
Cohesion: 0.20
Nodes (12): EventParticipants, participationSearchedClient(), SmallSearchField, deleteClientRelation(), relateClients(), updateClientRelation(), ClientDetail, AddRelationDialog (+4 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "api/client.ts"
Cohesion: 0.17
Nodes (8): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), Addresses

### Community 33 - ".aus_leistungen"
Cohesion: 0.17
Nodes (15): Bezahlt, Offen, Rechnung, RechnungBezahlt, RechnungFehler, RechnungIn, RechnungOffen, From (+7 more)

### Community 34 - "KlientId"
Cohesion: 0.21
Nodes (14): Haustier, HaustierId, NeuesHaustier, NaiveDate, Uuid, Klient, KlientId, NeuerKlient (+6 more)

### Community 35 - "LeistungId"
Cohesion: 0.16
Nodes (13): Abgerechnet, Leistung, LeistungAbgerechnet, LeistungId, Offen, From, Self, Uuid (+5 more)

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

### Community 45 - "FakeDatastore"
Cohesion: 0.16
Nodes (15): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository, Arc, Clone, Mutex (+7 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.21
Nodes (10): ErrorReportExt, Result<T, E>, C, Error, Report, Send, Sync, T (+2 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "ExecutionContext"
Cohesion: 0.29
Nodes (5): ExecutionContext, Arc, LeistungOffen, LeistungAusBehandlungBuchen, Error

### Community 50 - "leistung_from_row"
Cohesion: 0.23
Nodes (12): leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex, NaiveDate, Option, RepositoryResult (+4 more)

### Community 51 - "Preis"
Cohesion: 0.16
Nodes (7): Add, Preis, PreisFehler, Decimal, Output, Self, RechnungIn<S>

### Community 52 - "Adresse"
Cohesion: 0.26
Nodes (8): Adresse, domain::Adresse, Ländercode, Error, Example, From, Self, TryFrom

### Community 53 - "LeftMenuLayout.tsx"
Cohesion: 0.21
Nodes (4): MenuEntryData, Home, setupStore, Layout

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "DialogStore"
Cohesion: 0.27
Nodes (3): DialogInfo, DialogInfoType, DialogStore

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "HaustierRepository"
Cohesion: 0.16
Nodes (5): HaustierRepository, ProduktRepository, Send, Sync, Debug

### Community 62 - "base_app_builder"
Cohesion: 0.16
Nodes (13): base_app_builder(), AppBuilder, SetUowProvider, AbrechnungSetup, Arc, Klient, NaiveDate, setup_abrechnung_fixture() (+5 more)

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

### Community 70 - "leistung-form.tsx"
Cohesion: 0.12
Nodes (35): Alert(), AlertProps, AlertVariant, variantClasses, Button(), ButtonProps, ButtonSize, ButtonVariant (+27 more)

### Community 71 - "Abrechnung — Domain-Spezifikation"
Cohesion: 0.22
Nodes (8): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte

### Community 72 - "makeRecordForTable"
Cohesion: 0.15
Nodes (15): createSeminar(), ensureSeminar(), Address, Client, DatabaseObject, DatabaseResponse, isRecord(), makeRecord() (+7 more)

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
Cohesion: 0.11
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 89 - "common.rs"
Cohesion: 0.20
Nodes (16): decimal_to_str(), format_naive_date(), parse_decimal(), parse_haustier_id(), parse_naive_date(), parse_preis(), parse_uuid(), quelle_from_row() (+8 more)

### Community 92 - "Versioned"
Cohesion: 0.16
Nodes (11): Versioned, FakeHaustiereRepository, FakeKlientenRepository, Behandlung, Haustier, Klient, Leistung, Produkt (+3 more)

### Community 100 - "useStore"
Cohesion: 0.09
Nodes (33): AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons (+25 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.19
Nodes (18): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+10 more)

### Community 106 - "Behandlung"
Cohesion: 0.70
Nodes (4): Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "libs/database/index.ts"
Cohesion: 0.10
Nodes (17): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+9 more)

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - "Produkt"
Cohesion: 0.60
Nodes (4): Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 112 - "SQLiteHaustierRepository"
Cohesion: 0.25
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, Mutex, Option, RepositoryResult, Row (+4 more)

### Community 113 - "LeistungQuelle"
Cohesion: 0.14
Nodes (7): LeistungIn, LeistungIn<S>, LeistungQuelle, Decimal, NaiveDate, Option, S

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 116 - ".new"
Cohesion: 0.27
Nodes (7): FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec

### Community 117 - "SQLiteKlientRepository"
Cohesion: 0.26
Nodes (9): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 118 - "notification.ts"
Cohesion: 0.21
Nodes (11): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+3 more)

### Community 121 - "NotificationStore"
Cohesion: 0.27
Nodes (4): NotificationInfo, NotificationInfoType, NotificationEntry, NotificationStore

### Community 122 - "Rechnungsposition"
Cohesion: 0.43
Nodes (3): position_from_leistung(), Rechnungsposition, Decimal

## Knowledge Gaps
- **223 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+218 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `ProduktId`, `EmailAdresse`, `SQLiteRechnungRepository`, `BehandlungId`, `use_cases/abrechnung.rs`, `UseCase`, `EmailAdresse`, `KlientId`, `LeistungId`, `YamsApiSpec`, `Ländercode`, `schema/leistung.rs`, `FakeDatastore`, `KlientErstellen`, `Adresse`, `HaustierErstellung`, `StructuredError`, `Haustier`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `Behandlung`, `Klient`, `Produkt`, `LeistungQuelle`, `Rechnungsposition`, `ProduktErstellen`?**
  _High betweenness centrality (0.148) - this node is a cross-community bridge._
- **Why does `!.next` connect `layout.tsx` to `useStore`, `LeftMenuLayout.tsx`?**
  _High betweenness centrality (0.066) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `.add_dyn`, `String`, `.contextualize_with`, `apply_up_migrations`, `StructuredError`, `.run`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _223 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._