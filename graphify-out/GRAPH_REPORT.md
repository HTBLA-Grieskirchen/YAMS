# Graph Report - yams  (2026-08-23)

## Corpus Check
- 208 files · ~296,870 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1358 nodes · 2887 edges · 100 communities (77 shown, 23 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 71 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- AppliableMigration
- AppliedLog
- babel plugin react compiler
- enabled
- Hexagonal Architecture
- Energetik Logo legacy
- config rs
- RegisterAddress tsx
- spec rs
- animal ts
- Connection
- clientRelation ts
- tsconfig json
- index tsx
- repository rs
- queryClient ts
- index tsx 1
- index ts
- EventResponse
- address rs
- Versioned
- Versioned T
- AddressTable tsx
- AddressResponse
- export spec rs
- App
- index tsx 2
- repos rs
- UseCase
- uow rs
- ClientParticipation tsx
- contact rs
- client ts
- Notifications tsx
- Animal
- Animal 1
- client rs
- LockedUnitOfWorkImpl
- index tsx 3
- clock rs
- SQLiteConnection
- Arc
- system clock rs
- FnOp F
- uow rs 1
- EventForm tsx
- ClientResponse
- EventParticipationResponse
- ClientRelationResponse
- NotificationInfo
- address rs 1
- client rs 1
- client rs 2
- client rs 3
- DialogInfo
- AnimalStore
- client rs 4
- structured error rs
- EventParticipants tsx
- DatabaseConnection
- animal rs
- business conform rs
- AddressStore
- ResultReport
- autoStore ts
- mise
- schema d ts
- schema d ts 1
- molting
- main rs
- internal error rs
- context rs
- business conform rs 1
- BackendClient
- diffx finish review
- index d ts
- validation error rs
- Next js
- postcss config mjs
- Built on SurrealDB
- YAMS Banner SVG
- YAMS Logo SVG
- File Icon
- Globe Icon
- Next js Logo
- Vercel Logo
- Window Icon
- yams persistence

## God Nodes (most connected - your core abstractions)
1. `useStore()` - 52 edges
2. `query()` - 42 edges
3. `E` - 31 edges
4. `makeRecordForTable()` - 29 edges
5. `Record` - 28 edges
6. `Client` - 26 edges
7. `live()` - 24 edges
8. `Event` - 24 edges
9. `Versioned` - 23 edges
10. `paths` - 22 edges

## Surprising Connections (you probably didn't know these)
- `Config` --references--> `String`  [EXTRACTED]
  backend/server/src/main.rs → crates/yams-api/src/errors/internal_error.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `create_animal()` --references--> `AnimalCreation`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/api/requests/animal.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  crates/yams-api/src/bin/export_spec.rs → crates/yams-api/src/spec.rs
- `create_animal()` --references--> `String`  [EXTRACTED]
  frontend/src-tauri/src/commands.rs → crates/yams-api/src/errors/internal_error.rs

## Import Cycles
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **Frontend Backend Communication Bridge** — specs_backend_hexagonal_backend_client, specs_backend_hexagonal_http_adapter, specs_backend_hexagonal_tauri_adapter, specs_backend_hexagonal_tanstack_query, specs_frontend_migration_yamsconfig [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **CI Release Pipeline** — github_workflows_publish_release_workflow, github_workflows_smoke_tests_workflow, github_workflows_unit_tests_workflow, github_workflows_tauri_action, github_workflows_mise_action [EXTRACTED 1.00]

## Communities (100 total, 23 thin omitted)

### Community 0 - "AppliableMigration"
Cohesion: 0.07
Nodes (49): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+41 more)

### Community 1 - "AppliedLog"
Cohesion: 0.06
Nodes (41): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+33 more)

### Community 2 - "babel plugin react compiler"
Cohesion: 0.05
Nodes (41): babel-plugin-react-compiler, @biomejs/biome, dependencies, next, react, react-dom, devDependencies, babel-plugin-react-compiler (+33 more)

### Community 3 - "enabled"
Cohesion: 0.05
Nodes (35): source, assist, actions, next, react, files, ignoreUnknown, includes (+27 more)

### Community 4 - "Hexagonal Architecture"
Cohesion: 0.06
Nodes (37): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+29 more)

### Community 5 - "Energetik Logo legacy"
Cohesion: 0.07
Nodes (37): Energetik Logo (legacy), Backend ER Model, Adresse, Behandlung, Behandlungsart, BehandlungsTermin, Buchungsart, Energetik Sabine Petschl (+29 more)

### Community 6 - "config rs"
Cohesion: 0.14
Nodes (16): dev_var_set(), project_dirs(), Arc, Default, From, Option, Self, Send (+8 more)

### Community 7 - "RegisterAddress tsx"
Cohesion: 0.15
Nodes (20): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, AddClientForm, ComboBoxItem (+12 more)

### Community 8 - "spec rs"
Cohesion: 0.11
Nodes (22): openapi_service(), Animal, C, Client, From, Into, Item, Path (+14 more)

### Community 9 - "animal ts"
Cohesion: 0.15
Nodes (14): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAnimal(), patchAnimal(), query(), deleteRace(), patchRace() (+6 more)

### Community 10 - "Connection"
Cohesion: 0.14
Nodes (20): Connection, RepositoryError, Option, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, InstanceType, Arc (+12 more)

### Community 11 - "clientRelation ts"
Cohesion: 0.18
Nodes (11): Address, Client, DatabaseObject, isRecord(), makeRecord(), makeRecordForTable(), Record, RecordError (+3 more)

### Community 12 - "tsconfig json"
Cohesion: 0.07
Nodes (28): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+20 more)

### Community 13 - "index tsx"
Cohesion: 0.19
Nodes (9): AnimalList, ClientItem, deleteClient(), ClientTableHeader, ClientTableRow, LiveRefresher, ClientOverview, NavigationPage (+1 more)

### Community 14 - "repository rs"
Cohesion: 0.18
Nodes (10): FakeClientsRepository, FakeDatastore, Arc, Client, Clone, Mutex, Self, T (+2 more)

### Community 15 - "queryClient ts"
Cohesion: 0.10
Nodes (17): queryClient, LanguagePicker, MainMenu, MainMenuCategory, MainMenuEntries, MainMenuItem, MainMenuItemData, MainMenuItems (+9 more)

### Community 16 - "index tsx 1"
Cohesion: 0.22
Nodes (13): EventsUsages, EventDetailItem, EventOverviewItem, askSubmitDeleteEvent(), submitDeleteEvent(), deleteEvent(), SubmissionState, useSubmissionState() (+5 more)

### Community 17 - "index ts"
Cohesion: 0.10
Nodes (17): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, DatabaseError, db (+9 more)

### Community 18 - "EventResponse"
Cohesion: 0.10
Nodes (4): EventResponse, SeminarResponse, EventStore, Store

### Community 19 - "address rs"
Cohesion: 0.21
Nodes (12): From, String, Address, EmailAddress, EmailAddressValidationError, MobileNumber, MobileNumberValidationError, AsRef (+4 more)

### Community 20 - "Versioned"
Cohesion: 0.21
Nodes (14): Versioned, animal_from_row(), parse_naive_date(), Animal, Arc, Mutex, NaiveDate, Option (+6 more)

### Community 21 - "Versioned T"
Cohesion: 0.13
Nodes (9): Clone, Deref, Formatter, T, Target, Versioned<T>, DerefMut, PartialEq (+1 more)

### Community 22 - "AddressTable tsx"
Cohesion: 0.13
Nodes (16): useAddresses(), AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages, EditButton (+8 more)

### Community 23 - "AddressResponse"
Cohesion: 0.25
Nodes (4): live(), AddressResponse, ano(), no()

### Community 24 - "export spec rs"
Cohesion: 0.14
Nodes (13): main(), Box, RepositoryResult, Unimplemented, Option, Send, Sync, SharedUnitOfWorkImpl (+5 more)

### Community 25 - "App"
Cohesion: 0.18
Nodes (12): App, Arc, Box, F, O, ResultReport, T, U (+4 more)

### Community 26 - "index tsx 2"
Cohesion: 0.17
Nodes (14): clientSearched(), deleteClientRelation(), relateClients(), updateClientRelation(), notification, NotificationActions, NotificationContent, TODO: Add possibility to also display notification on host system if in Tauri (+6 more)

### Community 27 - "repos rs"
Cohesion: 0.16
Nodes (5): AnimalRepository, ClientRepository, Send, Sync, Debug

### Community 28 - "UseCase"
Cohesion: 0.17
Nodes (12): UseCase, CreateAnimal, CreateAnimalError, CreateManyAnimals, CreateManyAnimalsError, Animal, Context, Error (+4 more)

### Community 29 - "uow rs"
Cohesion: 0.28
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 30 - "ClientParticipation tsx"
Cohesion: 0.23
Nodes (13): ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation(), relateClientParticipateEvent() (+5 more)

### Community 31 - "contact rs"
Cohesion: 0.22
Nodes (9): domain::EmailAddress, domain::MobileNumber, EmailAddress, MobileNumber, Error, Example, From, Self (+1 more)

### Community 32 - "client ts"
Cohesion: 0.19
Nodes (6): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useBackend()

### Community 33 - "Notifications tsx"
Cohesion: 0.19
Nodes (13): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationBehaviour, NotificationType, EditClient() (+5 more)

### Community 34 - "Animal"
Cohesion: 0.22
Nodes (10): Animal, Client, ResultReport, Uuid, Vec, Animal, NaiveDate, Uuid (+2 more)

### Community 35 - "Animal 1"
Cohesion: 0.27
Nodes (9): Animal, AnimalId, NewAnimal, NaiveDate, Uuid, FakeAnimalsRepository, Animal, RepositoryResult (+1 more)

### Community 36 - "client rs"
Cohesion: 0.23
Nodes (10): parse_naive_date(), Arc, Client, Mutex, NaiveDate, Option, ParseError, RepositoryResult (+2 more)

### Community 37 - "LockedUnitOfWorkImpl"
Cohesion: 0.26
Nodes (5): LockedUnitOfWorkImpl, Box, RepositoryResult, Self, UnitOfWork<'a>

### Community 38 - "index tsx 3"
Cohesion: 0.20
Nodes (5): MenuEntryData, Home, setupStore, StoreContext, Layout

### Community 39 - "clock rs"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - "SQLiteConnection"
Cohesion: 0.22
Nodes (10): Box, Error, Future, Option, Output, Pin, Send, Transaction (+2 more)

### Community 41 - "Arc"
Cohesion: 0.29
Nodes (8): Arc, Self, YamsAppApi, create_animal(), get_animals(), Animal, State, Vec

### Community 42 - "system clock rs"
Cohesion: 0.20
Nodes (7): DateTime, NaiveDate, Utc, SystemClock, Clock, Send, Sync

### Community 43 - "FnOp F"
Cohesion: 0.26
Nodes (10): FnOp<F>, OrchestrateFn, Context, Future, O, Output, Report, ResultReport (+2 more)

### Community 44 - "uow rs 1"
Cohesion: 0.29
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 45 - "EventForm tsx"
Cohesion: 0.33
Nodes (9): EventForm, createSeminarFormDate(), defaultSeminarFormData(), NewSeminarFormData, RegisterSeminarForm, createEvent(), updateEvent(), ensureSeminar() (+1 more)

### Community 50 - "NotificationInfo"
Cohesion: 0.27
Nodes (4): NotificationInfo, NotificationInfoType, NotificationEntry, NotificationStore

### Community 51 - "address rs 1"
Cohesion: 0.36
Nodes (6): Address, CountryCode, domain::Address, Example, From, Self

### Community 52 - "client rs 1"
Cohesion: 0.31
Nodes (9): Client, Address, Animal, EmailAddress, MobileNumber, NaiveDate, Uuid, Vec (+1 more)

### Community 53 - "client rs 2"
Cohesion: 0.36
Nodes (9): Client, ClientId, NewClient, Address, EmailAddress, MobileNumber, NaiveDate, Uuid (+1 more)

### Community 54 - "client rs 3"
Cohesion: 0.24
Nodes (8): CreateClient, CreateClientError, Address, Client, EmailAddress, Error, MobileNumber, NaiveDate

### Community 55 - "DialogInfo"
Cohesion: 0.27
Nodes (3): DialogInfo, DialogInfoType, DialogStore

### Community 57 - "client rs 4"
Cohesion: 0.28
Nodes (7): ClientCreation, CreateClient, Address, Error, NaiveDate, Self, TryFrom

### Community 58 - "structured error rs"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "EventParticipants tsx"
Cohesion: 0.33
Nodes (6): EventParticipants, participationSearchedClient(), SmallSearchField, DisplayCategory, eventSearched(), EventsOverview

### Community 61 - "animal rs"
Cohesion: 0.36
Nodes (6): AnimalCreation, CreateAnimal, From, NaiveDate, Self, Uuid

### Community 62 - "business conform rs"
Cohesion: 0.25
Nodes (5): base_app_builder(), AppBuilder, SetUowProvider, test_animal(), test_client()

### Community 66 - "mise"
Cohesion: 0.53
Nodes (6): mise, mise-action, Release Workflow, Integration Test Workflow, tauri-action, Unit Test Workflow

### Community 67 - "schema d ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "schema d ts 1"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 69 - "molting"
Cohesion: 0.60
Nodes (6): molting, yams, yams-api, yams-core, yams-persistence, yams-server

### Community 70 - "main rs"
Cohesion: 0.50
Nodes (4): BackendServerError, Config, main(), Report

### Community 73 - "business conform rs 1"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 74 - "BackendClient"
Cohesion: 0.50
Nodes (4): BackendClient, MobX, TanStack Query, yamsconfig.json

### Community 75 - "diffx finish review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

## Knowledge Gaps
- **173 isolated node(s):** `molting`, `ValidationError`, `UnitOfWork<'a>`, `CreateManyAnimalsError`, `CreateClientError` (+168 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `address rs` to `AppliedLog`, `Animal`, `Animal 1`, `main rs`, `internal error rs`, `spec rs`, `Arc`, `config rs`, `uow rs`, `address rs 1`, `client rs 1`, `client rs 2`, `client rs 3`, `client rs 4`, `structured error rs`, `UseCase`, `animal rs`, `contact rs`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **Why does `E` connect `AppliableMigration` to `structured error rs`, `FnOp F`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Why does `StructuredError` connect `structured error rs` to `AppliableMigration`, `spec rs`, `address rs`, `internal error rs`?**
  _High betweenness centrality (0.038) - this node is a cross-community bridge._
- **What connects `molting`, `ValidationError`, `UnitOfWork<'a>` to the rest of the system?**
  _173 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppliableMigration` be split into smaller, more focused modules?**
  _Cohesion score 0.06582278481012659 - nodes in this community are weakly interconnected._
- **Should `AppliedLog` be split into smaller, more focused modules?**
  _Cohesion score 0.05721153846153846 - nodes in this community are weakly interconnected._
- **Should `babel plugin react compiler` be split into smaller, more focused modules?**
  _Cohesion score 0.047619047619047616 - nodes in this community are weakly interconnected._