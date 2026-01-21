# YAMS Hexagonal Backend Specification

## Core Domain (`yams-core`)
- **Entities**: Client, Address, Animal, Race, Event, Seminar, Participation, Relation.
- **Value Objects**: Email, PhoneNumber, Date, Price.
- **Ports**: 
  - `Repository`: For each entity (e.g., `ClientRepository`).
  - `UseCase`: Application services (e.g., `RegisterClientUseCase`).

## Persistence (`yams-persistence`)
- **Driver**: `libsql`.
- **Schema**: Replicate `setup.surql` in SQL. Use migrations.
- **Mappers**: Convert database rows to/from domain entities.

## Standalone Server (`yams-server`)
- **Framework**: `poem-openapi`.
- **Adapters**: REST controllers mapping OpenAPI requests to `yams-core` use cases.
- **Spec Generation**: Export `openapi.json` on build/start.

## Tauri Embedded (`yams-tauri`)
- **Adapters**: Tauri commands mapping IPC messages to `yams-core` use cases.
- **State**: Shared state holding the `yams-core` services and `yams-persistence` adapter.

## Communication Bridge
- **Frontend Interface**: `BackendClient` interface in TypeScript.
- **Implementations**: 
  - `HttpAdapter`: Uses `openapi-fetch` to talk to `yams-server`.
  - `TauriAdapter`: Uses `invoke` to talk to `yams-tauri`.
- **State Management**: TanStack Query hooks consuming `BackendClient`.
