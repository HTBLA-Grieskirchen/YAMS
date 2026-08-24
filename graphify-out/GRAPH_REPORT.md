# Graph Report - yams  (2026-08-24)

## Corpus Check
- 242 files · ~315,157 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2182 nodes · 5046 edges · 130 communities (105 shown, 25 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 145 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `1d344083`
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
- EventStore
- RepositoryError
- UpMigration
- compilerOptions
- participation/index.tsx
- produkt_from_row
- Behandlung
- App
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- behandlung_from_row
- live
- UnitOfWorkImpl
- LeistungOffen
- stores/index.tsx
- KlientRepository
- HaustierErstellen
- .run
- relations/index.tsx
- EmailAdresse
- api/client.ts
- domain/rechnung.rs
- KlientId
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
- Versioned
- .contextualize_with
- KlientErstellen
- AddressTable.tsx
- leistung_from_row
- Preis
- base_app_builder
- TypicalJsonResponse
- HaustierErstellung
- Produkt
- Ratio
- e2e/main.rs
- StructuredError
- HaustierRepository
- DatabaseConnection
- _app.tsx
- base_app_builder
- libs/database/index.ts
- LeftMenuLayout.tsx
- openapi_service
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- src/api/schema.d.ts
- yams-core
- leistung-form.tsx
- Abrechnung — Domain-Spezifikation
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
- common.rs
- UseCase
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
- DialogStore
- .behandlung_erstellen
- AddressStore
- Klient
- .produkt_erstellen
- page.tsx
- ClientStore
- HaustierId
- api/index.ts
- HttpYamsApi
- ParticipationStore
- Store
- notification.ts
- KlientErstellung
- Menge
- preis.rs
- .from
- event/index.tsx
- layout.tsx
- BehandlungRepository
- RelationStore
- ExecutionContext
- tagesabschluss_returns_rechnungen_as_json
- haustier_erstellen_is_listed_and_fetchable

## God Nodes (most connected - your core abstractions)
1. `KlientId` - 58 edges
2. `useStore()` - 52 edges
3. `Versioned` - 49 edges
4. `query()` - 42 edges
5. `Preis` - 41 edges
6. `Ratio` - 35 edges
7. `FakeDatastore` - 33 edges
8. `E` - 32 edges
9. `YamsAppApi` - 30 edges
10. `makeRecordForTable()` - 29 edges

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

## Communities (130 total, 25 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (41): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+33 more)

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
Cohesion: 0.12
Nodes (31): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+23 more)

### Community 8 - "String"
Cohesion: 0.15
Nodes (34): YamsAppApi, From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen() (+26 more)

### Community 10 - "RepositoryError"
Cohesion: 0.26
Nodes (10): RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, AsRef, Path, ResultReport (+2 more)

### Community 11 - "UpMigration"
Cohesion: 0.15
Nodes (12): Send, Sync, UpMigration, Migration, Error, Option, Transaction, Migration (+4 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "participation/index.tsx"
Cohesion: 0.29
Nodes (11): ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation(), relateClientParticipateEvent() (+3 more)

### Community 14 - "produkt_from_row"
Cohesion: 0.27
Nodes (9): produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row, Transaction (+1 more)

### Community 15 - "Behandlung"
Cohesion: 0.14
Nodes (10): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+2 more)

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

### Community 20 - "parse_position_from_row"
Cohesion: 0.19
Nodes (16): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "behandlung_from_row"
Cohesion: 0.27
Nodes (9): behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 23 - "live"
Cohesion: 0.36
Nodes (3): live(), ano(), no()

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.19
Nodes (13): LeistungOffen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler (+5 more)

### Community 26 - "stores/index.tsx"
Cohesion: 0.11
Nodes (15): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAddress(), deleteAnimal(), patchAnimal(), query(), deleteRace() (+7 more)

### Community 28 - "HaustierErstellen"
Cohesion: 0.21
Nodes (11): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, ResultReport (+3 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "relations/index.tsx"
Cohesion: 0.09
Nodes (25): clientSearched(), deleteClientRelation(), relateClients(), updateClientRelation(), createSeminar(), ensureSeminar(), Address, Client (+17 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "api/client.ts"
Cohesion: 0.17
Nodes (8): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), Addresses

### Community 33 - "domain/rechnung.rs"
Cohesion: 0.13
Nodes (24): aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position(), Rechnung (+16 more)

### Community 34 - "KlientId"
Cohesion: 0.11
Nodes (18): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, KlientId, neu(), NeuerKlient (+10 more)

### Community 35 - "LeistungId"
Cohesion: 0.12
Nodes (22): Abgerechnet, Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungId, LeistungIn, LeistungIn<S> (+14 more)

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
Cohesion: 0.13
Nodes (15): AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons (+7 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.16
Nodes (17): format_naive_date(), parse_naive_date(), parse_rechnung_id(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung (+9 more)

### Community 51 - "Preis"
Cohesion: 0.11
Nodes (8): Add, Preis, Output, Self, position_from_leistung(), RechnungIn<S>, Rechnungsposition, Mul

### Community 52 - "base_app_builder"
Cohesion: 0.20
Nodes (16): behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), klient_body(), klient_erstellen_rejects_empty_name(), klient_erstellen_rejects_invalid_email(), klient_erstellen_rejects_invalid_ländercode(), klient_erstellen_returns_camelcase_utf8_json(), Value (+8 more)

### Community 53 - "TypicalJsonResponse"
Cohesion: 0.20
Nodes (9): Haustier, HaustierErstellung, Path, Rechnung, StatusCode, TagesabschlussErstellung, Uuid, Vec (+1 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.14
Nodes (10): produkt_betrag_multiplies_menge(), NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport (+2 more)

### Community 56 - "Ratio"
Cohesion: 0.20
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 57 - "e2e/main.rs"
Cohesion: 0.29
Nodes (9): json_decimal(), json_response(), Decimal, StatusCode, Value, YamsApiTestClient, Route, TestClient (+1 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "HaustierRepository"
Cohesion: 0.16
Nodes (5): HaustierRepository, ProduktRepository, Send, Sync, Debug

### Community 61 - "_app.tsx"
Cohesion: 0.10
Nodes (18): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+10 more)

### Community 62 - "base_app_builder"
Cohesion: 0.11
Nodes (24): base_app_builder(), AppBuilder, SetUowProvider, AbrechnungSetup, menge(), mwst_19(), Arc, Klient (+16 more)

### Community 63 - "libs/database/index.ts"
Cohesion: 0.10
Nodes (17): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+9 more)

### Community 64 - "LeftMenuLayout.tsx"
Cohesion: 0.13
Nodes (7): MenuEntryData, Data, Home, setupStore, Layout, nextConfig, !.next

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

### Community 72 - "Adresse"
Cohesion: 0.26
Nodes (8): Adresse, domain::Adresse, Ländercode, Error, Example, From, Self, TryFrom

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
Cohesion: 0.22
Nodes (16): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis(), parse_ratio(), parse_uuid() (+8 more)

### Community 92 - "UseCase"
Cohesion: 0.24
Nodes (8): UseCase, BehandlungErstellen, ProduktErstellen, Behandlung, Error, Produkt, Vec, TagesabschlussDurchführen

### Community 100 - "useStore"
Cohesion: 0.10
Nodes (28): EventsUsages, AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, EventDetailItem, EventOverviewItem (+20 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.19
Nodes (18): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+10 more)

### Community 105 - "DialogStore"
Cohesion: 0.27
Nodes (3): DialogInfo, DialogInfoType, DialogStore

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.36
Nodes (6): Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 108 - "Klient"
Cohesion: 0.22
Nodes (11): Klient, KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate (+3 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.32
Nodes (6): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 113 - "HaustierId"
Cohesion: 0.15
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+5 more)

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 118 - "notification.ts"
Cohesion: 0.14
Nodes (14): ActionButton, buttonColor(), Notification, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent, NotificationInfo (+6 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellung, Adresse, Error, NaiveDate, Self, TryFrom

### Community 120 - "Menge"
Cohesion: 0.24
Nodes (4): Menge, MengeFehler, Decimal, Self

### Community 121 - "preis.rs"
Cohesion: 0.36
Nodes (5): preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 122 - ".from"
Cohesion: 0.43
Nodes (6): C, From, Report, T, TypicalJsonResponse<T>, PlainText

### Community 123 - "event/index.tsx"
Cohesion: 0.31
Nodes (4): askSubmitDeleteEvent(), submitDeleteEvent(), deleteEvent(), EventResponse

### Community 124 - "layout.tsx"
Cohesion: 0.31
Nodes (5): geistMono, geistSans, metadata, Providers(), createQueryClient()

### Community 128 - "tagesabschluss_returns_rechnungen_as_json"
Cohesion: 0.67
Nodes (3): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json()

### Community 129 - "haustier_erstellen_is_listed_and_fetchable"
Cohesion: 0.67
Nodes (3): haustier_erstellen_is_listed_and_fetchable(), klient_body(), Value

## Knowledge Gaps
- **222 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+217 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `YAMSFrontendConfig`, `Behandlung`, `domain/kontakt.rs`, `parse_position_from_row`, `LeistungOffen`, `HaustierErstellen`, `EmailAdresse`, `KlientId`, `LeistungId`, `ResultReport`, `YamsApiSpec`, `domain/adresse.rs`, `schema/leistung.rs`, `Versioned`, `KlientErstellen`, `leistung_from_row`, `Preis`, `HaustierErstellung`, `Produkt`, `StructuredError`, `openapi_service`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `UseCase`, `Rechnung`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `HaustierId`, `KlientErstellung`?**
  _High betweenness centrality (0.199) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.044) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `openapi_service`, `ResultReport`, `Clock`, `String`, `YamsApiSpec`, `UnitOfWorkImpl`, `base_app_builder`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _222 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._