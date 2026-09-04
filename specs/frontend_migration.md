# YAMS Frontend Migration Specification

## UI Framework Upgrade

- **Tailwind CSS**: v4.0.0+ (Latest Stable). Uses `@import "tailwindcss";` in `globals.css`.
- **Tauri**: v2.x (Latest Stable).

## State Management Transition

- **From**: MobX stores (`makeAutoObservable`).
- **To**: TanStack Query (`useQuery`, `useMutation`).
- **Pattern**:
  - Centralized hooks in `src/api/hooks/`.
  - Uses `YamsApi` abstraction for mode-agnostic data fetching.

## Backend Abstraction

- **Configuration**: per-runtime files, not a shared schema.
  - Tauri: tagged `mode` (`embedded` | `remote`) in `YAMS_CONFIG_PATH` / `{ProjectDirs}/yams.json`. Env overlays (`YAMS_MODE`, path/URL vars, `YAMS_DEV`). Next.js in Tauri uses only the `frontend_config` invoke.
  - Browser: static `src/yams-config.json` plus `NEXT_PUBLIC_YAMS_API_URL` / `NEXT_PUBLIC_YAMS_DEV`. Always remote.
- **Bridge**:
  - `HttpYamsApi` / `TauriYamsApi` implementing `YamsApi`.
  - `schema.d.ts`: Generated types from the Rust backend using `mise run build:openapi`.

## Developer Experience

- **Tooling**: `mise` manages all tasks (build, dev, test, etc.).
- **Task Convention**: `function:scope` (e.g. `test:backend`, `build:tauri`).
- **Type Generation**: Automated from Rust OpenAPI via `mise run build:openapi`.
- **CI/CD**: GitHub Actions use `mise run ci` for checks and delegate builds to `mise run build:tauri` via `tauri-action`.
- **Environment**: Task-local config paths in `tasks/dev.toml` (`dev:server`, `dev:tauri+embedded`, `dev:tauri+server`, `dev:frontend+server`).
