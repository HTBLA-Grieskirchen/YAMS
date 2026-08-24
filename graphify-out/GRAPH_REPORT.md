# Graph Report - yams  (2026-08-24)

## Corpus Check
- 239 files · ~314,738 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2161 nodes · 4991 edges · 119 communities (102 shown, 17 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 134 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5c490583`
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
- makeRecordForTable
- SQLiteInstance
- UpMigration
- compilerOptions
- participation/index.tsx
- ProduktId
- BehandlungId
- App
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- behandlung_from_row
- libs/database/index.ts
- UnitOfWorkImpl
- ExecutionContext
- AnimalAddItem.tsx
- KlientRepository
- UseCase
- .run
- relations/index.tsx
- EmailAdresse
- addresses/index.tsx
- domain/rechnung.rs
- NeuerKlient
- LeistungId
- SQLiteConnection
- UnitOfWork
- YamsAppApi
- Clock
- .get_current_version
- TypicalJsonResponse
- domain/adresse.rs
- schema/leistung.rs
- SQLiteUnitOfWork
- Versioned
- .contextualize_with
- KlientErstellen
- AddressTable.tsx
- leistung_from_row
- Preis
- support.rs
- YamsApiSpec
- HaustierErstellung
- NeuesProdukt
- Ratio
- Api
- StructuredError
- HaustierRepository
- DatabaseConnection
- ClientItem.tsx
- cases/abrechnung.rs
- Haustier
- migration_error_to_persistence_error
- openapi_service
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- src/api/schema.d.ts
- yams-core
- leistung-form.tsx
- Abrechnung — Domain-Spezifikation
- NotificationStore
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
- hooks/index.ts
- RechnungRepository
- DialogStore
- Behandlung
- Klient
- Produkt
- page.tsx
- KlientId
- api/index.ts
- HttpYamsApi
- notification.ts
- KlientErstellung
- Menge
- .from
- BehandlungRepository

## God Nodes (most connected - your core abstractions)
1. `KlientId` - 53 edges
2. `useStore()` - 52 edges
3. `Versioned` - 49 edges
4. `query()` - 42 edges
5. `Preis` - 41 edges
6. `Ratio` - 33 edges
7. `FakeDatastore` - 33 edges
8. `E` - 32 edges
9. `YamsAppApi` - 30 edges
10. `makeRecordForTable()` - 29 edges

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

## Communities (119 total, 17 thin omitted)

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
Cohesion: 0.13
Nodes (33): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+25 more)

### Community 8 - "String"
Cohesion: 0.14
Nodes (33): From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen(), leistung_aus_behandlung_buchen() (+25 more)

### Community 9 - "makeRecordForTable"
Cohesion: 0.11
Nodes (9): createSeminar(), ensureSeminar(), isRecord(), makeRecord(), makeRecordForTable(), RecordError, EventResponse, Seminar (+1 more)

### Community 10 - "SQLiteInstance"
Cohesion: 0.38
Nodes (5): AsRef, Path, ResultReport, Self, SQLiteInstance

### Community 11 - "UpMigration"
Cohesion: 0.15
Nodes (12): Send, Sync, UpMigration, Migration, Error, Option, Transaction, Migration (+4 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "participation/index.tsx"
Cohesion: 0.11
Nodes (16): EventParticipants, participationSearchedClient(), SmallSearchField, ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation() (+8 more)

### Community 14 - "ProduktId"
Cohesion: 0.15
Nodes (13): produkt_betrag_multiplies_menge(), Produkt, ProduktId, Uuid, produkt_from_row(), Arc, Mutex, Option (+5 more)

### Community 15 - "BehandlungId"
Cohesion: 0.13
Nodes (13): Behandlung, behandlung_accepts_full_mwst(), behandlung_accepts_zero_mwst(), behandlung_rejects_empty_name(), BehandlungFehler, BehandlungId, NeueBehandlung, preis() (+5 more)

### Community 16 - "App"
Cohesion: 0.16
Nodes (13): Self, App, Arc, Box, F, O, ResultReport, T (+5 more)

### Community 17 - "apply_up_migrations"
Cohesion: 0.21
Nodes (11): apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationUp, MigrationError, MigrationTarget, Box, Item (+3 more)

### Community 18 - "E"
Cohesion: 0.35
Nodes (9): AppliableMigration, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration, T (+1 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (12): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+4 more)

### Community 20 - "parse_position_from_row"
Cohesion: 0.20
Nodes (16): parse_klient_id(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "behandlung_from_row"
Cohesion: 0.27
Nodes (9): behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.05
Nodes (22): CompatibilityResult, DatabaseError, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented (+14 more)

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "ExecutionContext"
Cohesion: 0.11
Nodes (23): ExecutionContext, Arc, LeistungOffen, RechnungOffen, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler (+15 more)

### Community 26 - "AnimalAddItem.tsx"
Cohesion: 0.10
Nodes (13): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAnimal(), patchAnimal(), deleteRace(), patchRace(), Animal (+5 more)

### Community 28 - "UseCase"
Cohesion: 0.18
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "relations/index.tsx"
Cohesion: 0.15
Nodes (14): clientSearched(), deleteClientRelation(), relateClients(), updateClientRelation(), Client, Record, ClientRelation, ClientRelationResponse (+6 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "addresses/index.tsx"
Cohesion: 0.14
Nodes (12): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), AddressTableHeader (+4 more)

### Community 33 - "domain/rechnung.rs"
Cohesion: 0.13
Nodes (25): aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position(), Rechnung (+17 more)

### Community 34 - "NeuerKlient"
Cohesion: 0.10
Nodes (17): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, neu_ok(), NeuerKlient, Adresse (+9 more)

### Community 35 - "LeistungId"
Cohesion: 0.11
Nodes (19): Abgerechnet, Leistung, LeistungAbgerechnet, LeistungFehler, LeistungId, LeistungIn, LeistungIn<S>, LeistungQuelle (+11 more)

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

### Community 41 - "TypicalJsonResponse"
Cohesion: 0.13
Nodes (13): Behandlung, BehandlungErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung (+5 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.24
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

### Community 48 - "AddressTable.tsx"
Cohesion: 0.10
Nodes (24): AddressEditTableRowContent, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons, EditState, EventsUsages (+16 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.23
Nodes (12): leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex, NaiveDate, Option, RepositoryResult (+4 more)

### Community 51 - "Preis"
Cohesion: 0.09
Nodes (14): Add, Preis, preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal, Output (+6 more)

### Community 52 - "support.rs"
Cohesion: 0.22
Nodes (14): produkt_erstellen_rejects_mwst_greater_than_one(), produkt_erstellen_rejects_negative_preis(), tagesabschluss_returns_rechnungen_as_json(), haustier_erstellen_is_listed_and_fetchable(), klient_erstellen_rejects_empty_name(), klient_erstellen_rejects_invalid_email(), klient_erstellen_rejects_invalid_ländercode(), klient_erstellen_returns_camelcase_utf8_json() (+6 more)

### Community 53 - "YamsApiSpec"
Cohesion: 0.20
Nodes (9): Haustier, HaustierErstellung, Path, Rechnung, Self, TagesabschlussErstellung, Uuid, Vec (+1 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "NeuesProdukt"
Cohesion: 0.19
Nodes (8): NeuesProdukt, preis(), produkt_accepts_zero_mwst(), produkt_rejects_empty_name(), ProduktFehler, Into, ResultReport, Self

### Community 56 - "Ratio"
Cohesion: 0.24
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 57 - "Api"
Cohesion: 0.36
Nodes (7): Api, json_response(), StatusCode, Route, TestClient, TestResponse, Value

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "HaustierRepository"
Cohesion: 0.16
Nodes (5): HaustierRepository, ProduktRepository, Send, Sync, Debug

### Community 60 - "DatabaseConnection"
Cohesion: 0.16
Nodes (6): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection

### Community 61 - "ClientItem.tsx"
Cohesion: 0.43
Nodes (6): AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, LiveRefresher

### Community 62 - "cases/abrechnung.rs"
Cohesion: 0.15
Nodes (18): base_app_builder(), AppBuilder, SetUowProvider, AbrechnungSetup, menge(), mwst_19(), Arc, Klient (+10 more)

### Community 63 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 64 - "migration_error_to_persistence_error"
Cohesion: 0.83
Nodes (3): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error

### Community 65 - "openapi_service"
Cohesion: 0.17
Nodes (11): BackendServerError, Config, main(), Report, openapi_service(), Into, Item, Self (+3 more)

### Community 66 - "Integration Test Workflow"
Cohesion: 0.53
Nodes (6): mise, mise-action, Release Workflow, Integration Test Workflow, tauri-action, Unit Test Workflow

### Community 67 - "frontend-legacy/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "src/api/schema.d.ts"
Cohesion: 0.17
Nodes (11): components, $defs, leistungQuelle_LeistungQuelleBehandlungTypValues, leistungQuelle_LeistungQuelleManuellTypValues, leistungQuelle_LeistungQuelleProduktTypValues, leistungStatusValues, operations, paths (+3 more)

### Community 69 - "yams-core"
Cohesion: 0.48
Nodes (7): molting, yams, yams-api, yams-core, yams-fakes, yams-persistence, yams-server

### Community 70 - "leistung-form.tsx"
Cohesion: 0.12
Nodes (35): Alert(), AlertProps, AlertVariant, variantClasses, Button(), ButtonProps, ButtonSize, ButtonVariant (+27 more)

### Community 71 - "Abrechnung — Domain-Spezifikation"
Cohesion: 0.22
Nodes (8): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte

### Community 72 - "NotificationStore"
Cohesion: 0.27
Nodes (4): NotificationInfo, NotificationInfoType, NotificationEntry, NotificationStore

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
Cohesion: 0.12
Nodes (28): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+20 more)

### Community 80 - "api/types.ts"
Cohesion: 0.11
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 89 - "common.rs"
Cohesion: 0.17
Nodes (22): RepositoryError, Option, format_naive_date(), menge_to_str(), parse_decimal(), parse_haustier_id(), parse_menge(), parse_naive_date() (+14 more)

### Community 100 - "stores/index.tsx"
Cohesion: 0.07
Nodes (35): queryClient, MenuEntryData, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData (+27 more)

### Community 101 - "Rechnung"
Cohesion: 0.13
Nodes (22): Adresse, domain::Adresse, Ländercode, Error, Example, From, Self, TryFrom (+14 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.19
Nodes (18): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+10 more)

### Community 105 - "DialogStore"
Cohesion: 0.13
Nodes (4): DialogInfo, DialogInfoType, DialogStore, SettingsStore

### Community 106 - "Behandlung"
Cohesion: 0.70
Nodes (4): Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - "Produkt"
Cohesion: 0.60
Nodes (4): Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 113 - "KlientId"
Cohesion: 0.15
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, NeuesHaustier, Into (+5 more)

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 118 - "notification.ts"
Cohesion: 0.23
Nodes (10): ActionButton, buttonColor(), Notification, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent, NotificationType (+2 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellung, Adresse, Error, NaiveDate, Self, TryFrom

### Community 120 - "Menge"
Cohesion: 0.22
Nodes (4): Menge, MengeFehler, Decimal, Self

### Community 122 - ".from"
Cohesion: 0.43
Nodes (6): C, From, Report, T, TypicalJsonResponse<T>, PlainText

## Knowledge Gaps
- **222 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+217 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **17 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `ProduktId`, `BehandlungId`, `domain/kontakt.rs`, `parse_position_from_row`, `ExecutionContext`, `UseCase`, `EmailAdresse`, `NeuerKlient`, `LeistungId`, `TypicalJsonResponse`, `domain/adresse.rs`, `schema/leistung.rs`, `Versioned`, `KlientErstellen`, `Preis`, `HaustierErstellung`, `NeuesProdukt`, `StructuredError`, `Haustier`, `openapi_service`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `Behandlung`, `Klient`, `Produkt`, `KlientId`, `KlientErstellung`, `Menge`?**
  _High betweenness centrality (0.201) - this node is a cross-community bridge._
- **Why does `KlientId` connect `KlientId` to `domain/rechnung.rs`, `NeuerKlient`, `LeistungId`, `YamsAppApi`, `requests/abrechnung.rs`, `Versioned`, `Preis`, `parse_position_from_row`, `HaustierErstellung`, `ExecutionContext`, `UseCase`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `openapi_service`, `YamsAppApi`, `Clock`, `YamsApiSpec`, `UnitOfWorkImpl`, `cases/abrechnung.rs`?**
  _High betweenness centrality (0.030) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _222 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._