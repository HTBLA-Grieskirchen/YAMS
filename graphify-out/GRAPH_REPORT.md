# Graph Report - yams  (2026-09-03)

## Corpus Check
- 269 files · ~331,800 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2844 nodes · 6952 edges · 142 communities (128 shown, 14 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 224 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `54cf7fc6`
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
- yams-filesystemstore/src/lib.rs
- UpMigration
- compilerOptions
- relations/index.tsx
- Json
- Behandlung
- bad_request
- apply_up_migrations
- E
- domain/kontakt.rs
- parse_position_from_row
- Versioned<T>
- LeistungId
- libs/database/index.ts
- tests.rs
- LeistungOffen
- makeRecordForTable
- dialog.ts
- HaustierErstellen
- BehandlungId
- SeminarTerminId
- EmailAdresse
- api/client.ts
- RechnungId
- KlientId
- NeueLeistung
- PdfDokument
- produkt_from_row
- YamsAppApi
- FixedClock
- .get_current_version
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- openapi_service
- FakeDatastore
- .contextualize_with
- KlientErstellen
- participation/index.tsx
- leistung_from_row
- Menge
- base_app_builder
- SeminarId
- HaustierErstellung
- Produkt
- Ratio
- service/pdf.rs
- StructuredError
- SQLiteUnitOfWork
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
- SQLiteInstance
- Clock
- .begin
- hooks/index.ts
- Rechnungsposition
- stores/index.tsx
- .behandlung_erstellen
- HttpStatusMapping
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
- .rendern
- Preis
- domain/seminar_termin.rs
- migration_error_to_persistence_error
- FakeUnitOfWork
- seminar_from_row
- ObjectStore
- .seminar_umsatz_prognose
- Migration
- Versioned
- YamsApiSpec
- main
- validation_error.rs
- Haustier
- !.next
- InternalServerError
- ports/object_store.rs
- .seminar_umsatz_prognose
- query
- wal_connection_race.rs
- document_text
- layout.tsx

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
10. `LeistungId` - 33 edges

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

## Communities (142 total, 14 thin omitted)

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

### Community 10 - "yams-filesystemstore/src/lib.rs"
Cohesion: 0.14
Nodes (23): FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing(), put_get_roundtrip(), rejects_parent_segment(), Arc (+15 more)

### Community 11 - "UpMigration"
Cohesion: 0.14
Nodes (11): Send, Sync, UpMigration, Migration, Error, Option, Transaction, Migration (+3 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "relations/index.tsx"
Cohesion: 0.18
Nodes (11): clientSearched(), deleteClientRelation(), relateClients(), updateClientRelation(), ClientRelationResponse, ClientDetail, AddRelationDialog, ClientRelations (+3 more)

### Community 14 - "Json"
Cohesion: 0.12
Nodes (11): Behandlung, BehandlungErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung (+3 more)

### Community 15 - "Behandlung"
Cohesion: 0.12
Nodes (8): Behandlung, BehandlungFehler, NeueBehandlung, preis(), Into, ResultReport, Self, Uuid

### Community 16 - "bad_request"
Cohesion: 0.14
Nodes (12): bad_request(), Arc, C, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Report (+4 more)

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
Cohesion: 0.11
Nodes (13): Clone, Deref, Formatter, Option, Self, T, Target, UnitOfWork<'a> (+5 more)

### Community 22 - "LeistungId"
Cohesion: 0.16
Nodes (13): LeistungId, Abgehalten, Abgesagt, DateTime, From, FxHashMap, Into, Utc (+5 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.05
Nodes (22): CompatibilityResult, DatabaseError, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented (+14 more)

### Community 24 - "tests.rs"
Cohesion: 0.21
Nodes (33): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+25 more)

### Community 25 - "LeistungOffen"
Cohesion: 0.19
Nodes (14): LeistungOffen, BehandlungErstellen, LeistungAusBehandlungBuchen, LeistungAusProduktBuchen, LeistungManuellErfassen, ProduktErstellen, Behandlung, Error (+6 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.07
Nodes (17): createSeminar(), ensureSeminar(), Animal, AnimalResponse, Client, DatabaseResponse, isRecord(), makeRecord() (+9 more)

### Community 27 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 28 - "HaustierErstellen"
Cohesion: 0.23
Nodes (10): HaustierErstellen, Context, Error, Haustier, NaiveDate, Report, ResultReport, Vec (+2 more)

### Community 29 - "BehandlungId"
Cohesion: 0.27
Nodes (10): BehandlungId, behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row (+2 more)

### Community 30 - "SeminarTerminId"
Cohesion: 0.10
Nodes (15): seminar_betrag_full_rabatt_is_zero(), seminar_betrag_uses_nach_rabatt(), Item, Iterator, S, Seminar, Uuid, Vec (+7 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.22
Nodes (9): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Self (+1 more)

### Community 32 - "api/client.ts"
Cohesion: 0.17
Nodes (8): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), Addresses

### Community 33 - "RechnungId"
Cohesion: 0.09
Nodes (42): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+34 more)

### Community 34 - "KlientId"
Cohesion: 0.11
Nodes (18): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, KlientId, neu(), NeuerKlient (+10 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.09
Nodes (21): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, LeistungIn<S> (+13 more)

### Community 36 - "PdfDokument"
Cohesion: 0.12
Nodes (17): Klientbericht, PdfDokument, Rechnungsbericht, Adresse, DateTime, EmailAdresse, NaiveDate, Option (+9 more)

### Community 37 - "produkt_from_row"
Cohesion: 0.27
Nodes (9): produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row, Transaction (+1 more)

### Community 38 - "YamsAppApi"
Cohesion: 0.13
Nodes (19): Haustier, HaustierErstellung, ObjectStoreError, ObjectStream, Rechnung, RepositoryError, ResultReport, Self (+11 more)

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - ".get_current_version"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.15
Nodes (22): abgehalten_use_case(), buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Error, Option (+14 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.24
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (15): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungQuelleSeminar, LeistungStatus, Decimal (+7 more)

### Community 44 - "openapi_service"
Cohesion: 0.20
Nodes (8): openapi_service(), Into, Item, Self, IntoIterator, MetaSchemaRef, OpenApiService, ServerObject

### Community 45 - "FakeDatastore"
Cohesion: 0.08
Nodes (27): FakeBehandlungenRepository, FakeDatastore, FakeHaustiereRepository, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository, FakeSeminareRepository, FakeSeminarTermineRepository (+19 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.21
Nodes (10): ErrorReportExt, Result<T, E>, C, Error, Report, Send, Sync, T (+2 more)

### Community 47 - "KlientErstellen"
Cohesion: 0.18
Nodes (10): Klient, KlientErstellung, KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient (+2 more)

### Community 48 - "participation/index.tsx"
Cohesion: 0.30
Nodes (10): ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation(), relateClientParticipateEvent() (+2 more)

### Community 50 - "leistung_from_row"
Cohesion: 0.17
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 51 - "Menge"
Cohesion: 0.24
Nodes (5): Rechnungspositionsbericht, Menge, MengeFehler, Decimal, Self

### Community 52 - "base_app_builder"
Cohesion: 0.08
Nodes (40): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), klient_body(), Value (+32 more)

### Community 53 - "SeminarId"
Cohesion: 0.12
Nodes (13): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+5 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.31
Nodes (7): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.13
Nodes (9): NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport, Self (+1 more)

### Community 56 - "Ratio"
Cohesion: 0.27
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 57 - "service/pdf.rs"
Cohesion: 0.16
Nodes (19): PraxisAngaben, klient_bericht(), rechnung_object_key(), rechnung_pdf_laden(), rechnungsdokument(), Klient, ObjectStoreError, ObjectStream (+11 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "SQLiteUnitOfWork"
Cohesion: 0.05
Nodes (34): ExecutionSource, BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository, ProduktRepository, RechnungRepository, RepositoryError (+26 more)

### Community 60 - "DatabaseConnection"
Cohesion: 0.16
Nodes (6): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.21
Nodes (14): C, From, ObjectStream, Report, StatusCode, T, status_from_report(), StreamBinaryResponse (+6 more)

### Community 62 - "base_app_builder"
Cohesion: 0.05
Nodes (71): App, Arc, Box, Context, ResultReport, base_app_builder(), AppBuilder, SetUowProvider (+63 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.15
Nodes (25): BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, Error, From, NaiveDate (+17 more)

### Community 64 - "Option"
Cohesion: 0.23
Nodes (5): NeuerSeminarTermin, Adresse, Option, Self, SeminarOrt

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
Cohesion: 0.16
Nodes (25): BehandlungErstellen, BehandlungErstellung, LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung, LeistungManuellErfassen (+17 more)

### Community 80 - "api/types.ts"
Cohesion: 0.10
Nodes (21): ApiError, JsonClient, invokeCommand(), TauriYamsApi, Adresse, Behandlung, BehandlungErstellung, Haustier (+13 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.15
Nodes (28): PdfRenderError, adresse_dict(), compile_paged(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict() (+20 more)

### Community 89 - "common.rs"
Cohesion: 0.21
Nodes (18): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis(), parse_ratio(), parse_rechnung_id() (+10 more)

### Community 92 - "ExecutionContext"
Cohesion: 0.11
Nodes (33): ExecutionContext, SeminarTerminGeplant, UseCase, buchung_umsatz(), BuchungUmsatz, Error, NaiveDate, Option (+25 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "Clock"
Cohesion: 0.22
Nodes (7): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync

### Community 102 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 103 - "hooks/index.ts"
Cohesion: 0.17
Nodes (20): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useTagesabschlussDurchführenMutation() (+12 more)

### Community 104 - "Rechnungsposition"
Cohesion: 0.15
Nodes (3): position_from_leistung(), RechnungIn<S>, Rechnungsposition

### Community 105 - "stores/index.tsx"
Cohesion: 0.05
Nodes (64): queryClient, AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton (+56 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.31
Nodes (7): Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain(), BehandlungErstellenFehler

### Community 107 - "HttpStatusMapping"
Cohesion: 0.31
Nodes (6): HttpStatusMapping, ObjectStoreError, RepositoryError, Option, StatusCode, ValidationError

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.28
Nodes (7): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain(), ProduktErstellenFehler

### Community 111 - "page.tsx"
Cohesion: 0.12
Nodes (11): deriveCurrentStep(), Home(), defaultHaustier(), HaustierForm(), defaultKlient(), KlientForm(), RechnungenPanel(), TagesabschlussForm() (+3 more)

### Community 112 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 113 - "HaustierId"
Cohesion: 0.14
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+5 more)

### Community 114 - "api/index.ts"
Cohesion: 0.23
Nodes (14): createYamsApi(), envDeploymentMode(), envRemoteApiUrl(), getYamsApi(), normalizeApiBaseUrl(), resolveDeploymentMode(), resolveRemoteApiBaseUrl(), tauriFrontendConfig() (+6 more)

### Community 116 - "Zeitraum"
Cohesion: 0.22
Nodes (10): DateTime, Display, Formatter, Self, Utc, utc(), Zeitraum, zeitraum_accepts_ende_after_beginn() (+2 more)

### Community 117 - "InMemoryObjectStore"
Cohesion: 0.23
Nodes (12): get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap, Mutex, ObjectStoreError, ObjectStream (+4 more)

### Community 118 - "notification.ts"
Cohesion: 0.13
Nodes (15): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+7 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.22
Nodes (9): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Error, Mobilnummer, NaiveDate, Self (+1 more)

### Community 120 - ".rendern"
Cohesion: 0.36
Nodes (5): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec

### Community 121 - "Preis"
Cohesion: 0.16
Nodes (13): Add, nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), Preis, preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value() (+5 more)

### Community 122 - "domain/seminar_termin.rs"
Cohesion: 0.23
Nodes (20): absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient() (+12 more)

### Community 123 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 124 - "FakeUnitOfWork"
Cohesion: 0.30
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 125 - "seminar_from_row"
Cohesion: 0.27
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, seminar_from_row() (+1 more)

### Community 126 - "ObjectStore"
Cohesion: 0.16
Nodes (10): ExecutionContext<'a>, Arc, RepositoryResult, Self, ObjectStore, Send, Sync, PdfRenderer (+2 more)

### Community 127 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, SeminarUmsatzPrognoseBisDatumFehler

### Community 128 - "Migration"
Cohesion: 0.36
Nodes (5): Migration, Error, Option, Transaction, table_exists()

### Community 129 - "Versioned"
Cohesion: 0.20
Nodes (12): Versioned, FakeKlientenRepository, Klient, klient_from_row(), Arc, Klient, Mutex, Option (+4 more)

### Community 130 - "YamsApiSpec"
Cohesion: 0.19
Nodes (12): Haustier, HaustierErstellung, Path, Rechnung, Seminar, SeminarTermin, SeminarUmsatzVorschau, TagesabschlussErstellung (+4 more)

### Community 131 - "main"
Cohesion: 0.24
Nodes (8): BackendServerError, catch_panic(), Config, main(), Report, init_tracing(), CatchPanic, PanicHandler

### Community 134 - "Haustier"
Cohesion: 0.70
Nodes (4): Haustier, NaiveDate, Uuid, schema_haustier_from_domain()

### Community 138 - "!.next"
Cohesion: 0.33
Nodes (3): Data, nextConfig, !.next

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 142 - "ports/object_store.rs"
Cohesion: 0.53
Nodes (5): collect_object(), ObjectStoreError, once_stream(), ObjectStream, Vec

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "query"
Cohesion: 0.20
Nodes (15): AnimalAddItem, AnimalComboBox, AnimalList, AnimalRow, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow (+7 more)

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
- **228 isolated node(s):** `molting`, `ObjectStoreError`, `ValidationError`, `VieleHaustiereErstellenFehler`, `yams-fakes` (+223 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **14 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `main`, `Haustier`, `YAMSFrontendConfig`, `termin_from_parts`, `InternalServerError`, `Json`, `Behandlung`, `domain/kontakt.rs`, `parse_position_from_row`, `document_text`, `LeistungId`, `tests.rs`, `LeistungOffen`, `HaustierErstellen`, `EmailAdresse`, `RechnungId`, `KlientId`, `NeueLeistung`, `PdfDokument`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `KlientErstellen`, `leistung_from_row`, `Menge`, `base_app_builder`, `SeminarId`, `HaustierErstellung`, `Produkt`, `service/pdf.rs`, `StructuredError`, `base_app_builder`, `schema/seminar.rs`, `Option`, `Adresse`, `requests/abrechnung.rs`, `common.rs`, `ExecutionContext`, `Rechnungsposition`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `HaustierId`, `InMemoryObjectStore`, `KlientErstellung`, `FakeUnitOfWork`?**
  _High betweenness centrality (0.245) - this node is a cross-community bridge._
- **Why does `App` connect `base_app_builder` to `YamsApiSpec`, `Clock`, `YamsAppApi`, `openapi_service`, `bad_request`, `base_app_builder`, `SQLiteUnitOfWork`, `ObjectStore`?**
  _High betweenness centrality (0.038) - this node is a cross-community bridge._
- **Why does `TestError` connect `arc_up` to `String`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `ObjectStoreError`, `ValidationError` to the rest of the system?**
  _228 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.041666666666666664 - nodes in this community are weakly interconnected._