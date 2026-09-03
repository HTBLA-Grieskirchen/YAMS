# YAMS — Repository Context

YAMS is a Rust backend for veterinary practice management (Rust edition **2024**, **async** throughout). The codebase follows **Domain-Driven Design** and **hexagonal architecture**. Business crates are prefixed `yams-`. The frontend (Next.js + Tauri) is a separate concern — only touch it when explicitly asked.

## Workspace Layout

```
yams/
├── crates/
│   ├── yams-core/          # Domain, use cases, port definitions
│   ├── yams-api/           # Public API surface (DTOs, YamsAppApi, OpenAPI)
│   ├── yams-persistence/   # SQLite/libsql repository adapter
│   ├── yams-fakes/         # In-memory adapters, FixedClock, future factories/seeding
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

## Language & Naming

Two rules apply everywhere in this repo.

### English everywhere (except Ubiquitous Language)

**Docs, comments, commit messages, and non-domain source** are **English** — `AGENTS.md`, architecture notes, `///` and `//` comments, DDD/technical terms (`Aggregate`, `Value object`, `Repository`, `Use case`), technical error context.

**German is reserved for Ubiquitous Language only** — domain concepts that belong in the veterinary-practice vocabulary:

- Rust types and modules: `Klient`, `Behandlung`, `Leistung`, `Tagesabschluss`, `KlientErstellen`
- Persisted names: SQLite tables/columns for domain fields (`klienten`, `leistungen`, `straße_und_hausnummer`)
- API surface for domain: HTTP paths (`/klient`, `/tagesabschluss`), JSON keys (`vorname`, `straßeUndHausnummer`, `ländercode`, `mwst`)
- Domain specs ([`specs/abrechnung.md`](specs/abrechnung.md)) — trust code over stale English specs

**Naming ladder** — `NeuerKlient` (domain) → `KlientErstellen` (use case) → `KlientErstellung` (API request). German feature-slice file names (`klient.rs`, `abrechnung.rs`), not English technical names (`client.rs`, `billing.rs`).

### UTF-8, not ASCII escapes

Use **proper UTF-8** in identifiers and persisted names. Never transliterate umlauts to `ae`/`oe`/`ue` (no `naechste`, `strasse`, `laendercode`, `stueckzahl`).

- **Rust** — `nächste_rechnungsnummer`, `straße_und_hausnummer`, `Ländercode`, `TagesabschlussDurchführen`
- **SQLite** — quote when needed: `"straße_und_hausnummer"`, `"stückzahl"`, `"ländercode"`
- JSON — UTF-8 keys via `camelCase` serde (`straßeUndHausnummer`, `stückzahl`, `mwst`); prefer UTF-8 field names in schema types over ASCII fields + `serde(rename)` workarounds
- **Migrations** — final schema in `v0002` (feature dev); squash incremental alters while pre-deploy

Tooling handles UTF-8 in source files — use it.

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

**Make invalid state unrepresentable.** Guiding principle for domain modeling:

- **Newtypes for identity** — `KlientId(Uuid)`, `HaustierId(Uuid)` prevent ID mix-ups at compile time.
- **Validated value objects** — `EmailAdresse`, `Mobilnummer`, `Preis`, `Ratio` (`0..=1`; 100% = `1`), `Menge` (non-negative, unitless) via `new()` / `TryFrom`. Invalid values cannot exist in the type system.
- **Closed enums where closed** — `Ländercode` is an enum (`AT`, `DE`, `CH`), not a free-form string.
- **Type-state aggregates** — `LeistungIn<Offen>` / `LeistungIn<Abgerechnet>` (aliases `LeistungOffen`, `LeistungAbgerechnet`), `RechnungIn<…>` likewise. Enum `Leistung` / `Rechnung` sums all valid states when compile-time state unknown.
- **Separate creation types** — `NeuerKlient` / `NeuesHaustier` (and the other `Neue*` builders) have no ID and do **not** validate; they are input records. Repositories call `Haustier::neu` / `Klient::neu` (and `from_parts` when loading), which own empty-name and construction invariants.
- **Encapsulation over `pub`** — aggregate fields private; expose accessors and mutation through domain methods (`neu`, `mark_abgerechnet`, `aus_leistungen`). Request DTOs and `Neue*` builders never assemble a valid aggregate. Repositories invoke `::neu` on create and `from_parts` on load. Construction `Report`s `.attach("while constructing klient")` (and the analogous haustier/produkt/behandlung/leistung strings) so the site is visible in the report.
- **Derived values as getters** — `Leistung::betrag()` from `LeistungQuelle`; `Rechnung::gesamtbetrag_brutto()` from `Rechnungspositionen`. No stored duplicates that can drift.
- **Price snapshots on Leistung** — `LeistungQuelle` stores booked prices (`einzelpreis`, `menge`, `preis`) so Tagesabschluss uses historical values, not current catalog prices.
- **Versioned concurrency** — `Versioned<T>` bundles entity + optimistic-lock version.
- **UoW** — use cases call `ctx.enter()` for an owned UoW (borrows ctx). Nested work uses `ctx.sub(&uow)` (ports stay on the context). Nested `enter` joins the outer TX; nested `commit`/`rollback` are no-ops. Collect work without `?` across the UoW, then `uow.finish(result, commit_context)` (commit on `Ok`, rollback on `Err`). Drop panics if neither `commit` nor `rollback` ran. `#[must_use]` on `UnitOfWork`.
- **Single linkage** — `Haustier.klient_id` is source of truth; no `haustier_ids` on `Klient`.

**Repositories are dumb.** Ports persist and load domain types — no business logic (no `mark_abgerechnet` on repository). State transitions happen in domain/use cases; repos `update` the mutated entity.

**Error handling conventions:**
- **`Report<E>` everywhere** — use cases, domain services (`RechnungOffen::aus_leistungen`), orchestration. Use `.change_context()` directly; avoid redundant `IntoReport::into_report`.
- **Exception: newtype validation** — `EmailAdresse::new`, `Preis::new`, `Ländercode::from_str` return plain `Result<T, ValidationError>` — nothing cross-cutting can fail at construction.
- **Preis arithmetic** — implement `Add`; addition of two `Preis` values cannot fail. Scaling is infallible `Mul`: `&Preis * &Menge` and `&Preis * &Ratio` (MwSt is a ratio, so `netto * 0.19`, never `/100`).

Domain may depend on ports directly (e.g. `Clock`). Use cases receive an `ExecutionContext` (ports + UoW provider). They `enter` / `finish` the UoW themselves.

### App & Orchestration

`App` is the composition root. Built via `bon` builder:

```rust
App::builder()
    .uow_provider(Box::new(provider))
    .clock(Arc::new(SystemClock))
    .build()
```

All business operations go through `App::execute(use_case)` or `App::execute_fn(closure)`. `App` only builds an `ExecutionContext`; the use case (or closure) enters and finishes the UoW. The use-case `Result` is returned as-is — no `ExecutionError` wrap.

### UseCase Trait

```rust
pub trait UseCase<Output> {
    type Error: ThreadSafeError;
    fn perform(&self, ctx: ExecutionContext<'_>) -> impl Future<Output = ResultReport<Output, Self::Error>>;
}
```

One use case per business operation (`KlientErstellen`, `HaustierErstellen`, `TagesabschlussDurchführen`, …).

## yams-api — Public Interface

Framework-agnostic API layer between driving adapters and core.

- **`YamsAppApi`** — wraps `Arc<App>`, exposes typed methods (`klient_erstellen`, `haustier_erstellen`, `tagesabschluss_durchführen`, …). Translates domain → API schema DTOs. Use-case errors pass through.
- **`schema/`** — German DTOs, JSON `camelCase` (`vorname`, `ländercode`, `mwst` as a `0..=1` ratio, e.g. `"0.20"` not `"20"`).
- **`requests/`** — inbound request types with `TryFrom` into use-case inputs. Domain validation is `change_context(ValidationError)` only; HTTP status for validation is attached as `StatusCode` at the API boundary.
- **`errors/`** — `Report<E>` → structured JSON error trees. `YamsApiSpec` resolves HTTP status: attached `StatusCode` on the report, else `HttpStatusMapping` on the current context, else the first nested mapped context (domain / other use-case / repository), else 500. Persistence wrappers (`Persistenz`, `Erstellung`, `Invariante`, `Rechnung`) return `None` so the cause can speak. Not-found → 404, wrong aggregate state / booking conflict → 409, request validation → 400, unprocessable invoice contents → 422, permission → 403, connection → 503.

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
- **`SQLiteUnitOfWork`** — implements `UnitOfWorkProvider` + `UnitOfWorkImpl`
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

`error_stack::Report` throughout. Core defines `ResultReport<T, E> = Result<T, Report<E>>` and `ErrorReportExt` for contextualizing `thiserror` enums. `App::execute` returns the use-case report. API layer maps `Report<E>` to `StructuredError` and HTTP status as above.

## Testing Strategy

| Layer              | Where                                      | What                                      |
|--------------------|--------------------------------------------|-------------------------------------------|
| Domain unit        | `#[cfg(test)]` in the domain source file   | Isolated VO/aggregate math (Preis, MwSt, contact validation) |
| Business conform   | `yams-core/tests/cases/` (integration)     | Full use-case flows with `yams-fakes`     |
| Adapter conformance| `yams-persistence/tests/`                  | Same cases as core, real SQLite UoW        |
| E2E / API          | `yams-api/tests/e2e/`                      | Poem `YamsApiTestClient` JSON nested at `/api`; `base_app_builder()` (SQLite, overridable adapters) |

### yams-fakes

`crates/yams-fakes/` — shared fake adapters, not buried in test trees: `FakeUnitOfWorkProvider`, `FakeDatastore`, per-entity `Fake*Repository`, `FixedClock`. Dev-dep of test crates today; also intended for scripts and data seeding (factory-style builders later).

### Shared Conformance Pattern

`base_app_builder()` in `yams-core/tests/business_conform.rs` wires `yams_fakes::FakeUnitOfWorkProvider`. Cases live in `yams-core/tests/cases/` and are shared via `#[path]` in `yams-persistence/tests/business_conform.rs`, which overrides `base_app_builder()` to `SQLiteInstance::in_temp_dir()`. Tests that need a fixed date use `yams_fakes::FixedClock` on either builder.

Persistence proves adapter conformance by running the same case suite against real SQLite.

## Tooling

- **mise** (`mise.toml`) — Rust toolchain, Node 22, env vars (`DATABASE_URL`, `FRONTEND_DIR`, `OPENAPI_SPEC`), task includes from `tasks/`
- **Rust nightly pinned** — `rust-toolchain.toml` uses `nightly-2026-07-02`; do not bump without testing (newer nightlies break poem-openapi lifetime capturing across await)
- **cargo-nextest** — `cargo nextest` for running tests
- **Key tasks**: `test:backend` (`cargo test`), `build:openapi` (export spec → `openapi-typescript` → `frontend/src/api/schema.d.ts`), `dev:server`, `dev:tauri`
- **Formatting/linting**: `fmt:rust`, `lint:crates` (clippy), `fmt:biome` (frontend)

## Frontend

Next.js in `frontend/`, Tauri shell in `frontend/src-tauri/`. OpenAPI types at `frontend/src/api/schema.d.ts` (`mise run build:openapi`).

### API layer

- **`YamsApi`** (`src/api/yams-api.ts`) — framework-agnostic interface mirroring `YamsAppApi`.
- **Adapters** — `HttpYamsApi` (openapi-fetch + schema paths) for remote mode; `TauriYamsApi` (invoke) for embedded mode. Mode from `NEXT_PUBLIC_YAMS_MODE`, `isTauri()`, or `frontend_config.remoteDatabaseLocation`.
- **Bootstrap** — `YamsApiProvider` resolves adapter once; components never choose HTTP vs Tauri themselves.

### TanStack Query (server state)

**All backend reads and writes go through TanStack Query hooks** in `frontend/src/api/hooks/` — not raw `useEffect` + `fetch`, not direct `YamsApi` calls in components.

| Layer | Responsibility |
|-------|----------------|
| `YamsApi` + adapters | Transport only (HTTP or Tauri) |
| `api/hooks/queries.ts` | `useQuery` for reads (`useAlleHaustiereQuery`, …) |
| `api/hooks/mutations.ts` | `useMutation` for writes; invalidate or update `query-keys` on success |
| `api/query-keys.ts` | Hierarchical keys for targeted invalidation |
| Components | Consume hooks; render `isPending` / `error` / `data` |

**Rules:**

1. **Queries** — `enabled: isReady` from `useYamsApiReady()` so nothing runs before the adapter exists.
2. **Mutations** — call `YamsApi` inside `mutationFn`; on success invalidate the smallest matching key prefix (e.g. `yamsKeys.haustiere.all()` after create).
3. **No duplicate server state** — do not copy query results into local `useState`; use `queryClient.setQueryData` when optimistically updating cache.
4. **New endpoint** — add method to `YamsApi`, both adapters, a key in `query-keys.ts`, and a hook in `queries.ts` or `mutations.ts`; use the hook in UI.
5. **Defaults** — `createQueryClient()` (`src/lib/query-client.ts`): 30s `staleTime`, refetch on window focus, mutations do not retry.

`QueryClientProvider` wraps `YamsApiProvider` in `src/app/providers.tsx`.

**Remote dev (browser + `yams-server`)** — Next and the API must use different ports (`mise run dev:frontend+server`: API `:3000`, Next `:3001`). Cross-origin fetch requires CORS on `yams-server` (localhost / 127.0.0.1). Set `NEXT_PUBLIC_YAMS_API_URL` when the API is not at `http://127.0.0.1:3000/api`. Use matching hostnames (`localhost` vs `127.0.0.1`) in the browser URL and API URL.

## Conventions for Contributors

1. **New feature?** Walk the vertical slice in order, German feature names (`seminar.rs`, not `model.rs`):
   `domain/` (unit tests in the same file) → `application/ports/` → `service/use_cases/` → `yams-api` (`requests/`, `schema/`, `YamsAppApi` method, `spec.rs` route if HTTP) → `yams-persistence/repos/` → migration in `migrations/` (if schema change) → use-case case in `yams-core/tests/cases/` → API e2e in `yams-api/tests/e2e/` when the public surface changed → `mise run build:openapi` (if API surface changed).
2. **Domain changes stay in core.** API DTOs are a separate translation layer; never leak serde/openapi concerns into core.
3. **All mutations through `App::execute`.** No direct repo calls from adapters.
4. **Prefer types over runtime checks.** If a value can be invalid, make it impossible to construct without validation.
5. **Errors: `thiserror` in domain/use cases, `Report` at boundaries.** Use `.contextualize()` / `.change_context()` when crossing layers. English for technical messages; German only in domain/user-facing UL strings where appropriate.
6. **Language** — English docs/comments/technical code; German UL for domain names; UTF-8 identifiers, no `ae`/`oe`/`ue` transliteration (see Language & Naming).
7. **Tests:** domain units live in a `#[cfg(test)]` module in the source file; use-case flows go in `yams-core/tests/cases/` (persistence conformance follows automatically); API e2e uses Poem `YamsApiTestClient` in `yams-api/tests/e2e/` (`main.rs` holds the client, `base_app_builder()`, and JSON helpers).
8. **Frontend data** — add TanStack Query hooks in `frontend/src/api/hooks/`; never fetch from components directly (see Frontend → TanStack Query).
