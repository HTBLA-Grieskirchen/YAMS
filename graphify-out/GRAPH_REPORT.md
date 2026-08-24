# Graph Report - yams  (2026-08-24)

## Corpus Check
- 233 files · ~312,473 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2011 nodes · 4634 edges · 115 communities (93 shown, 22 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 118 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `482378ab`
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
- domain/kontakt.rs
- SQLiteRechnungRepository
- Versioned<T>
- BehandlungId
- stores/index.tsx
- UnitOfWorkImpl
- ExecutionContext
- makeRecordForTable
- KlientRepository
- HaustierErstellen
- .run
- query
- EmailAdresse
- AddressTable.tsx
- Rechnungsposition
- NeuerKlient
- LeistungId
- SQLiteConnection
- UnitOfWork
- ResultReport
- Clock
- .get_current_version
- YamsApiSpec
- domain/adresse.rs
- schema/leistung.rs
- SQLiteUnitOfWork
- FakeDatastore
- .contextualize_with
- KlientErstellen
- useStore
- leistung_from_row
- Preis
- e2e.rs
- LeftMenuLayout.tsx
- HaustierErstellung
- dialog.ts
- AnimalResponse
- ProduktRepository
- StructuredError
- BehandlungRepository
- DatabaseConnection
- FakeKlientenRepository
- base_app_builder
- FakeProdukteRepository
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- src/api/schema.d.ts
- yams-core
- leistung-form.tsx
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
- RepositoryResult
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- paths.ts
- Rechnung
- LeistungRepository
- hooks/index.ts
- RechnungRepository
- .behandlung_erstellen
- libs/database/index.ts
- Klient
- .produkt_erstellen
- page.tsx
- Versioned
- KlientId
- api/index.ts
- HttpYamsApi
- FakeUnitOfWork
- SQLiteKlientRepository
- notification.ts

## God Nodes (most connected - your core abstractions)
1. `useStore()` - 52 edges
2. `Versioned` - 49 edges
3. `query()` - 42 edges
4. `KlientId` - 39 edges
5. `FakeDatastore` - 33 edges
6. `E` - 32 edges
7. `Preis` - 32 edges
8. `YamsAppApi` - 31 edges
9. `makeRecordForTable()` - 29 edges
10. `Record` - 28 edges

## Surprising Connections (you probably didn't know these)
- `Config` --references--> `String`  [EXTRACTED]
  backend/server/src/main.rs → crates/yams-api/src/errors/internal_error.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  crates/yams-api/src/bin/export_spec.rs → crates/yams-api/src/spec.rs
- `YAMSBackendConfig` --references--> `String`  [EXTRACTED]
  frontend/src-tauri/src/config.rs → crates/yams-api/src/errors/internal_error.rs
- `YAMSFileConfig` --references--> `String`  [EXTRACTED]
  frontend/src-tauri/src/config.rs → crates/yams-api/src/errors/internal_error.rs

## Import Cycles
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **CI Release Pipeline** — github_workflows_publish_release_workflow, github_workflows_smoke_tests_workflow, github_workflows_unit_tests_workflow, github_workflows_tauri_action, github_workflows_mise_action [EXTRACTED 1.00]
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **Frontend Backend Communication Bridge** — specs_backend_hexagonal_backend_client, specs_backend_hexagonal_http_adapter, specs_backend_hexagonal_tauri_adapter, specs_backend_hexagonal_tanstack_query, specs_frontend_migration_yamsconfig [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (115 total, 22 thin omitted)

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
Cohesion: 0.12
Nodes (31): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+23 more)

### Community 8 - "String"
Cohesion: 0.15
Nodes (34): YamsAppApi, From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen() (+26 more)

### Community 9 - "model/event.ts"
Cohesion: 0.06
Nodes (11): createSeminar(), ensureSeminar(), AddressResponse, ClientResponse, DatabaseResponse, Event, EventResponse, ClientRelationResponse (+3 more)

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
Cohesion: 0.19
Nodes (15): produkt_betrag_multiplies_menge(), NeuesProdukt, Produkt, ProduktId, Decimal, Uuid, produkt_from_row(), Arc (+7 more)

### Community 15 - "participation/index.tsx"
Cohesion: 0.12
Nodes (15): askSubmitDeleteEvent(), submitDeleteEvent(), askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitUpdateEventParticipation(), deleteEvent(), deleteEventParticipation() (+7 more)

### Community 16 - "App"
Cohesion: 0.17
Nodes (14): Self, App, Arc, Box, F, O, ResultReport, T (+6 more)

### Community 17 - "apply_up_migrations"
Cohesion: 0.21
Nodes (11): apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationUp, MigrationError, MigrationTarget, Box, Item (+3 more)

### Community 18 - "E"
Cohesion: 0.35
Nodes (9): AppliableMigration, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration, T (+1 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (12): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+4 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.19
Nodes (16): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "BehandlungId"
Cohesion: 0.19
Nodes (15): Behandlung, BehandlungId, NeueBehandlung, Decimal, Uuid, behandlung_betrag_uses_snapshot_preis(), behandlung_from_row(), Arc (+7 more)

### Community 23 - "stores/index.tsx"
Cohesion: 0.08
Nodes (10): live(), AddressStore, AnimalStore, RelationStore, ClientStore, Store, StoreContext, SettingsStore (+2 more)

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "ExecutionContext"
Cohesion: 0.12
Nodes (24): ExecutionContext, Arc, LeistungOffen, UseCase, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler (+16 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.14
Nodes (17): AnimalComboBox, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableRow, deleteAnimal(), LiveRefresher (+9 more)

### Community 28 - "HaustierErstellen"
Cohesion: 0.21
Nodes (11): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, ResultReport (+3 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "query"
Cohesion: 0.20
Nodes (16): AnimalAddItem, clientSearched(), patchAnimal(), deleteClientRelation(), relateClients(), updateClientRelation(), query(), useQuery() (+8 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "AddressTable.tsx"
Cohesion: 0.09
Nodes (22): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), AddressEditTableRowContent (+14 more)

### Community 33 - "Rechnungsposition"
Cohesion: 0.12
Nodes (24): Bezahlt, Offen, position(), position_from_leistung(), Rechnung, rechnung_offen_rejects_empty_positionen(), RechnungBezahlt, RechnungFehler (+16 more)

### Community 34 - "NeuerKlient"
Cohesion: 0.43
Nodes (7): Klient, NeuerKlient, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Uuid

### Community 35 - "LeistungId"
Cohesion: 0.11
Nodes (16): Abgerechnet, Leistung, LeistungAbgerechnet, LeistungId, LeistungIn, LeistungIn<S>, LeistungQuelle, NeueLeistung (+8 more)

### Community 36 - "SQLiteConnection"
Cohesion: 0.23
Nodes (10): Connection, InstanceType, Arc, Deref, Mutex, Target, SQLiteConnection, Database (+2 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "ResultReport"
Cohesion: 0.11
Nodes (16): Arc, Haustier, HaustierErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Rechnung (+8 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "YamsApiSpec"
Cohesion: 0.05
Nodes (45): BackendServerError, Config, main(), Report, KlientErstellen, KlientErstellung, Adresse, Error (+37 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.24
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (14): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungStatus, Decimal, NaiveDate (+6 more)

### Community 44 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "FakeDatastore"
Cohesion: 0.13
Nodes (15): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeRechnungenRepository, Arc, Behandlung, Clone, Leistung (+7 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.21
Nodes (10): ErrorReportExt, Result<T, E>, C, Error, Report, Send, Sync, T (+2 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "useStore"
Cohesion: 0.21
Nodes (15): EventsUsages, EventDetailItem, EventOverviewItem, EventParticipants, participationSearchedClient(), ClientParticipation, submitDeleteEventParticipation(), useSubmissionState() (+7 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.21
Nodes (13): format_naive_date(), leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex, NaiveDate, Option (+5 more)

### Community 51 - "Preis"
Cohesion: 0.12
Nodes (9): Add, Preis, preis_add_sums_values(), preis_multiply_scales_value(), PreisFehler, Decimal, Output, Self (+1 more)

### Community 52 - "e2e.rs"
Cohesion: 0.16
Nodes (19): Adresse, domain::Adresse, Ländercode, Error, Example, From, Self, TryFrom (+11 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "BehandlungRepository"
Cohesion: 0.16
Nodes (5): BehandlungRepository, HaustierRepository, Send, Sync, Debug

### Community 62 - "base_app_builder"
Cohesion: 0.16
Nodes (13): base_app_builder(), AppBuilder, SetUowProvider, AbrechnungSetup, Arc, Klient, NaiveDate, setup_abrechnung_fixture() (+5 more)

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
Cohesion: 0.19
Nodes (19): decimal_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_naive_date(), parse_preis(), parse_rechnung_id(), parse_uuid() (+11 more)

### Community 92 - "RepositoryResult"
Cohesion: 0.40
Nodes (4): FakeHaustiereRepository, Haustier, RepositoryResult, Vec

### Community 100 - "paths.ts"
Cohesion: 0.13
Nodes (12): ClientTableHeader, SmallSearchField, EditClient(), ClientOverview, DisplayCategory, eventSearched(), EventsOverview, Home (+4 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.19
Nodes (18): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+10 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.36
Nodes (6): Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "libs/database/index.ts"
Cohesion: 0.11
Nodes (16): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+8 more)

### Community 108 - "Klient"
Cohesion: 0.22
Nodes (11): Klient, KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate (+3 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.32
Nodes (6): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 112 - "Versioned"
Cohesion: 0.27
Nodes (11): Versioned, query_all_haustiere(), Arc, Haustier, Mutex, Option, RepositoryResult, Transaction (+3 more)

### Community 113 - "KlientId"
Cohesion: 0.42
Nodes (6): Haustier, HaustierId, NeuesHaustier, NaiveDate, Uuid, KlientId

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 116 - "FakeUnitOfWork"
Cohesion: 0.28
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 117 - "SQLiteKlientRepository"
Cohesion: 0.26
Nodes (9): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 118 - "notification.ts"
Cohesion: 0.13
Nodes (15): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+7 more)

## Knowledge Gaps
- **223 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+218 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `ProduktId`, `domain/kontakt.rs`, `SQLiteRechnungRepository`, `BehandlungId`, `ExecutionContext`, `HaustierErstellen`, `EmailAdresse`, `Rechnungsposition`, `NeuerKlient`, `LeistungId`, `ResultReport`, `YamsApiSpec`, `domain/adresse.rs`, `schema/leistung.rs`, `KlientErstellen`, `leistung_from_row`, `e2e.rs`, `HaustierErstellung`, `StructuredError`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `KlientId`, `FakeUnitOfWork`?**
  _High betweenness centrality (0.168) - this node is a cross-community bridge._
- **Why does `!.next` connect `layout.tsx` to `paths.ts`?**
  _High betweenness centrality (0.052) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `.add_dyn`, `String`, `.contextualize_with`, `apply_up_migrations`, `StructuredError`, `.run`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _223 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._