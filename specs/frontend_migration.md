# YAMS Frontend Migration Specification

## UI Framework Upgrade

- **Tailwind CSS**: v4.0.0+ (Latest Stable). Uses `@import "tailwindcss";` in `globals.css`.
- **Tauri**: v2.x (Latest Stable).

## State Management Transition

- **From**: MobX stores (`makeAutoObservable`).
- **To**: TanStack Query (`useQuery`, `useMutation`).
- **Pattern**:
  - Centralized hooks in `src/api/hooks/`.
  - Uses `BackendClient` abstraction for mode-agnostic data fetching.

## Backend Abstraction

- **Configuration**: `yamsconfig.json` determines the mode (`standalone` vs `embedded`).
- **Bridge**:
  - `client.ts`: Exports `HttpBackendClient` and `TauriBackendClient`.
  - `schema.d.ts`: Generated types from the Rust backend using `mise run frontend:generate-types`.

## Developer Experience

- **Tooling**: `mise` manages all tasks (build, dev, test, etc.).
- **Task Convention**: `function:scope` (e.g., `test:backend`, `build:tauri`).
- **Type Generation**: Automated from Rust `yams-dto` crate via `mise run generate:types`.
- **CI/CD**: GitHub Actions use `mise run ci` for checks and delegate builds to `mise run build:tauri` via `tauri-action`.
- **Environment**: Centralized configuration in `mise.toml`.
