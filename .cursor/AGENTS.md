# YAMS — Repository Context

YAMS is a Rust backend for veterinary practice management (Rust edition **2024**, **async** throughout). The codebase follows **Domain-Driven Design** and **hexagonal architecture**. Business crates are prefixed `yams-`. The frontend (Next.js + Tauri) is a separate concern — only touch it when explicitly asked.

## Workspace Layout

```
yams/
├── crates/
│   ├── yams-core/          # Domain, use cases, port definitions
│   ├── yams-api/           # Public API surface (DTOs, YamsAppApi, OpenAPI)
│   ├── yams-persistence/   # SQLite/libsql repository adapter
│   └── molting/            # Generic async migration framework
├── backend/server/         # Standalone HTTP server (yams-server)
├── frontend/
│   ├── src-tauri/          # Tauri shell (embedded deployment)
│   └── src/                # Next.js UI (deferred — minimal context here)
├── tasks/                  # mise task definitions
├── specs/                  # Architecture specs — may lag code; trust implementation
└── mise.toml               # Tool, env, and task orchestration
```

## Architectural Layers

```
┌─────────────────────────────────────────────────────────┐
│  Driving adapters                                       │
│  backend/server (Poem OpenAPI)  ·  src-tauri (IPC)      │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│  yams-api — YamsAppApi, schema DTOs, error mapping      │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│  yams-core — App, use cases, domain, ports              │
└────────────────────────┬────────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────────┐
│  yams-persistence — SQLite UoW, repos, migrations       │
└─────────────────────────────────────────────────────────┘
```

Dependency rule: **inward only**. Core knows nothing about HTTP, SQLite, or Tauri.

## German Ubiquitous Language

Domain language is **German** across the full stack — Rust types, SQLite tables/columns, API JSON keys (UTF-8, e.g. `Ländercode` → JSON `ländercode`). English legacy names are obsolete.

- **Types & files** — German feature-slice names (`klient.rs`, `KlientErstellen`), not English technical names.
- **Naming ladder** — `NeuerKlient` (domain) → `KlientErstellen` (use case) → `KlientErstellung` (API request).
- **JSON** — German field names, `camelCase` serde (`vorName`, `strasseUndHausnummer`).
- **HTTP paths** — German (`/klient`, `/tagesabschluss`).
- **Billing detail** — [`specs/abrechnung.md`](specs/abrechnung.md); trust code over stale English specs.

## yams-core — The Heart

Single bounded context, single subdomain (for now). Organized by **feature slices**, not technical layers:

```
yams-core/src/
├── domain/           # Aggregates & VOs (klient.rs, haustier.rs, leistung.rs, …)
├── service/          # Use cases (use_cases/klient.rs, use_cases/abrechnung.rs, …)
├── application/
│   ├── ports/        # Clock, KlientRepository, HaustierRepository, …
│   ├── uow.rs        # UnitOfWork facade, Versioned<T>
│   ├── context.rs    # ExecutionContext
│   └── mod.rs        # App, orchestration (begin → execute → commit/rollback)
└── adapters/         # Default port implementations (e.g. system_clock.rs)
```

Ports are `async_trait` traits; all repository and use-case I/O is async.

### Design Principles

**Make invalid state unrepresentable.** This is the guiding principle for domain modeling:

- **Newtypes for identity** — `KlientId(Uuid)`, `HaustierId(Uuid)` prevent ID mix-ups at compile time.
- **Validated value objects** — `EmailAdresse`, `Mobilnummer`, `Ländercode`, `Preis` can only be constructed through `new()` / `TryFrom`. Invalid values cannot exist in the type system.
- **Separate creation types** — `NeuerKlient` / `NeuesHaustier` have no ID; persisted aggregates always do.
- **Versioned concurrency** — `Versioned<T>` bundles entity + optimistic-lock version. Updates require the expected version; stale writes are rejected at the type boundary.
- **UoW access modes** — `locked()` vs `shared()` encode transaction semantics. Multi-step ops (`VieleHaustiereErstellen`, `TagesabschlussDurchfuehren`) use `ctx.to_locked()` + `checkpoint()`. **Rollback after checkpoint only reverts to last checkpoint** — do not checkpoint casually.
- **Single linkage** — `Haustier.klient_id` is the source of truth; no `haustier_ids` on `Klient`.

Domain may depend on ports directly (e.g. `Clock`). Use cases receive an `ExecutionContext` with UoW + clock access.

### App & Orchestration

`App` is the composition root. Built via `bon` builder:

```rust
App::builder()
    .uow_provider(Box::new(provider))
    .clock(Arc::new(SystemClock))
    .build()
```

All business operations go through `App::execute(use_case)` or `App::execute_fn(closure)`. The orchestrator begins a UoW, runs the operation, commits on success, rolls back on failure.

### UseCase Trait

```rust
pub trait UseCase<Output> {
    type Error: ThreadSafeError;
    fn perform(&self, ctx: ExecutionContext<'_>) -> impl Future<Output = ResultReport<Output, Self::Error>>;
}
```

One use case per business operation (`KlientErstellen`, `HaustierErstellen`, `TagesabschlussDurchfuehren`, …).

## yams-api — Public Interface

Framework-agnostic API layer between driving adapters and core.

- **`YamsAppApi`** — wraps `Arc<App>`, exposes typed methods (`klient_erstellen`, `haustier_erstellen`, `tagesabschluss_durchfuehren`, …). Translates domain → API schema DTOs.
- **`schema/`** — German DTOs, JSON `camelCase` (`vorName`, `ländercode`).
- **`requests/`** — inbound request types with `TryFrom` into use-case inputs.
- **`errors/`** — `Report<E>` → structured JSON error trees for HTTP responses.

### Feature Flags

| Feature   | Default | Purpose                              |
|-----------|---------|--------------------------------------|
| `serde`   | yes     | Serde derives on DTOs                |
| `openapi` | yes     | poem-openapi types, `spec.rs`, `export_spec` bin |

Consumers:
- **yams-server** — full features (OpenAPI HTTP + Swagger/ReDoc)
- **src-tauri** — `default-features = false, features = ["serde"]` (IPC only, no poem)

After schema or route changes, run `mise run build:openapi` to refresh `frontend/src/api/schema.d.ts`.

## yams-persistence — Driven Adapter

Single repository adapter (SQLite via libsql). Name may become more specific when additional adapters appear.

- **No ORM** — manual row parsing and saving. Lean, explicit, some boilerplate. DRY and AI assistance keep it manageable.
- **`SQLiteInstance`** — factory: `local(path)`, `in_memory()`, `in_temp_dir()`
- **`SQLiteUnitOfWork`** — implements `UnitOfWorkProvider` + `UnitOfWorkImpl`; checkpoint = commit + new deferred transaction
- **`repos/`** — `SQLiteKlientRepository`, `SQLiteHaustierRepository`, …
- **`migrations/`** — versioned SQL via `molting` framework (`v0001_initial.rs`, …)

## molting

Generic async migration framework used by yams-persistence. Provides `UpMigration`, `DownMigration`, `MigrationRegistry`, `MigrationTarget` traits. Persistence implements `MigrationTarget` for `SQLiteConnection`.

## Deployment Modes

| Mode       | Entry point          | How core is reached        |
|------------|----------------------|----------------------------|
| Server     | `backend/server/`    | Poem routes → `YamsAppApi` |
| Embedded   | `frontend/src-tauri/` | Tauri commands → `YamsAppApi` |

Both wire the same `App` + `SQLiteInstance` + migrations. Only the driving adapter differs.

## Error Handling

`error_stack::Report` throughout. Core defines `ResultReport<T, E> = Result<T, Report<E>>` and `ErrorReportExt` for contextualizing `thiserror` enums. Orchestration chains context on failure (`ExecutionError`, rollback errors via `.expand()` / `.push()`). API layer maps `Report<E>` to `StructuredError` for JSON responses.

## Testing Strategy

| Layer              | Where                          | What                                      |
|--------------------|--------------------------------|-------------------------------------------|
| Unit (conceptual)  | `yams-core` tests              | Pure business logic with fake adapters    |
| Adapter conformance| `yams-persistence/tests/`      | Same test cases as core, real SQLite UoW  |
| E2E / API          | `yams-api`                     | Invoke API methods, lightweight persistence |
| Integration        | Adapter crates (future)        | Adapter-specific validity                 |

### Shared Conformance Pattern

`base_app_builder()` in `yams-core/tests/business_conform.rs` returns an `AppBuilder` wired with `FakeUnitOfWorkProvider`. Test cases live in `yams-core/tests/cases/` and are shared via `#[path]` include in `yams-persistence/tests/business_conform.rs`, which overrides `base_app_builder()` to use `SQLiteInstance::in_temp_dir()`.

Unit tests inject `FakeAdapters` through this partial app builder. Persistence adapters prove conformance by running the full core test suite against their `UnitOfWorkProvider`.

## Tooling

- **mise** (`mise.toml`) — Rust toolchain, Node 22, env vars (`DATABASE_URL`, `FRONTEND_DIR`, `OPENAPI_SPEC`), task includes from `tasks/`
- **Rust nightly pinned** — `rust-toolchain.toml` uses `nightly-2026-07-02`; do not bump without testing (newer nightlies break poem-openapi lifetime capturing across await)
- **Key tasks**: `test:backend` (`cargo test`), `build:openapi` (export spec → `openapi-typescript` → `frontend/src/api/schema.d.ts`), `dev:server`, `dev:tauri`
- **Formatting/linting**: `fmt:rust`, `lint:crates` (clippy), `fmt:biome` (frontend)

## Frontend (Brief)

Next.js app in `frontend/`. Tauri shell in `frontend/src-tauri/`. OpenAPI types generated at `frontend/src/api/schema.d.ts`. Not the current focus — defer frontend work unless explicitly requested.

## Conventions for Contributors

1. **New feature?** Walk the vertical slice in order, German feature names (`seminar.rs`, not `model.rs`):
   `domain/` → `application/ports/` → `service/use_cases/` → `yams-api` (`requests/`, `schema/`, `YamsAppApi` method, `spec.rs` route if HTTP) → `yams-persistence/repos/` → migration in `migrations/` (if schema change) → test case in `yams-core/tests/cases/` → `mise run build:openapi` (if API surface changed).
2. **Domain changes stay in core.** API DTOs are a separate translation layer; never leak serde/openapi concerns into core.
3. **All mutations through `App::execute`.** No direct repo calls from adapters.
4. **Prefer types over runtime checks.** If a value can be invalid, make it impossible to construct without validation.
5. **Errors: `thiserror` in domain/use cases, `Report` at boundaries.** Use `.contextualize()` / `.change_context()` when crossing layers.
6. **Tests: add cases to `yams-core/tests/cases/`.** Persistence conformance follows automatically.
