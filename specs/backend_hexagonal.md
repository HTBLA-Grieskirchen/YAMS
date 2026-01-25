# YAMS Hexagonal Backend Specification

## Core Domain (`yams-core`)

- **Entities**: Client, Address, Animal, Race, Event, Seminar, Participation, Relation.
- **Value Objects**: Email, PhoneNumber, Date, Price.
- **Ports**:
  - `Repository`: For each entity (e.g., `ClientRepository`).
- **Services**: Domain services (e.g., `AddressService`) that depend on specific `Repository` ports.
- **Context**: `YamsContext` holds all domain services and represents the unified entrypoint for ingoing adapters.
- **Error Handling**: Uses `thiserror` for idiomatic error propagation and `Result<T>` alias.

## DTO Layer (`yams-dto`)

- **Purpose**: Defines shared `poem-openapi` models used by both `yams-server` and `yams-tauri`.
- **Naming**: Uses `camelCase` for compatibility with the frontend.
- **Spec Export**: Provides a binary `export_spec` to print the OpenAPI spec to stdout.

## Persistence (`yams-persistence`)

- **Driver**: `libsql` (SQLite).
- **Schema**: Replicate `setup.surql` in SQL. Use migrations via `libsql_migration`.
- **Mappers**: Convert database rows to/from domain entities.

## Standalone Server (`yams-server`)

- **Framework**: `poem-openapi`.
- **Adapters**: REST controllers mapping OpenAPI requests to `yams-core` services.
- **Injection**: Uses `YamsContext` and domain services for logic.

## Tauri Embedded (`yams-tauri`)

- **Adapters**: Tauri commands in `frontend/src-tauri` mapping IPC messages to `yams-core` services.
- **State**: Shared state holding `YamsContext` and domain services.

## Communication Bridge

- **Frontend Interface**: `BackendClient` interface in TypeScript.
- **Implementations**:
  - `HttpAdapter`: Uses `openapi-fetch` to talk to `yams-server`.
  - `TauriAdapter`: Uses `invoke` to talk to `yams-tauri`.
- **State Management**: TanStack Query hooks consuming `BackendClient`.
