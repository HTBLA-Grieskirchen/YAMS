# Graph Report - yams  (2026-09-07)

## Corpus Check
- 320 files · ~354,862 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3475 nodes · 8900 edges · 172 communities (134 shown, 29 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 239 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `462c1e88`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .add_dyn
- TestError
- devDependencies
- biome.json
- yams-api
- Klient
- src-tauri/src/config.rs
- EventForm.tsx
- String
- termin_from_parts
- yams-filesystemstore/src/lib.rs
- UpMigration
- compilerOptions
- seminar-overview.tsx
- Json
- Behandlung
- api/types.ts
- E
- provider.tsx
- domain/kontakt.rs
- SQLiteRechnungRepository
- Versioned<T>
- arc_up
- libs/database/index.ts
- tests.rs
- PdfRenderer
- makeRecordForTable
- leistung_from_row
- src-tauri/src/tracing_setup.rs
- arc_down
- ExecutionContext
- EmailAdresse
- use_cases/listen.rs
- KlientId
- Klient
- NeueLeistung
- PdfDokument
- ObjectStore
- ResultReport
- FixedClock
- .get_current_version
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- SeminarTerminId
- FakeDatastore
- NotificationType
- RepoStorage
- domain/seminar_termin.rs
- common.rs
- Klientbericht
- base_app_builder
- Ratio
- HaustierErstellung
- Produkt
- YamsApi
- AddressTable.tsx
- StructuredError
- repos.rs
- DatabaseConnection
- StreamBinaryResponse
- cases/seminar.rs
- schema/seminar.rs
- Leistung
- abrechnung-panel.tsx
- Integration Test Workflow
- frontend-legacy/api/schema.d.ts
- TauriYamsApi
- yams-core
- termin-detail-panel.tsx
- Seminar — Domain-Spezifikation
- Option
- base_app_builder
- BackendClient
- diffx-finish-review
- index.d.ts
- Menge
- Next.js
- postcss.config.mjs
- bad_request
- Built on SurrealDB
- YAMS Banner SVG
- YAMS Logo SVG
- yams-typstreports/src/lib.rs
- HaustierErstellen
- use_cases/seminar.rs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- yams-persistence
- SQLiteInstance
- base_app_builder
- server/src/config.rs
- hooks/index.ts
- instrumented.rs
- stores/index.tsx
- .behandlung_erstellen
- StatusCode
- Klient
- .produkt_erstellen
- cn
- cases/abrechnung.rs
- HaustierId
- navigation.ts
- HttpYamsApi
- KlientErstellen
- Versioned
- Notifications.tsx
- KlientErstellung
- Rechnung
- UnitOfWork
- v0004_leistungen_quelle_mwst.rs
- src/errors.rs
- SQLiteHaustierRepository
- seminar_from_row
- repository.rs
- ProduktRepository
- BehandlungCreateForm
- SQLiteKlientRepository
- YamsApiSpec
- main
- validation_error.rs
- .from_parts
- .haustier_erstellen
- in_memory_object_store.rs
- FakeObjectStore
- dependencies
- scripts
- InternalServerError
- TypicalJsonResponse
- rechnung-bezahlt-button.tsx
- Zeitraum
- .seminar_umsatz_prognose
- FakeUnitOfWork
- RechnungRepository
- Preis
- Haustier
- dialog.ts
- SeminarCreateForm
- FakeSeminareRepository
- FakeBehandlungenRepository
- UnitOfWorkImpl
- App
- Adresse
- AnimalAddItem.tsx
- SQLiteUnitOfWork
- package.json
- ProduktErstellung
- card.tsx
- InstrumentedUnitOfWork
- tailwindcss
- @types/react
- HaustierCreateForm
- ProduktCreateForm
- ports/object_store.rs
- SeminarForm
- LeistungRepository
- FakeProdukteRepository
- !.next
- RowActionsMenu
- .haustier_erstellen

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 88 edges
2. `KlientId` - 74 edges
3. `YamsAppApi` - 72 edges
4. `Preis` - 56 edges
5. `Ratio` - 56 edges
6. `useStore()` - 52 edges
7. `useYamsApiReady()` - 44 edges
8. `query()` - 42 edges
9. `FakeDatastore` - 41 edges
10. `YamsApi` - 41 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `ConfigError` --references--> `String`  [EXTRACTED]
  backend/server/src/config.rs → crates/yams-api/src/errors/internal_error.rs
- `ServerConfig` --references--> `String`  [EXTRACTED]
  backend/server/src/config.rs → crates/yams-api/src/errors/internal_error.rs
- `Cli` --references--> `String`  [EXTRACTED]
  backend/server/src/config.rs → crates/yams-api/src/errors/internal_error.rs
- `parse_config_file()` --references--> `String`  [EXTRACTED]
  backend/server/src/config.rs → crates/yams-api/src/errors/internal_error.rs

## Import Cycles
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **CI Release Pipeline** — github_workflows_publish_release_workflow, github_workflows_smoke_tests_workflow, github_workflows_unit_tests_workflow, github_workflows_tauri_action, github_workflows_mise_action [EXTRACTED 1.00]
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **Frontend Backend Communication Bridge** — specs_backend_hexagonal_backend_client, specs_backend_hexagonal_http_adapter, specs_backend_hexagonal_tauri_adapter, specs_backend_hexagonal_tanstack_query, specs_frontend_migration_yamsconfig [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (172 total, 29 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.21
Nodes (13): DownMigration, MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From (+5 more)

### Community 1 - "TestError"
Cohesion: 0.12
Nodes (15): AppliedLog, Cell, FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Box, Display, Formatter (+7 more)

### Community 2 - "devDependencies"
Cohesion: 0.12
Nodes (17): babel-plugin-react-compiler, @biomejs/biome, devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, @tailwindcss/postcss, @tauri-apps/cli (+9 more)

### Community 3 - "biome.json"
Cohesion: 0.07
Nodes (29): source, assist, actions, next, react, files, ignoreUnknown, includes (+21 more)

### Community 4 - "yams-api"
Cohesion: 0.06
Nodes (37): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+29 more)

### Community 5 - "Klient"
Cohesion: 0.07
Nodes (37): Energetik Logo (legacy), Backend ER Model, Adresse, Behandlung, Behandlungsart, BehandlungsTermin, Buchungsart, Energetik Sabine Petschl (+29 more)

### Community 6 - "src-tauri/src/config.rs"
Cohesion: 0.10
Nodes (44): ConfigError, default_config_path(), default_log_dir(), DeploymentMode, deserializes_embedded_file(), deserializes_log_dir_from_file(), deserializes_log_dir_from_toml_file(), deserializes_remote_file() (+36 more)

### Community 7 - "EventForm.tsx"
Cohesion: 0.08
Nodes (53): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, clientSearched(), AddClientForm (+45 more)

### Community 8 - "String"
Cohesion: 0.11
Nodes (66): YamsAppApi, From, String, alle_behandlungen(), alle_haustiere(), alle_klienten(), alle_leistungen(), alle_produkte() (+58 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.16
Nodes (27): format_datetime(), parse_datetime(), DateTime, Utc, insert_params(), leistung_id_for(), load_buchungen(), load_termin() (+19 more)

### Community 10 - "yams-filesystemstore/src/lib.rs"
Cohesion: 0.14
Nodes (26): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+18 more)

### Community 11 - "UpMigration"
Cohesion: 0.11
Nodes (13): Send, Sync, UpMigration, Migration, Option, Transaction, Migration, Option (+5 more)

### Community 12 - "compilerOptions"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "seminar-overview.tsx"
Cohesion: 0.08
Nodes (25): useAlleSeminareQuery(), $defs, LeistungQuelle_LeistungQuelleBehandlungTyp, Behandlung, LeistungQuelle_LeistungQuelleManuellTyp, Manuell, LeistungQuelle_LeistungQuelleProduktTyp, Produkt (+17 more)

### Community 14 - "Json"
Cohesion: 0.20
Nodes (6): Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, SeminarTerminErstellung, Json

### Community 15 - "Behandlung"
Cohesion: 0.09
Nodes (19): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+11 more)

### Community 16 - "api/types.ts"
Cohesion: 0.12
Nodes (16): ApiError, JsonClient, components, paths, Adresse, Behandlung, BehandlungErstellung, KlientErstellung (+8 more)

### Community 17 - "E"
Cohesion: 0.17
Nodes (19): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+11 more)

### Community 18 - "provider.tsx"
Cohesion: 0.19
Nodes (16): createYamsApi(), normalizeApiBaseUrl(), resetYamsApiCache(), YamsApiContext, YamsApiContextValue, YamsApiProvider(), bootstrap(), BaseConfig (+8 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (11): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+3 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.23
Nodes (15): geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate, Option (+7 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.12
Nodes (11): Clone, Deref, Option, Self, T, Target, UnitOfWork<'a>, Versioned<T> (+3 more)

### Community 22 - "arc_up"
Cohesion: 0.16
Nodes (16): arc_up(), Arc, add_migration_out_of_order_results_in_sorted_apply_order(), apply_from_mid_to_latest_applies_only_pending(), apply_from_mid_to_mid_applies_only_in_range(), apply_from_none_to_latest_applies_all_migrations(), apply_from_none_to_specific_target_applies_only_up_to_target(), apply_from_none_to_target_zero_leaves_none_and_applies_nothing() (+8 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.05
Nodes (21): CompatibilityResult, db, live(), LiveCleaner, LoadingResult, QueryResultState, TODO: Provide live support once sync is implemented, TODO: Remove once sync is implemented (+13 more)

### Community 24 - "tests.rs"
Cohesion: 0.17
Nodes (38): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), collect_frame_text(), document_text(), generate(), hof_adresse() (+30 more)

### Community 25 - "PdfRenderer"
Cohesion: 0.11
Nodes (13): DateTime, NaiveDate, Utc, SystemClock, ExecutionContext<'a>, RepositoryResult, Self, Clock (+5 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.07
Nodes (21): createSeminar(), ensureSeminar(), Address, AddressResponse, Client, ClientResponse, DatabaseObject, DatabaseResponse (+13 more)

### Community 27 - "leistung_from_row"
Cohesion: 0.18
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 29 - "arc_down"
Cohesion: 0.31
Nodes (9): arc_down(), apply_down_from_current_to_lower_target_applies_down_migrations(), apply_down_from_current_to_mid_target_applies_only_relevant_down(), apply_down_from_one_to_none(), apply_down_one_step_from_2_to_1(), down_registry_apply_down_failure_propagates_as_migration_failed(), down_registry_apply_up_to_latest_applies_all_migrations(), down_registry_apply_when_current_equals_target_is_no_op() (+1 more)

### Community 30 - "ExecutionContext"
Cohesion: 0.09
Nodes (32): ExecutionContext, ExecutionSource, Arc, LeistungOffen, UseCase, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen (+24 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.23
Nodes (8): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Example, From, Self, TryFrom

### Community 32 - "use_cases/listen.rs"
Cohesion: 0.16
Nodes (14): AlleBehandlungenAuflisten, AlleHaustiereAuflisten, AlleKlientenAuflisten, AlleLeistungenAuflisten, AlleProdukteAuflisten, AlleRechnungenAuflisten, AlleSeminareAuflisten, AlleSeminarTermineAuflisten (+6 more)

### Community 33 - "KlientId"
Cohesion: 0.12
Nodes (31): KlientId, aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen (+23 more)

### Community 34 - "Klient"
Cohesion: 0.09
Nodes (21): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, neu(), NeuerKlient, Adresse (+13 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.09
Nodes (20): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, LeistungIn<S> (+12 more)

### Community 36 - "PdfDokument"
Cohesion: 0.11
Nodes (18): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, Vec, PdfDokument, PdfRenderError (+10 more)

### Community 37 - "ObjectStore"
Cohesion: 0.15
Nodes (26): ObjectStore, Send, Sync, klient_bericht(), mit_objekt_rollback(), nach_pdf_persistieren(), objekt_löschen_best_effort(), pdfs_rendern_und_ablegen() (+18 more)

### Community 38 - "ResultReport"
Cohesion: 0.08
Nodes (29): AuflistenFehler, Haustier, ObjectStoreError, ObjectStream, Rechnung, RechnungBezahltMarkieren, RepositoryError, ResultReport (+21 more)

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - ".get_current_version"
Cohesion: 0.21
Nodes (9): Box, Future, Option, Output, Pin, Send, Transaction, SQLiteConnection (+1 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.16
Nodes (21): abgehalten_use_case(), buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Option, Report (+13 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.24
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (15): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungQuelleSeminar, LeistungStatus, Decimal (+7 more)

### Community 44 - "SeminarTerminId"
Cohesion: 0.10
Nodes (15): seminar_betrag_full_rabatt_is_zero(), seminar_betrag_uses_nach_rabatt(), Item, Iterator, S, Seminar, Uuid, Vec (+7 more)

### Community 45 - "FakeDatastore"
Cohesion: 0.14
Nodes (12): FakeDatastore, FakeLeistungenRepository, Arc, Clone, Default, FxHashMap, Leistung, Mutex (+4 more)

### Community 46 - "NotificationType"
Cohesion: 0.14
Nodes (15): ErrorReportExt, Result<T, E>, C, Report, Send, Sync, T, ThreadSafeError (+7 more)

### Community 47 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedBehandlungRepository, InstrumentedHaustierRepository, InstrumentedKlientRepository, InstrumentedLeistungRepository (+4 more)

### Community 48 - "domain/seminar_termin.rs"
Cohesion: 0.30
Nodes (18): absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient() (+10 more)

### Community 50 - "common.rs"
Cohesion: 0.23
Nodes (18): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis(), parse_ratio(), parse_rechnung_id() (+10 more)

### Community 51 - "Klientbericht"
Cohesion: 0.17
Nodes (15): Klientbericht, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse, NaiveDate (+7 more)

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (44): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+36 more)

### Community 53 - "Ratio"
Cohesion: 0.10
Nodes (17): Ratio, RatioFehler, Decimal, Self, NeuesSeminar, preis(), Into, Option (+9 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.36
Nodes (6): HaustierErstellen, HaustierErstellung, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.09
Nodes (19): NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport, Self (+11 more)

### Community 57 - "AddressTable.tsx"
Cohesion: 0.09
Nodes (22): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useAddresses(), useBackend(), AddressEditTableRowContent (+14 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "repos.rs"
Cohesion: 0.09
Nodes (10): BehandlungRepository, HaustierRepository, KlientRepository, RepositoryError, Option, Send, Sync, SeminarRepository (+2 more)

### Community 60 - "DatabaseConnection"
Cohesion: 0.14
Nodes (7): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, DatabaseConnection, DatabaseError

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.11
Nodes (21): openapi_service(), C, From, Into, Item, ObjectStream, Report, Self (+13 more)

### Community 62 - "cases/seminar.rs"
Cohesion: 0.21
Nodes (28): abgehalten_maps_every_confirmed_buchung(), abgehalten_schreibt_teilnahme_pdf_für_bestätigte_buchungen(), absage_blockiert_abgehalten(), adresse(), aktualisieren_blockiert_nach_abgehalten(), aktualisieren_nur_geplant(), assert_offen_mwst(), kapazität_und_doppelbuchung() (+20 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.13
Nodes (27): NaiveDate, SeminarUmsatzPrognose, BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, From (+19 more)

### Community 64 - "Leistung"
Cohesion: 0.21
Nodes (4): Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung

### Community 65 - "abrechnung-panel.tsx"
Cohesion: 0.07
Nodes (50): useAlleKlientenQuery(), useAlleLeistungenQuery(), useAlleRechnungenQuery(), useRechnungenFürKlientQuery(), RechnungStatus, Bezahlt, Offen, KlientDetailContent() (+42 more)

### Community 66 - "Integration Test Workflow"
Cohesion: 0.53
Nodes (6): mise, mise-action, Release Workflow, Integration Test Workflow, tauri-action, Unit Test Workflow

### Community 67 - "frontend-legacy/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "TauriYamsApi"
Cohesion: 0.10
Nodes (8): invokeCommand(), TauriYamsApi, Rechnung, RechnungBezahltMarkieren, Seminar, SeminarErstellung, SeminarUmsatzVorschau, TagesabschlussErstellung

### Community 69 - "yams-core"
Cohesion: 0.44
Nodes (9): molting, yams-api, yams-core, yams-fakes, yams-filesystemstore, yams-persistence, yams-server, yams-tauri (+1 more)

### Community 70 - "termin-detail-panel.tsx"
Cohesion: 0.10
Nodes (37): useSeminarTerminPlanenMutation(), HaustierCreateFormProps, LeistungBuchenFormProps, LeistungQuelle, TerminBuchungForm(), TerminBuchungFormProps, TerminDetailPanelProps, TerminPlanForm() (+29 more)

### Community 71 - "Seminar — Domain-Spezifikation"
Cohesion: 0.10
Nodes (18): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+10 more)

### Community 72 - "Option"
Cohesion: 0.32
Nodes (4): NeuerSeminarTermin, Adresse, Option, SeminarOrt

### Community 73 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 74 - "BackendClient"
Cohesion: 0.50
Nodes (4): BackendClient, MobX, TanStack Query, yamsconfig.json

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 77 - "Menge"
Cohesion: 0.10
Nodes (30): BehandlungErstellen, BehandlungErstellung, into_rechnung_als_bezahlt_markieren(), LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung (+22 more)

### Community 80 - "bad_request"
Cohesion: 0.11
Nodes (14): bad_request(), Arc, C, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Report (+6 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.19
Nodes (22): adresse_dict(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict(), preis(), ratio() (+14 more)

### Community 89 - "HaustierErstellen"
Cohesion: 0.23
Nodes (10): HaustierErstellen, HaustierErstellenFehler, Context, Haustier, NaiveDate, Report, ResultReport, Vec (+2 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.12
Nodes (24): buchung_umsatz(), BuchungUmsatz, NaiveDate, Option, Report, Self, Seminar, TimeDelta (+16 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, InstanceType, Arc, AsRef, Connection, Deref, Mutex, Path (+9 more)

### Community 101 - "base_app_builder"
Cohesion: 0.16
Nodes (12): base_app_builder(), AppBuilder, SetUowProvider, test_tagesabschluss_empty_day_returns_no_rechnungen(), test_haustier(), mwst_19(), test_behandlung_erstellen(), test_behandlung_erstellen_rejects_empty_name() (+4 more)

### Community 102 - "server/src/config.rs"
Cohesion: 0.16
Nodes (24): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+16 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.08
Nodes (44): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useSeminarBuchungAnlegenMutation(), useSeminarBuchungStornierenMutation() (+36 more)

### Community 104 - "instrumented.rs"
Cohesion: 0.23
Nodes (9): InstrumentedObjectStore, InstrumentedPdfRenderer, pdf_dokument_kind(), Arc, ObjectStoreError, ObjectStream, Option, Pin (+1 more)

### Community 105 - "stores/index.tsx"
Cohesion: 0.05
Nodes (54): queryClient, EventsUsages, AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, EventDetailItem (+46 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.31
Nodes (7): BehandlungErstellenFehler, Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "StatusCode"
Cohesion: 0.06
Nodes (36): AuflistenFehler, BehandlungErstellenFehler, HaustierErstellenFehler, HttpStatusMapping, KlientErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler, LeistungManuellErfassenFehler (+28 more)

### Community 108 - "Klient"
Cohesion: 0.19
Nodes (12): Klient, KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate (+4 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.28
Nodes (7): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain(), ProduktErstellenFehler

### Community 111 - "cn"
Cohesion: 0.10
Nodes (17): useKlientErstellenMutation(), Klient, TabButton(), defaultKlient(), KlientRegisterForm(), KlientRegisterFormProps, Checkbox(), CheckboxProps (+9 more)

### Community 112 - "cases/abrechnung.rs"
Cohesion: 0.25
Nodes (16): AbrechnungSetup, app_with_pdf_fakes(), menge(), mwst_19(), rechnung_kann_als_bezahlt_markiert_werden(), Arc, Klient, NaiveDate (+8 more)

### Community 113 - "HaustierId"
Cohesion: 0.15
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+5 more)

### Community 114 - "navigation.ts"
Cohesion: 0.09
Nodes (27): geistMono, geistSans, metadata, Providers(), AppChrome(), AppShell(), PageChromeContext, PageChromeContextValue (+19 more)

### Community 116 - "KlientErstellen"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Klient, Mobilnummer, NaiveDate

### Community 117 - "Versioned"
Cohesion: 0.22
Nodes (8): Versioned, FakeHaustiereRepository, FakeKlientenRepository, FakeRechnungenRepository, Haustier, Klient, Rechnung, RepositoryResult

### Community 118 - "Notifications.tsx"
Cohesion: 0.15
Nodes (11): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationBehaviour, NotificationInfo, NotificationInfoType (+3 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Self, TryFrom

### Community 120 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 121 - "UnitOfWork"
Cohesion: 0.20
Nodes (10): Box, C, F, Formatter, Future, Output, Report, RepositoryResult (+2 more)

### Community 122 - "v0004_leistungen_quelle_mwst.rs"
Cohesion: 0.22
Nodes (19): adds_quelle_mwst_when_neither_column_exists(), align_column(), already_aligned_schema_is_noop(), apply(), column_exists(), columns(), converts_percentage_mwst_to_ratio(), idempotent_when_already_ratio() (+11 more)

### Community 123 - "src/errors.rs"
Cohesion: 1.00
Nodes (3): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), RepositoryError

### Community 124 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 125 - "seminar_from_row"
Cohesion: 0.25
Nodes (10): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, Vec (+2 more)

### Community 126 - "repository.rs"
Cohesion: 0.29
Nodes (4): FakeSeminarTermineRepository, NaiveDate, SeminarTermin, Vec

### Community 128 - "BehandlungCreateForm"
Cohesion: 1.00
Nodes (3): BehandlungCreateForm(), handleSubmit(), emptyBehandlung()

### Community 129 - "SQLiteKlientRepository"
Cohesion: 0.24
Nodes (10): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 130 - "YamsApiSpec"
Cohesion: 0.21
Nodes (9): Path, RechnungBezahltMarkieren, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarUmsatzVorschau, Uuid (+1 more)

### Community 131 - "main"
Cohesion: 0.24
Nodes (9): BackendServerError, catch_panic(), main(), Report, init_tracing(), Option, Path, CatchPanic (+1 more)

### Community 133 - ".from_parts"
Cohesion: 0.12
Nodes (17): Abgehalten, Abgesagt, DateTime, From, FxHashMap, Into, ResultReport, Self (+9 more)

### Community 134 - ".haustier_erstellen"
Cohesion: 0.36
Nodes (6): HaustierErstellung, Haustier, NaiveDate, Uuid, schema_haustier_from_domain(), HaustierErstellenFehler

### Community 135 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 136 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 137 - "dependencies"
Cohesion: 0.12
Nodes (17): dependencies, lucide-react, next, openapi-fetch, react, react-dom, @tanstack/react-query, @tanstack/react-query-devtools (+9 more)

### Community 138 - "scripts"
Cohesion: 0.20
Nodes (10): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+2 more)

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 140 - "TypicalJsonResponse"
Cohesion: 0.13
Nodes (12): Behandlung, BehandlungErstellung, Klient, KlientErstellung, Produkt, ProduktErstellung, Rechnung, Seminar (+4 more)

### Community 141 - "rechnung-bezahlt-button.tsx"
Cohesion: 0.28
Nodes (6): useRechnungAlsBezahltMarkierenMutation(), getYamsApi(), RechnungBezahltButton(), RechnungBezahltButtonProps, TagesabschlussForm(), todayIsoDate()

### Community 142 - "Zeitraum"
Cohesion: 0.22
Nodes (10): DateTime, Display, Formatter, Self, Utc, utc(), Zeitraum, zeitraum_accepts_ende_after_beginn() (+2 more)

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "FakeUnitOfWork"
Cohesion: 0.30
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 146 - "Preis"
Cohesion: 0.07
Nodes (18): Add, LeistungId, LeistungQuelle, nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), Preis, preis_add_sums_values() (+10 more)

### Community 148 - "dialog.ts"
Cohesion: 0.20
Nodes (6): DialogComponent, DialogInfo, DialogInfoType, DialogType, TODO: Add possibility to also display native dialog on host system if in Tauri, DialogStore

### Community 149 - "SeminarCreateForm"
Cohesion: 1.00
Nodes (3): emptySeminar(), SeminarCreateForm(), handleSubmit()

### Community 152 - "UnitOfWorkImpl"
Cohesion: 0.17
Nodes (12): main(), Box, RepositoryResult, Unimplemented, LockedUnitOfWorkImpl, Send, Sync, UnitOfWorkImpl (+4 more)

### Community 153 - "App"
Cohesion: 0.21
Nodes (10): App, Arc, Box, Context, F, ResultReport, app_with_pdf_fakes(), Arc (+2 more)

### Community 154 - "Adresse"
Cohesion: 0.29
Nodes (7): Adresse, domain::Adresse, Ländercode, Example, From, Self, TryFrom

### Community 155 - "AnimalAddItem.tsx"
Cohesion: 0.11
Nodes (12): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAnimal(), patchAnimal(), deleteRace(), patchRace(), Animal (+4 more)

### Community 156 - "SQLiteUnitOfWork"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 157 - "package.json"
Cohesion: 0.50
Nodes (3): name, private, version

### Community 159 - "card.tsx"
Cohesion: 0.14
Nodes (17): Card(), CardContent(), CardDescription(), CardHeader(), CardTitle(), defaultHaustier(), HaustierForm(), HaustierFormProps (+9 more)

### Community 160 - "InstrumentedUnitOfWork"
Cohesion: 0.31
Nodes (4): InstrumentedUnitOfWork, Box, RepositoryResult, Self

### Community 163 - "HaustierCreateForm"
Cohesion: 0.67
Nodes (3): emptyHaustier(), HaustierCreateForm(), handleSubmit()

### Community 164 - "ProduktCreateForm"
Cohesion: 1.00
Nodes (3): emptyProdukt(), ProduktCreateForm(), handleSubmit()

### Community 165 - "ports/object_store.rs"
Cohesion: 0.53
Nodes (5): collect_object(), ObjectStoreError, once_stream(), ObjectStream, Vec

### Community 172 - "!.next"
Cohesion: 0.33
Nodes (3): Data, nextConfig, !.next

## Knowledge Gaps
- **301 isolated node(s):** `molting`, `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler` (+296 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 868 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **29 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `TestError`, `.from_parts`, `.haustier_erstellen`, `in_memory_object_store.rs`, `FakeObjectStore`, `termin_from_parts`, `src-tauri/src/config.rs`, `InternalServerError`, `Json`, `Behandlung`, `FakeUnitOfWork`, `Preis`, `domain/kontakt.rs`, `SQLiteRechnungRepository`, `tests.rs`, `Adresse`, `leistung_from_row`, `ExecutionContext`, `EmailAdresse`, `KlientId`, `Klient`, `NeueLeistung`, `ObjectStore`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `common.rs`, `Klientbericht`, `base_app_builder`, `Ratio`, `HaustierErstellung`, `Produkt`, `StructuredError`, `schema/seminar.rs`, `Option`, `Menge`, `HaustierErstellen`, `use_cases/seminar.rs`, `server/src/config.rs`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `HaustierId`, `KlientErstellen`, `KlientErstellung`, `Rechnung`, `v0004_leistungen_quelle_mwst.rs`?**
  _High betweenness centrality (0.549) - this node is a cross-community bridge._
- **Why does `TestError` connect `TestError` to `String`, `NotificationType`, `arc_down`, `arc_up`?**
  _High betweenness centrality (0.380) - this node is a cross-community bridge._
- **Why does `Error` connect `NotificationType` to `TestError`?**
  _High betweenness centrality (0.378) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `ValidationError`, `RechnungFehler` to the rest of the system?**
  _301 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `TestError` be split into smaller, more focused modules?**
  _Cohesion score 0.11895161290322581 - nodes in this community are weakly interconnected._
- **Should `devDependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.11764705882352941 - nodes in this community are weakly interconnected._