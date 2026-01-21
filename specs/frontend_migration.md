# YAMS Frontend Migration Specification

## UI Framework Upgrade
- **Tailwind CSS**: v4.0.0+ (Latest Stable).
- **Tauri**: v2.9.5+ (Latest Stable).

## State Management Transition
- **From**: MobX stores (`makeAutoObservable`).
- **To**: TanStack Query (`useQuery`, `useMutation`).
- **Pattern**: 
  - Use `queryOptions` from generated OpenAPI types.
  - Implement optimistic updates for critical paths (e.g., adding a client).
  - Use centralized query keys based on the OpenAPI spec.

## Backend Abstraction
- **Configuration**: `yamsconfig.json` determines the mode (`standalone` vs `embedded`).
- **Bridge**:
  - `api-client.ts`: Exports a singleton or context provider that returns the correct `BackendClient` implementation.
  - `schema.d.ts`: Generated types from the Rust backend.
