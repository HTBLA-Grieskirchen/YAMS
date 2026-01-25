# Backend

The backend is implemented using [libsql](https://libsql.org) (SQLite) with a hexagonal architecture in Rust.

## Architecture

The backend follows the Hexagonal Architecture (Ports and Adapters) pattern:

- **Core Domain**: Pure Rust business logic and repository traits.
- **Persistence Adapter**: Implementation of repository traits using `libsql`.
- **Standalone Server**: REST API using `poem-openapi`.
- **Tauri Embedded**: Embedded Rust module within the Tauri application.

## Standalone Server

To run the standalone webserver:

```bash
cargo run -p yams-server
```

## Spec Export

To export the OpenAPI spec:

```bash
cargo run -p yams-dto --bin export_spec > openapi.json
```
