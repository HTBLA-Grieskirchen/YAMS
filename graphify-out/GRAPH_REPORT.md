# Graph Report - yams  (2026-08-25)

## Corpus Check
- 268 files · ~331,106 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2832 nodes · 6951 edges · 148 communities (125 shown, 23 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 227 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4cca9a77`
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
- termin_from_parts
- ObjectStoreError
- UpMigration
- compilerOptions
- ClientItem.tsx
- SQLiteProduktRepository
- Behandlung
- repos.rs
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- .from_parts
- stores/index.tsx
- tests.rs
- LeistungOffen
- makeRecordForTable
- dialog.ts
- HaustierErstellen
- .run
- SeminarTerminId
- EmailAdresse
- api/client.ts
- RechnungId
- KlientId
- NeueLeistung
- PdfDokument
- UnitOfWork
- YamsAppApi
- Clock
- .get_current_version
- YamsApiSpec
- domain/adresse.rs
- schema/leistung.rs
- SQLiteUnitOfWork
- Versioned
- .contextualize_with
- KlientErstellen
- participation/index.tsx
- leistung_from_row
- Menge
- base_app_builder
- Preis
- HaustierErstellung
- Produkt
- ratio.rs
- Klientbericht
- StructuredError
- BehandlungRepository
- DatabaseConnection
- PdfFileResponse
- base_app_builder
- schema/seminar.rs
- SeminarId
- AnimalStore
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
- use_cases/seminar.rs
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
- Rechnungsposition
- useStore
- .behandlung_erstellen
- SQLiteConnection
- Klient
- .produkt_erstellen
- page.tsx
- SQLiteHaustierRepository
- HaustierId
- api/index.ts
- HttpYamsApi
- Zeitraum
- InMemoryObjectStore
- notification.ts
- KlientErstellung
- FakeObjectStore
- preis.rs
- LeistungId
- ObjectStore
- ExecutionContext
- seminar_from_row
- openapi_service
- UseCase
- Migration
- SeminarTerminRepository
- TypicalJsonResponse
- main
- .poll_next
- KlientRepository
- Haustier
- behandlung_from_row
- EventResponse
- .haustier_erstellen
- LeistungRepository
- InternalServerError
- RechnungRepository
- SeminarRepository
- collect_object
- .seminar_umsatz_prognose
- LeftMenuLayout.tsx
- .tagesabschluss_durchführen
- ClientRelationResponse
- document_text

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
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `Config` --references--> `String`  [EXTRACTED]
  backend/server/src/main.rs → crates/yams-api/src/errors/internal_error.rs
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

## Communities (148 total, 23 thin omitted)

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
Cohesion: 0.11
Nodes (38): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+30 more)

### Community 8 - "String"
Cohesion: 0.14
Nodes (35): From, String, alle_haustiere(), behandlung_erstellen(), haustier_by_id(), haustier_erstellen(), klient_erstellen(), leistung_aus_behandlung_buchen() (+27 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.16
Nodes (27): format_datetime(), parse_datetime(), DateTime, Utc, insert_params(), leistung_id_for(), load_buchungen(), load_termin() (+19 more)

### Community 10 - "ObjectStoreError"
Cohesion: 0.23
Nodes (15): ObjectStoreError, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing(), put_get_roundtrip(), rejects_parent_segment(), Arc (+7 more)

### Community 11 - "UpMigration"
Cohesion: 0.14
Nodes (11): Send, Sync, UpMigration, Migration, Error, Option, Transaction, Migration (+3 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "ClientItem.tsx"
Cohesion: 0.30
Nodes (8): AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, deleteAnimal(), LiveRefresher

### Community 14 - "SQLiteProduktRepository"
Cohesion: 0.47
Nodes (5): Arc, Mutex, Option, Transaction, SQLiteProduktRepository

### Community 15 - "Behandlung"
Cohesion: 0.14
Nodes (9): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+1 more)

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
Nodes (16): parse_klient_id(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - ".from_parts"
Cohesion: 0.14
Nodes (13): Abgehalten, Abgesagt, DateTime, From, FxHashMap, Into, Self, Utc (+5 more)

### Community 23 - "stores/index.tsx"
Cohesion: 0.05
Nodes (25): CompatibilityResult, DatabaseError, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented (+17 more)

### Community 24 - "tests.rs"
Cohesion: 0.21
Nodes (33): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+25 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.15
Nodes (18): LeistungOffen, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen (+10 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.12
Nodes (20): AnimalAddItem, AnimalComboBox, patchAnimal(), patchRace(), createSeminar(), ensureSeminar(), Address, Animal (+12 more)

### Community 27 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 28 - "HaustierErstellen"
Cohesion: 0.21
Nodes (11): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, ResultReport (+3 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "SeminarTerminId"
Cohesion: 0.09
Nodes (15): seminar_betrag_full_rabatt_is_zero(), seminar_betrag_uses_nach_rabatt(), Item, Iterator, S, Seminar, Uuid, Vec (+7 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "api/client.ts"
Cohesion: 0.19
Nodes (6): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useBackend()

### Community 33 - "RechnungId"
Cohesion: 0.13
Nodes (27): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+19 more)

### Community 34 - "KlientId"
Cohesion: 0.11
Nodes (18): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, KlientId, neu(), NeuerKlient (+10 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.09
Nodes (21): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, LeistungIn<S> (+13 more)

### Community 36 - "PdfDokument"
Cohesion: 0.12
Nodes (17): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, PdfDokument, PdfRenderError, FakePdfRenderer (+9 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.17
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "YamsAppApi"
Cohesion: 0.12
Nodes (21): Arc, Haustier, HaustierErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, NaiveDate (+13 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "YamsApiSpec"
Cohesion: 0.33
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

### Community 45 - "Versioned"
Cohesion: 0.06
Nodes (47): Versioned, FakeBehandlungenRepository, FakeDatastore, FakeHaustiereRepository, FakeKlientenRepository, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository (+39 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.21
Nodes (10): ErrorReportExt, Result<T, E>, C, Error, Report, Send, Sync, T (+2 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate

### Community 48 - "participation/index.tsx"
Cohesion: 0.12
Nodes (16): clientSearched(), EventParticipants, participationSearchedClient(), SmallSearchField, ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent() (+8 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.17
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 51 - "Menge"
Cohesion: 0.24
Nodes (4): Menge, MengeFehler, Decimal, Self

### Community 52 - "base_app_builder"
Cohesion: 0.08
Nodes (40): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), klient_body(), Value (+32 more)

### Community 53 - "Preis"
Cohesion: 0.11
Nodes (17): Add, Preis, Output, Ratio, NeuesSeminar, preis(), Into, Option (+9 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.11
Nodes (13): NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport, Self (+5 more)

### Community 56 - "ratio.rs"
Cohesion: 0.22
Nodes (3): RatioFehler, Decimal, Self

### Community 57 - "Klientbericht"
Cohesion: 0.12
Nodes (23): Klientbericht, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse, NaiveDate (+15 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "BehandlungRepository"
Cohesion: 0.20
Nodes (4): BehandlungRepository, ProduktRepository, Send, Sync

### Community 60 - "DatabaseConnection"
Cohesion: 0.16
Nodes (6): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection

### Community 61 - "PdfFileResponse"
Cohesion: 0.29
Nodes (9): Binary, PdfFileResponse, C, From, Report, StatusCode, T, Vec (+1 more)

### Community 62 - "base_app_builder"
Cohesion: 0.06
Nodes (64): App, Arc, Box, F, O, ResultReport, T, U (+56 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.15
Nodes (25): BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, Error, From, NaiveDate (+17 more)

### Community 64 - "SeminarId"
Cohesion: 0.27
Nodes (6): SeminarId, Geplant, NeuerSeminarTermin, Adresse, Option, SeminarOrt

### Community 65 - "AnimalStore"
Cohesion: 0.12
Nodes (3): AnimalResponse, RaceResponse, AnimalStore

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
Nodes (46): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+38 more)

### Community 80 - "api/types.ts"
Cohesion: 0.10
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.18
Nodes (23): adresse_dict(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict(), preis(), ratio() (+15 more)

### Community 89 - "common.rs"
Cohesion: 0.20
Nodes (17): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_menge(), parse_preis(), parse_ratio(), parse_rechnung_id(), parse_uuid() (+9 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.15
Nodes (20): buchung_umsatz(), BuchungUmsatz, NaiveDate, Report, Self, Vec, SeminarBuchungAnlegenFehler, SeminarBuchungStornierenFehler (+12 more)

### Community 100 - "RepositoryError"
Cohesion: 0.26
Nodes (10): RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, AsRef, Path, ResultReport (+2 more)

### Community 101 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 102 - "SharedUnitOfWorkImpl"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.17
Nodes (20): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+12 more)

### Community 104 - "Rechnungsposition"
Cohesion: 0.16
Nodes (3): position_from_leistung(), RechnungIn<S>, Rechnungsposition

### Community 105 - "useStore"
Cohesion: 0.05
Nodes (59): useAddresses(), queryClient, AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages (+51 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.36
Nodes (6): Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "SQLiteConnection"
Cohesion: 0.23
Nodes (10): Connection, InstanceType, Arc, Deref, Mutex, Target, TempDir, SQLiteConnection (+2 more)

### Community 108 - "Klient"
Cohesion: 0.22
Nodes (11): Klient, KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate (+3 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.32
Nodes (6): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 112 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 113 - "HaustierId"
Cohesion: 0.15
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+5 more)

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 116 - "Zeitraum"
Cohesion: 0.20
Nodes (10): DateTime, Display, Formatter, Self, Utc, utc(), Zeitraum, zeitraum_accepts_ende_after_beginn() (+2 more)

### Community 117 - "InMemoryObjectStore"
Cohesion: 0.24
Nodes (11): get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap, Mutex, ObjectStream, Option (+3 more)

### Community 118 - "notification.ts"
Cohesion: 0.13
Nodes (15): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+7 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.22
Nodes (9): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Error, Mobilnummer, NaiveDate, Self (+1 more)

### Community 120 - "FakeObjectStore"
Cohesion: 0.19
Nodes (9): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStream, Option, ResultReport, Self (+1 more)

### Community 121 - "preis.rs"
Cohesion: 0.21
Nodes (9): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal (+1 more)

### Community 122 - "LeistungId"
Cohesion: 0.24
Nodes (21): LeistungId, absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity() (+13 more)

### Community 123 - "ObjectStore"
Cohesion: 0.29
Nodes (12): ObjectStore, Send, Sync, pdf_laden(), rechnung_key_uses_uuid(), rechnung_object_key(), rechnung_pdf_laden(), Option (+4 more)

### Community 124 - "ExecutionContext"
Cohesion: 0.21
Nodes (7): ExecutionContext, Arc, PdfRenderer, Send, Sync, ProduktErstellen, Produkt

### Community 125 - "seminar_from_row"
Cohesion: 0.27
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, seminar_from_row() (+1 more)

### Community 126 - "openapi_service"
Cohesion: 0.25
Nodes (7): openapi_service(), Into, Item, Self, IntoIterator, OpenApiService, ServerObject

### Community 127 - "UseCase"
Cohesion: 0.19
Nodes (14): UseCase, Error, Option, Seminar, SeminarOrt, SeminarTermin, TimeDelta, SeminarBuchungAnlegen (+6 more)

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (5): Migration, Error, Option, Transaction, table_exists()

### Community 130 - "TypicalJsonResponse"
Cohesion: 0.13
Nodes (13): Behandlung, BehandlungErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung (+5 more)

### Community 131 - "main"
Cohesion: 0.21
Nodes (10): BackendServerError, catch_panic(), Config, main(), Option, Report, init_tracing(), CatchPanic (+2 more)

### Community 132 - ".poll_next"
Cohesion: 0.22
Nodes (8): FileStream, Context, Item, Option, Pin, File, Poll, Stream

### Community 134 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 135 - "behandlung_from_row"
Cohesion: 0.27
Nodes (9): behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 142 - "collect_object"
Cohesion: 0.60
Nodes (4): collect_object(), once_stream(), ObjectStream, Vec

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "LeftMenuLayout.tsx"
Cohesion: 0.21
Nodes (4): MenuEntryData, Home, setupStore, Layout

### Community 147 - "document_text"
Cohesion: 0.50
Nodes (4): collect_frame_text(), document_text(), Frame, PagedDocument

## Knowledge Gaps
- **242 isolated node(s):** `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+237 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `TypicalJsonResponse`, `main`, `Haustier`, `YAMSFrontendConfig`, `termin_from_parts`, `InternalServerError`, `Behandlung`, `domain/kontakt.rs`, `parse_position_from_row`, `document_text`, `.from_parts`, `tests.rs`, `LeistungOffen`, `HaustierErstellen`, `EmailAdresse`, `KlientId`, `NeueLeistung`, `domain/adresse.rs`, `schema/leistung.rs`, `Versioned`, `KlientErstellen`, `leistung_from_row`, `Menge`, `base_app_builder`, `Preis`, `HaustierErstellung`, `Produkt`, `Klientbericht`, `StructuredError`, `schema/seminar.rs`, `SeminarId`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `Rechnungsposition`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `HaustierId`, `InMemoryObjectStore`, `KlientErstellung`, `FakeObjectStore`, `ObjectStore`, `ExecutionContext`, `UseCase`?**
  _High betweenness centrality (0.224) - this node is a cross-community bridge._
- **Why does `KlientId` connect `KlientId` to `termin_from_parts`, `parse_position_from_row`, `.from_parts`, `tests.rs`, `LeistungOffen`, `HaustierErstellen`, `SeminarTerminId`, `RechnungId`, `NeueLeistung`, `PdfDokument`, `YamsAppApi`, `Versioned`, `HaustierErstellung`, `Klientbericht`, `requests/abrechnung.rs`, `use_cases/seminar.rs`, `Rechnungsposition`, `SQLiteHaustierRepository`, `HaustierId`, `LeistungId`, `UseCase`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _242 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._