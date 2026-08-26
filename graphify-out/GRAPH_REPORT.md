# Graph Report - yams  (2026-08-26)

## Corpus Check
- 269 files · ~331,617 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2850 nodes · 6975 edges · 149 communities (127 shown, 22 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `44b44779`
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
- LeistungId
- libs/database/index.ts
- tests.rs
- LeistungOffen
- makeRecordForTable
- dialog.ts
- UseCase
- .run
- SeminarBuchungId
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
- requests/seminar.rs
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
- SeminarId
- EventStore
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
- Preis
- stores/index.tsx
- .behandlung_erstellen
- SeminarTermin
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
- domain/seminar_termin.rs
- ObjectStore
- FakeUnitOfWork
- seminar_from_row
- ExecutionContext
- SQLiteConnection
- Migration
- SQLiteKlientRepository
- YamsApiSpec
- server/src/main.rs
- repos.rs
- KlientRepository
- Haustier
- .poll_next
- ProduktRepository
- LeistungIn<S>
- !.next
- InternalServerError
- RechnungRepository
- SeminarRepository
- ports/object_store.rs
- .seminar_umsatz_prognose
- ClientItem.tsx
- wal_connection_race.rs
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

## Communities (149 total, 22 thin omitted)

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
Nodes (32): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, EventForm (+24 more)

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

### Community 13 - "relations/index.tsx"
Cohesion: 0.18
Nodes (11): clientSearched(), deleteClientRelation(), relateClients(), updateClientRelation(), ClientRelationResponse, ClientDetail, AddRelationDialog, ClientRelations (+3 more)

### Community 14 - "Versioned"
Cohesion: 0.17
Nodes (11): Versioned, FakeHaustiereRepository, FakeSeminareRepository, FakeSeminarTermineRepository, Haustier, Leistung, NaiveDate, RepositoryResult (+3 more)

### Community 15 - "Behandlung"
Cohesion: 0.09
Nodes (18): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+10 more)

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
Cohesion: 0.18
Nodes (15): parse_klient_id(), preis_to_str(), geladene_rechnung_from_parts(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+7 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.13
Nodes (8): Clone, Deref, Formatter, T, Target, Versioned<T>, PartialEq, PartialOrd

### Community 22 - "LeistungId"
Cohesion: 0.18
Nodes (14): LeistungId, Abgehalten, Abgesagt, DateTime, FxHashMap, Into, ResultReport, Utc (+6 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.05
Nodes (27): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+19 more)

### Community 24 - "tests.rs"
Cohesion: 0.22
Nodes (31): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+23 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.15
Nodes (18): LeistungOffen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler (+10 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.07
Nodes (24): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAnimal(), patchAnimal(), patchRace(), createSeminar(), ensureSeminar() (+16 more)

### Community 27 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 28 - "UseCase"
Cohesion: 0.20
Nodes (12): UseCase, HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report (+4 more)

### Community 29 - ".run"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 30 - "SeminarBuchungId"
Cohesion: 0.12
Nodes (8): Item, Iterator, Seminar, Uuid, SeminarBuchung, SeminarBuchungId, SeminarTerminIn<S>, SeminarBuchungStatus

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
Cohesion: 0.10
Nodes (21): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, mark_abgerechnet_sets_rechnung_id() (+13 more)

### Community 36 - "PdfDokument"
Cohesion: 0.14
Nodes (12): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, PdfDokument, FakePdfRenderer, Arc (+4 more)

### Community 37 - "UnitOfWork"
Cohesion: 0.18
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, ResultReport, Self, UnitOfWork, UnitOfWork<'a>

### Community 38 - "YamsAppApi"
Cohesion: 0.11
Nodes (23): Arc, Haustier, HaustierErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, NaiveDate (+15 more)

### Community 39 - "Clock"
Cohesion: 0.11
Nodes (14): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync, FixedClock (+6 more)

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.14
Nodes (22): abgehalten_use_case(), buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Error, Option (+14 more)

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
Cohesion: 0.17
Nodes (11): ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation(), relateClientParticipateEvent() (+3 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.17
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 51 - "Menge"
Cohesion: 0.24
Nodes (4): Menge, MengeFehler, Decimal, Self

### Community 52 - "base_app_builder"
Cohesion: 0.08
Nodes (40): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), klient_body(), Value (+32 more)

### Community 53 - "Seminar"
Cohesion: 0.15
Nodes (12): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+4 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.09
Nodes (18): NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport, Self (+10 more)

### Community 56 - "Ratio"
Cohesion: 0.15
Nodes (8): Rechnungspositionsbericht, LeistungQuelle, Ratio, RatioFehler, Decimal, Self, menge(), position()

### Community 57 - "Klientbericht"
Cohesion: 0.13
Nodes (21): Klientbericht, PraxisAngaben, Rechnungsbericht, Adresse, DateTime, EmailAdresse, NaiveDate, Option (+13 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "BehandlungRepository"
Cohesion: 0.20
Nodes (4): BehandlungRepository, LeistungRepository, Send, Sync

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.11
Nodes (21): openapi_service(), C, From, Into, Item, ObjectStream, Report, Response (+13 more)

### Community 62 - "base_app_builder"
Cohesion: 0.06
Nodes (65): App, Arc, Box, F, O, ResultReport, T, U (+57 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.15
Nodes (25): BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, Error, From, NaiveDate (+17 more)

### Community 64 - "SeminarId"
Cohesion: 0.16
Nodes (12): SeminarId, Geplant, NeuerSeminarTermin, Adresse, From, Option, S, Self (+4 more)

### Community 65 - "EventStore"
Cohesion: 0.09
Nodes (4): EventResponse, EventStore, Store, SettingsStore

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
Cohesion: 0.17
Nodes (24): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+16 more)

### Community 80 - "api/types.ts"
Cohesion: 0.10
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.15
Nodes (28): PdfRenderError, adresse_dict(), compile_paged(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict() (+20 more)

### Community 89 - "common.rs"
Cohesion: 0.19
Nodes (18): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_menge(), parse_preis(), parse_ratio(), parse_rechnung_id(), parse_uuid() (+10 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.15
Nodes (20): buchung_umsatz(), BuchungUmsatz, NaiveDate, Report, Self, Vec, SeminarBuchungAnlegenFehler, SeminarBuchungStornierenFehler (+12 more)

### Community 100 - "RepositoryError"
Cohesion: 0.23
Nodes (12): AtomicBool, Connection, RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, AsRef (+4 more)

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
Cohesion: 0.12
Nodes (8): Add, Preis, Output, Self, position_from_leistung(), RechnungIn<S>, Rechnungsposition, Mul

### Community 105 - "stores/index.tsx"
Cohesion: 0.05
Nodes (64): useAddresses(), queryClient, AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages (+56 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.36
Nodes (6): Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "SeminarTermin"
Cohesion: 0.16
Nodes (14): teilnahme_object_key(), Error, Option, Seminar, SeminarOrt, SeminarTermin, TimeDelta, SeminarBuchungAnlegen (+6 more)

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
Cohesion: 0.22
Nodes (10): DateTime, Display, Formatter, Self, Utc, utc(), Zeitraum, zeitraum_accepts_ende_after_beginn() (+2 more)

### Community 117 - "InMemoryObjectStore"
Cohesion: 0.24
Nodes (11): get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap, Mutex, ObjectStream, Option (+3 more)

### Community 118 - "notification.ts"
Cohesion: 0.12
Nodes (16): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), notification, NotificationActions, NotificationBehaviour (+8 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.22
Nodes (9): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Error, Mobilnummer, NaiveDate, Self (+1 more)

### Community 120 - "FakeObjectStore"
Cohesion: 0.19
Nodes (9): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStream, Option, ResultReport, Self (+1 more)

### Community 121 - "preis.rs"
Cohesion: 0.29
Nodes (8): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 122 - "domain/seminar_termin.rs"
Cohesion: 0.32
Nodes (17): absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient() (+9 more)

### Community 123 - "ObjectStore"
Cohesion: 0.23
Nodes (11): ObjectStore, Send, Sync, rechnung_key_uses_uuid(), rechnung_object_key(), rechnung_pdf_laden(), ObjectStream, Option (+3 more)

### Community 124 - "FakeUnitOfWork"
Cohesion: 0.28
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 125 - "seminar_from_row"
Cohesion: 0.27
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, seminar_from_row() (+1 more)

### Community 126 - "ExecutionContext"
Cohesion: 0.21
Nodes (7): ExecutionContext, Arc, PdfRenderer, Send, Sync, BehandlungErstellen, Behandlung

### Community 127 - "SQLiteConnection"
Cohesion: 0.24
Nodes (9): InstanceType, Arc, Deref, Mutex, Target, TempDir, SQLiteConnection, Database (+1 more)

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (5): Migration, Error, Option, Transaction, table_exists()

### Community 129 - "SQLiteKlientRepository"
Cohesion: 0.26
Nodes (9): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 130 - "YamsApiSpec"
Cohesion: 0.11
Nodes (23): Behandlung, BehandlungErstellung, Haustier, HaustierErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung (+15 more)

### Community 131 - "server/src/main.rs"
Cohesion: 0.19
Nodes (11): BackendServerError, catch_panic(), Config, log_unsuccessful_response_body(), main(), Error, Report, Response (+3 more)

### Community 134 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 135 - ".poll_next"
Cohesion: 0.22
Nodes (8): FileStream, Context, Item, Pin, Self, File, Poll, Stream

### Community 138 - "!.next"
Cohesion: 0.33
Nodes (3): Data, nextConfig, !.next

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 142 - "ports/object_store.rs"
Cohesion: 0.60
Nodes (4): collect_object(), once_stream(), ObjectStream, Vec

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "ClientItem.tsx"
Cohesion: 0.43
Nodes (6): AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, LiveRefresher

### Community 145 - "wal_connection_race.rs"
Cohesion: 0.70
Nodes (4): app(), neuer_klient(), parallel_execute_fn_survives_wal_connection_init(), Arc

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

- **Why does `String` connect `String` to `arc_up`, `YamsApiSpec`, `server/src/main.rs`, `Haustier`, `YAMSFrontendConfig`, `termin_from_parts`, `InternalServerError`, `Behandlung`, `domain/kontakt.rs`, `SQLiteRechnungRepository`, `document_text`, `LeistungId`, `tests.rs`, `LeistungOffen`, `UseCase`, `EmailAdresse`, `KlientId`, `NeueLeistung`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `KlientErstellen`, `leistung_from_row`, `Menge`, `base_app_builder`, `Seminar`, `HaustierErstellung`, `Produkt`, `Ratio`, `Klientbericht`, `StructuredError`, `schema/seminar.rs`, `SeminarId`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `Rechnung`, `Preis`, `.behandlung_erstellen`, `SeminarTermin`, `Klient`, `.produkt_erstellen`, `HaustierId`, `InMemoryObjectStore`, `KlientErstellung`, `FakeObjectStore`, `ObjectStore`, `FakeUnitOfWork`, `ExecutionContext`?**
  _High betweenness centrality (0.259) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.044) - this node is a cross-community bridge._
- **Why does `KlientId` connect `KlientId` to `SQLiteKlientRepository`, `LeistungIn<S>`, `termin_from_parts`, `Versioned`, `SQLiteRechnungRepository`, `FakeKlientenRepository`, `tests.rs`, `LeistungOffen`, `UseCase`, `SeminarBuchungId`, `RechnungId`, `NeueLeistung`, `PdfDokument`, `YamsAppApi`, `requests/seminar.rs`, `HaustierErstellung`, `Klientbericht`, `requests/abrechnung.rs`, `use_cases/seminar.rs`, `Preis`, `SeminarTermin`, `SQLiteHaustierRepository`, `HaustierId`, `domain/seminar_termin.rs`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `UnitOfWork<'a>`, `ProduktErstellenFehler` to the rest of the system?**
  _242 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._