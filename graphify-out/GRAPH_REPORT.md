# Graph Report - yams  (2026-08-26)

## Corpus Check
- 269 files · ~331,565 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2847 nodes · 6971 edges · 146 communities (124 shown, 22 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d572fc74`
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
- ObjectStoreError
- UpMigration
- compilerOptions
- relations/index.tsx
- Versioned
- Behandlung
- HaustierRepository
- apply_up_migrations
- E
- domain/kontakt.rs
- SQLiteRechnungRepository
- Versioned<T>
- SeminarBuchungId
- libs/database/index.ts
- tests.rs
- LeistungOffen
- query
- dialog.ts
- HaustierErstellen
- .run
- SeminarBuchung
- EmailAdresse
- api/client.ts
- RechnungId
- Klient
- LeistungId
- PdfDokument
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
- Menge
- base_app_builder
- Seminar
- HaustierErstellung
- Produkt
- Ratio
- Klientbericht
- StructuredError
- BehandlungRepository
- DatabaseConnection
- StreamBinaryResponse
- base_app_builder
- schema/seminar.rs
- Option
- AddressStore
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
- yams-typstreports/src/lib.rs
- common.rs
- ExecutionContext
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- RepositoryError
- Rechnung
- SharedUnitOfWorkImpl
- hooks/index.ts
- Preis
- stores/index.tsx
- .behandlung_erstellen
- AnimalStore
- Klient
- .produkt_erstellen
- page.tsx
- SQLiteHaustierRepository
- KlientId
- api/index.ts
- HttpYamsApi
- Zeitraum
- behandlung_from_row
- notification.ts
- KlientErstellung
- produkt_from_row
- preis.rs
- domain/seminar_termin.rs
- SeminarTerminId
- FakeUnitOfWork
- SeminarId
- StreamBody
- RechnungIn<S>
- Migration
- SQLiteKlientRepository
- TypicalJsonResponse
- openapi_service
- repos.rs
- KlientRepository
- Haustier
- SeminarTermin
- ProduktRepository
- InternalServerError
- RechnungRepository
- SeminarRepository
- .seminar_umsatz_prognose
- paths.ts
- .tagesabschluss_durchführen
- document_text
- layout.tsx
- FakeKlientenRepository

## God Nodes (most connected - your core abstractions)
1. `KlientId` - 74 edges
2. `Versioned` - 70 edges
3. `Preis` - 56 edges
4. `Ratio` - 56 edges
5. `useStore()` - 52 edges
6. `YamsAppApi` - 45 edges
7. `query()` - 42 edges
8. `FakeDatastore` - 41 edges
9. `SeminarTerminId` - 38 edges
10. `ExecutionContext` - 33 edges

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

## Communities (146 total, 22 thin omitted)

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
Cohesion: 0.13
Nodes (30): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+22 more)

### Community 8 - "String"
Cohesion: 0.14
Nodes (35): From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen(), leistung_aus_behandlung_buchen() (+27 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.16
Nodes (27): format_datetime(), parse_datetime(), DateTime, Utc, insert_params(), leistung_id_for(), load_buchungen(), load_termin() (+19 more)

### Community 10 - "ObjectStoreError"
Cohesion: 0.06
Nodes (47): get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap, Mutex, ObjectStream, Option (+39 more)

### Community 11 - "UpMigration"
Cohesion: 0.14
Nodes (11): Send, Sync, UpMigration, Migration, Error, Option, Transaction, Migration (+3 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "relations/index.tsx"
Cohesion: 0.18
Nodes (11): clientSearched(), deleteClientRelation(), relateClients(), updateClientRelation(), ClientRelationResponse, ClientDetail, AddRelationDialog, ClientRelations (+3 more)

### Community 14 - "Versioned"
Cohesion: 0.17
Nodes (11): Versioned, FakeHaustiereRepository, FakeSeminareRepository, FakeSeminarTermineRepository, Haustier, Leistung, NaiveDate, RepositoryResult (+3 more)

### Community 15 - "Behandlung"
Cohesion: 0.14
Nodes (10): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+2 more)

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
Cohesion: 0.21
Nodes (12): preis_to_str(), geladene_rechnung_from_parts(), RechnungRowData, Arc, Mutex, NaiveDate, Option, Rechnung (+4 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "SeminarBuchungId"
Cohesion: 0.14
Nodes (15): seminar_betrag_full_rabatt_is_zero(), seminar_betrag_uses_nach_rabatt(), Abgehalten, Abgesagt, DateTime, FxHashMap, Into, Self (+7 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.05
Nodes (22): CompatibilityResult, DatabaseError, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented (+14 more)

### Community 24 - "tests.rs"
Cohesion: 0.21
Nodes (33): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+25 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.12
Nodes (21): LeistungOffen, RechnungOffen, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler (+13 more)

### Community 26 - "query"
Cohesion: 0.09
Nodes (23): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAddress(), deleteAnimal(), patchAnimal(), query(), deleteRace() (+15 more)

### Community 27 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 28 - "HaustierErstellen"
Cohesion: 0.21
Nodes (11): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, ResultReport (+3 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "SeminarBuchung"
Cohesion: 0.12
Nodes (9): Item, Iterator, S, Seminar, Vec, SeminarBuchung, SeminarTerminIn, SeminarTerminIn<S> (+1 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "api/client.ts"
Cohesion: 0.17
Nodes (8): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), Addresses

### Community 33 - "RechnungId"
Cohesion: 0.13
Nodes (28): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+20 more)

### Community 34 - "Klient"
Cohesion: 0.09
Nodes (21): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, neu(), NeuerKlient, Adresse (+13 more)

### Community 35 - "LeistungId"
Cohesion: 0.10
Nodes (20): Abgerechnet, Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungId, LeistungIn, LeistungIn<S> (+12 more)

### Community 36 - "PdfDokument"
Cohesion: 0.12
Nodes (17): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, PdfDokument, PdfRenderError, FakePdfRenderer (+9 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "YamsAppApi"
Cohesion: 0.12
Nodes (21): Arc, Haustier, HaustierErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung (+13 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "YamsApiSpec"
Cohesion: 0.32
Nodes (5): Path, SeminarTermin, SeminarUmsatzVorschau, Uuid, YamsApiSpec

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
Nodes (16): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository, Arc, Behandlung, Clone (+8 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.21
Nodes (10): ErrorReportExt, Result<T, E>, C, Error, Report, Send, Sync, T (+2 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "participation/index.tsx"
Cohesion: 0.30
Nodes (10): ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation(), relateClientParticipateEvent() (+2 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.21
Nodes (13): parse_rechnung_id(), leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex, NaiveDate, Option (+5 more)

### Community 51 - "Menge"
Cohesion: 0.22
Nodes (4): Menge, MengeFehler, Decimal, Self

### Community 52 - "base_app_builder"
Cohesion: 0.08
Nodes (40): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), klient_body(), Value (+32 more)

### Community 53 - "Seminar"
Cohesion: 0.17
Nodes (11): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Seminar (+3 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.14
Nodes (10): produkt_betrag_multiplies_menge(), NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport (+2 more)

### Community 56 - "Ratio"
Cohesion: 0.18
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 57 - "Klientbericht"
Cohesion: 0.12
Nodes (22): Klientbericht, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse, NaiveDate (+14 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "BehandlungRepository"
Cohesion: 0.20
Nodes (4): BehandlungRepository, LeistungRepository, Send, Sync

### Community 60 - "DatabaseConnection"
Cohesion: 0.16
Nodes (6): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.31
Nodes (9): C, From, ObjectStream, Report, StatusCode, T, StreamBinaryResponse, TypicalJsonResponse<T> (+1 more)

### Community 62 - "base_app_builder"
Cohesion: 0.06
Nodes (65): App, Arc, Box, F, O, ResultReport, T, U (+57 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.12
Nodes (28): NaiveDate, SeminarUmsatzPrognose, SeminarUmsatzVorschau, BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal (+20 more)

### Community 64 - "Option"
Cohesion: 0.26
Nodes (6): Geplant, NeuerSeminarTermin, Adresse, Option, SeminarOrt, SeminarTerminGeplant

### Community 65 - "AddressStore"
Cohesion: 0.10
Nodes (4): AddressResponse, AddressStore, Store, SettingsStore

### Community 66 - "Integration Test Workflow"
Cohesion: 0.53
Nodes (6): mise, mise-action, Release Workflow, Integration Test Workflow, tauri-action, Unit Test Workflow

### Community 67 - "frontend-legacy/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "src/api/schema.d.ts"
Cohesion: 0.13
Nodes (14): components, $defs, leistungQuelle_LeistungQuelleBehandlungTypValues, leistungQuelle_LeistungQuelleManuellTypValues, leistungQuelle_LeistungQuelleProduktTypValues, leistungQuelle_LeistungQuelleSeminarTypValues, leistungStatusValues, operations (+6 more)

### Community 69 - "yams-core"
Cohesion: 0.46
Nodes (8): molting, yams, yams-core, yams-fakes, yams-filesystemstore, yams-persistence, yams-server, yams-typstreports

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
Cohesion: 0.08
Nodes (44): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+36 more)

### Community 80 - "api/types.ts"
Cohesion: 0.10
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.18
Nodes (23): adresse_dict(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict(), preis(), ratio() (+15 more)

### Community 89 - "common.rs"
Cohesion: 0.15
Nodes (24): format_naive_date(), menge_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_naive_date(), parse_preis() (+16 more)

### Community 92 - "ExecutionContext"
Cohesion: 0.07
Nodes (41): ExecutionContext, Arc, ObjectStore, Send, Sync, PdfRenderer, Send, Sync (+33 more)

### Community 100 - "RepositoryError"
Cohesion: 0.14
Nodes (21): AtomicBool, Connection, RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, InstanceType (+13 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 102 - "SharedUnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.17
Nodes (20): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+12 more)

### Community 104 - "Preis"
Cohesion: 0.16
Nodes (5): Add, Preis, Output, Self, Mul

### Community 105 - "stores/index.tsx"
Cohesion: 0.09
Nodes (39): AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton, EditButtons (+31 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.36
Nodes (6): Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "AnimalStore"
Cohesion: 0.14
Nodes (3): AnimalResponse, RaceResponse, AnimalStore

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

### Community 117 - "behandlung_from_row"
Cohesion: 0.27
Nodes (9): behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 118 - "notification.ts"
Cohesion: 0.13
Nodes (15): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+7 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.22
Nodes (9): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Error, Mobilnummer, NaiveDate, Self (+1 more)

### Community 120 - "produkt_from_row"
Cohesion: 0.27
Nodes (9): produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row, Transaction (+1 more)

### Community 121 - "preis.rs"
Cohesion: 0.29
Nodes (8): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 122 - "domain/seminar_termin.rs"
Cohesion: 0.27
Nodes (19): absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient() (+11 more)

### Community 123 - "SeminarTerminId"
Cohesion: 0.22
Nodes (13): abgehalten_use_case(), SeminarTerminAbsage, SeminarTerminId, rechnung_key_uses_uuid(), rechnung_object_key(), rechnung_pdf_laden(), ObjectStream, Option (+5 more)

### Community 124 - "FakeUnitOfWork"
Cohesion: 0.28
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 125 - "SeminarId"
Cohesion: 0.18
Nodes (11): Uuid, SeminarId, Arc, Mutex, Option, RepositoryResult, Row, Seminar (+3 more)

### Community 126 - "StreamBody"
Cohesion: 0.22
Nodes (6): Self, StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (5): Migration, Error, Option, Transaction, table_exists()

### Community 129 - "SQLiteKlientRepository"
Cohesion: 0.26
Nodes (9): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 130 - "TypicalJsonResponse"
Cohesion: 0.12
Nodes (14): Behandlung, BehandlungErstellung, HaustierErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung (+6 more)

### Community 131 - "openapi_service"
Cohesion: 0.14
Nodes (14): BackendServerError, catch_panic(), Config, main(), Report, init_tracing(), CatchPanic, openapi_service() (+6 more)

### Community 134 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "paths.ts"
Cohesion: 0.05
Nodes (34): queryClient, AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, MenuEntryData, LanguagePicker (+26 more)

### Community 145 - ".tagesabschluss_durchführen"
Cohesion: 0.33
Nodes (4): Haustier, Rechnung, TagesabschlussErstellung, Vec

### Community 147 - "document_text"
Cohesion: 0.50
Nodes (4): collect_frame_text(), document_text(), Frame, PagedDocument

### Community 148 - "layout.tsx"
Cohesion: 0.31
Nodes (5): geistMono, geistSans, metadata, Providers(), createQueryClient()

## Knowledge Gaps
- **242 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+237 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `TypicalJsonResponse`, `openapi_service`, `Haustier`, `YAMSFrontendConfig`, `termin_from_parts`, `ObjectStoreError`, `InternalServerError`, `Behandlung`, `domain/kontakt.rs`, `SQLiteRechnungRepository`, `document_text`, `SeminarBuchungId`, `tests.rs`, `LeistungOffen`, `HaustierErstellen`, `EmailAdresse`, `RechnungId`, `Klient`, `LeistungId`, `domain/adresse.rs`, `schema/leistung.rs`, `KlientErstellen`, `Menge`, `base_app_builder`, `Seminar`, `HaustierErstellung`, `Produkt`, `Klientbericht`, `StructuredError`, `schema/seminar.rs`, `Option`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `ExecutionContext`, `Rechnung`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `KlientId`, `KlientErstellung`, `SeminarTerminId`, `FakeUnitOfWork`?**
  _High betweenness centrality (0.251) - this node is a cross-community bridge._
- **Why does `Ratio` connect `Ratio` to `common.rs`, `RechnungId`, `LeistungId`, `Preis`, `requests/abrechnung.rs`, `Behandlung`, `Menge`, `LeistungOffen`, `Seminar`, `SeminarBuchungId`, `Produkt`, `base_app_builder`, `Klientbericht`, `domain/seminar_termin.rs`, `tests.rs`, `ExecutionContext`, `SeminarBuchung`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _242 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._