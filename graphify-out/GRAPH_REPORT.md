# Graph Report - yams  (2026-08-25)

## Corpus Check
- 254 files · ~324,965 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2597 nodes · 6327 edges · 137 communities (116 shown, 21 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 201 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f5bc2565`
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
- requests/seminar.rs
- UpMigration
- compilerOptions
- stores/index.tsx
- SQLiteProduktRepository
- Behandlung
- App
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- LeistungId
- libs/database/index.ts
- UnitOfWorkImpl
- UseCase
- query
- KlientRepository
- HaustierErstellen
- .run
- layout.ts
- EmailAdresse
- api/client.ts
- domain/rechnung.rs
- Klient
- NeueLeistung
- SQLiteConnection
- UnitOfWork
- YamsAppApi
- Clock
- .get_current_version
- YamsApiSpec
- domain/adresse.rs
- schema/leistung.rs
- SQLiteUnitOfWork
- FakeDatastore
- .contextualize_with
- KlientErstellen
- participation/index.tsx
- leistung_from_row
- Preis
- base_app_builder
- Seminar
- HaustierErstellung
- Produkt
- Ratio
- Versioned
- StructuredError
- BehandlungRepository
- DatabaseConnection
- _app.tsx
- base_app_builder
- schema/seminar.rs
- SeminarId
- SeminarBuchung
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
- common.rs
- use_cases/seminar.rs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- relations/index.tsx
- Rechnung
- LeistungRepository
- hooks/index.ts
- RechnungRepository
- dialog.ts
- .behandlung_erstellen
- EventStore
- Klient
- .produkt_erstellen
- page.tsx
- SQLiteHaustierRepository
- KlientId
- api/index.ts
- HttpYamsApi
- Zeitraum
- FakeUnitOfWork
- notification.ts
- KlientErstellung
- Menge
- preis.rs
- domain/seminar_termin.rs
- SQLiteKlientRepository
- layout.tsx
- seminar_from_row
- repos.rs
- SeminarTerminId
- Migration
- FakeSeminarTermineRepository
- LeistungIn<S>
- SeminarErstellen
- SeminarRepository
- ProduktRepository
- Haustier
- SQLiteBehandlungRepository
- SeminarTermin

## God Nodes (most connected - your core abstractions)
1. `KlientId` - 71 edges
2. `Versioned` - 70 edges
3. `Preis` - 54 edges
4. `Ratio` - 54 edges
5. `useStore()` - 52 edges
6. `query()` - 42 edges
7. `YamsAppApi` - 41 edges
8. `FakeDatastore` - 41 edges
9. `LeistungId` - 33 edges
10. `E` - 32 edges

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

## Communities (137 total, 21 thin omitted)

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
Cohesion: 0.08
Nodes (40): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+32 more)

### Community 8 - "String"
Cohesion: 0.14
Nodes (33): From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen(), leistung_aus_behandlung_buchen() (+25 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.16
Nodes (27): format_datetime(), parse_datetime(), DateTime, Utc, insert_params(), leistung_id_for(), load_buchungen(), load_termin() (+19 more)

### Community 10 - "requests/seminar.rs"
Cohesion: 0.15
Nodes (22): buchung_id(), parse_preis(), parse_ratio(), Adresse, DateTime, Decimal, Error, Option (+14 more)

### Community 11 - "UpMigration"
Cohesion: 0.14
Nodes (11): Send, Sync, UpMigration, Migration, Error, Option, Transaction, Migration (+3 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "stores/index.tsx"
Cohesion: 0.09
Nodes (39): AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons (+31 more)

### Community 14 - "SQLiteProduktRepository"
Cohesion: 0.47
Nodes (5): Arc, Mutex, Option, Transaction, SQLiteProduktRepository

### Community 15 - "Behandlung"
Cohesion: 0.11
Nodes (13): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+5 more)

### Community 16 - "App"
Cohesion: 0.16
Nodes (13): Self, App, Arc, Box, F, O, ResultReport, T (+5 more)

### Community 17 - "apply_up_migrations"
Cohesion: 0.25
Nodes (8): apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Iterator, Option, DoubleEndedIterator

### Community 18 - "E"
Cohesion: 0.29
Nodes (12): AppliableMigration, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>> (+4 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (12): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+4 more)

### Community 20 - "parse_position_from_row"
Cohesion: 0.19
Nodes (16): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "LeistungId"
Cohesion: 0.17
Nodes (16): LeistungId, Abgehalten, Abgesagt, DateTime, FxHashMap, Into, ResultReport, Utc (+8 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.06
Nodes (21): CompatibilityResult, DatabaseError, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented (+13 more)

### Community 24 - "UnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "UseCase"
Cohesion: 0.13
Nodes (21): LeistungOffen, UseCase, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler (+13 more)

### Community 26 - "query"
Cohesion: 0.10
Nodes (15): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAddress(), deleteAnimal(), patchAnimal(), query(), deleteRace() (+7 more)

### Community 28 - "HaustierErstellen"
Cohesion: 0.21
Nodes (11): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, ResultReport (+3 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "layout.ts"
Cohesion: 0.12
Nodes (9): MenuEntryData, Data, EditClient(), Home, setupStore, Layout, LayoutPage, nextConfig (+1 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "api/client.ts"
Cohesion: 0.17
Nodes (8): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), Addresses

### Community 33 - "domain/rechnung.rs"
Cohesion: 0.13
Nodes (27): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+19 more)

### Community 34 - "Klient"
Cohesion: 0.11
Nodes (17): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, neu(), NeuerKlient, Adresse (+9 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.11
Nodes (21): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, mark_abgerechnet_sets_rechnung_id() (+13 more)

### Community 36 - "SQLiteConnection"
Cohesion: 0.16
Nodes (15): Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path, ResultReport (+7 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "YamsAppApi"
Cohesion: 0.12
Nodes (22): Arc, Haustier, HaustierErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung (+14 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "YamsApiSpec"
Cohesion: 0.05
Nodes (47): BackendServerError, Config, main(), Report, InternalServerError, Self, openapi_service(), Behandlung (+39 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.24
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (15): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungQuelleSeminar, LeistungStatus, Decimal (+7 more)

### Community 44 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "FakeDatastore"
Cohesion: 0.12
Nodes (16): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository, FakeSeminareRepository, Arc, Behandlung (+8 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.21
Nodes (10): ErrorReportExt, Result<T, E>, C, Error, Report, Send, Sync, T (+2 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "participation/index.tsx"
Cohesion: 0.19
Nodes (11): ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation(), relateClientParticipateEvent() (+3 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.17
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 51 - "Preis"
Cohesion: 0.12
Nodes (8): Add, Preis, Output, Self, position_from_leistung(), RechnungIn<S>, Rechnungsposition, Mul

### Community 52 - "base_app_builder"
Cohesion: 0.08
Nodes (39): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), klient_body(), Value (+31 more)

### Community 53 - "Seminar"
Cohesion: 0.14
Nodes (12): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+4 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.11
Nodes (13): NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport, Self (+5 more)

### Community 56 - "Ratio"
Cohesion: 0.21
Nodes (5): LeistungQuelle, Ratio, RatioFehler, Decimal, Self

### Community 57 - "Versioned"
Cohesion: 0.21
Nodes (8): Versioned, FakeHaustiereRepository, FakeKlientenRepository, Haustier, Klient, Leistung, RepositoryResult, Seminar

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "BehandlungRepository"
Cohesion: 0.20
Nodes (4): BehandlungRepository, HaustierRepository, Send, Sync

### Community 60 - "DatabaseConnection"
Cohesion: 0.16
Nodes (6): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection

### Community 61 - "_app.tsx"
Cohesion: 0.11
Nodes (16): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+8 more)

### Community 62 - "base_app_builder"
Cohesion: 0.09
Nodes (47): base_app_builder(), AppBuilder, SetUowProvider, AbrechnungSetup, menge(), mwst_19(), Arc, Klient (+39 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.18
Nodes (22): BuchungUmsatz, Adresse, DateTime, Decimal, From, NaiveDate, Option, Self (+14 more)

### Community 64 - "SeminarId"
Cohesion: 0.14
Nodes (12): SeminarId, aktualisieren_rejects_max_below_confirmed(), Geplant, NeuerSeminarTermin, Adresse, Option, S, Self (+4 more)

### Community 65 - "SeminarBuchung"
Cohesion: 0.13
Nodes (6): Item, Iterator, Seminar, SeminarBuchung, SeminarTerminIn<S>, SeminarBuchungStatus

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

### Community 71 - "Seminar — Domain-Spezifikation"
Cohesion: 0.10
Nodes (18): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+10 more)

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
Cohesion: 0.16
Nodes (25): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+17 more)

### Community 80 - "api/types.ts"
Cohesion: 0.11
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 89 - "common.rs"
Cohesion: 0.17
Nodes (22): RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, menge_to_str(), parse_decimal(), parse_haustier_id() (+14 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.16
Nodes (19): buchung_umsatz(), BuchungUmsatz, NaiveDate, Report, Self, Vec, SeminarBuchungAnlegenFehler, SeminarBuchungStornierenFehler (+11 more)

### Community 100 - "relations/index.tsx"
Cohesion: 0.12
Nodes (20): AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, clientSearched(), deleteClientRelation(), relateClients() (+12 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.19
Nodes (18): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+10 more)

### Community 105 - "dialog.ts"
Cohesion: 0.10
Nodes (8): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore, Store, SettingsStore

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.36
Nodes (6): Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "EventStore"
Cohesion: 0.12
Nodes (3): EventResponse, SeminarResponse, EventStore

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.32
Nodes (6): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 112 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 113 - "KlientId"
Cohesion: 0.16
Nodes (14): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+6 more)

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 116 - "Zeitraum"
Cohesion: 0.20
Nodes (10): DateTime, Display, Formatter, Self, Utc, utc(), Zeitraum, zeitraum_accepts_ende_after_beginn() (+2 more)

### Community 117 - "FakeUnitOfWork"
Cohesion: 0.28
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 118 - "notification.ts"
Cohesion: 0.13
Nodes (15): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+7 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellung, Adresse, Error, NaiveDate, Self, TryFrom

### Community 120 - "Menge"
Cohesion: 0.24
Nodes (4): Menge, MengeFehler, Decimal, Self

### Community 121 - "preis.rs"
Cohesion: 0.29
Nodes (8): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 122 - "domain/seminar_termin.rs"
Cohesion: 0.36
Nodes (15): absagen_archives_buchungen(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient(), klient() (+7 more)

### Community 123 - "SQLiteKlientRepository"
Cohesion: 0.26
Nodes (9): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 124 - "layout.tsx"
Cohesion: 0.31
Nodes (5): geistMono, geistSans, metadata, Providers(), createQueryClient()

### Community 125 - "seminar_from_row"
Cohesion: 0.27
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, seminar_from_row() (+1 more)

### Community 127 - "SeminarTerminId"
Cohesion: 0.19
Nodes (11): abgehalten_use_case(), ExecutionContext, Arc, SeminarTerminId, Error, SeminarTermin, SeminarBuchungAnlegen, SeminarBuchungStornieren (+3 more)

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (5): Migration, Error, Option, Transaction, table_exists()

### Community 129 - "FakeSeminarTermineRepository"
Cohesion: 0.29
Nodes (5): FakeSeminarTermineRepository, NaiveDate, Rechnung, SeminarTermin, Vec

### Community 131 - "SeminarErstellen"
Cohesion: 0.32
Nodes (7): Option, Seminar, SeminarOrt, TimeDelta, SeminarErstellen, SeminarTerminAktualisieren, SeminarTerminPlanen

### Community 134 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 135 - "SQLiteBehandlungRepository"
Cohesion: 0.47
Nodes (5): Arc, Mutex, Option, Transaction, SQLiteBehandlungRepository

## Knowledge Gaps
- **239 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+234 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **21 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `SeminarErstellen`, `Haustier`, `YAMSFrontendConfig`, `termin_from_parts`, `requests/seminar.rs`, `Behandlung`, `domain/kontakt.rs`, `parse_position_from_row`, `LeistungId`, `UseCase`, `HaustierErstellen`, `EmailAdresse`, `Klient`, `NeueLeistung`, `YamsApiSpec`, `domain/adresse.rs`, `schema/leistung.rs`, `KlientErstellen`, `leistung_from_row`, `Preis`, `Seminar`, `HaustierErstellung`, `Produkt`, `StructuredError`, `schema/seminar.rs`, `SeminarId`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `KlientId`, `FakeUnitOfWork`, `KlientErstellung`, `SeminarTerminId`?**
  _High betweenness centrality (0.234) - this node is a cross-community bridge._
- **Why does `KlientId` connect `KlientId` to `FakeSeminarTermineRepository`, `LeistungIn<S>`, `termin_from_parts`, `requests/seminar.rs`, `parse_position_from_row`, `UseCase`, `HaustierErstellen`, `domain/rechnung.rs`, `Klient`, `NeueLeistung`, `YamsAppApi`, `Preis`, `HaustierErstellung`, `Versioned`, `SeminarId`, `SeminarBuchung`, `requests/abrechnung.rs`, `common.rs`, `use_cases/seminar.rs`, `SQLiteHaustierRepository`, `domain/seminar_termin.rs`, `SQLiteKlientRepository`, `SeminarTerminId`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Why does `Preis` connect `Preis` to `common.rs`, `LeistungIn<S>`, `SeminarErstellen`, `requests/seminar.rs`, `requests/abrechnung.rs`, `Behandlung`, `use_cases/seminar.rs`, `parse_position_from_row`, `Seminar`, `Produkt`, `Ratio`, `preis.rs`, `Menge`, `UseCase`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _239 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._