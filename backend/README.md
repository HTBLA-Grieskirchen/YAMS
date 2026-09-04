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
mise run dev:server
```

### Configuration

Resolution order: **config path → file → env → CLI**.

Default file: `yams-server.json` in the current working directory. Override with `YAMS_CONFIG_PATH` or `--config-path`. Missing default file falls back to built-in defaults. An explicit path that is missing or malformed is an error.

Dev file used by mise: [`backend/config.dev.json`](config.dev.json). Mise also sets absolute `YAMS_DATABASE_URL` and `YAMS_OBJECT_STORE_DIR` so the server is not cwd-dependent.

| Source | Keys |
|--------|------|
| File (camelCase JSON) | `databaseUrl`, `objectStoreDir`, `bindAddress`, `port`, `subpath` |
| Env / CLI | `YAMS_DATABASE_URL` / `--database-url`, `YAMS_OBJECT_STORE_DIR` / `--object-store-dir`, `BIND_ADDRESS` / `--bind-address`, `PORT` / `--port`, `SUBPATH` / `--subpath` |
| Tracing | `YAMS_LOG_LEVEL` (default `info`) |

## Spec Export

To export the OpenAPI spec:

```bash
mise run build:openapi
```
