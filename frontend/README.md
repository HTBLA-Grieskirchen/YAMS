# Frontend using React in Next.js and Tauri

The UI Frontend of YAMS is implemented as a TypeScript React App with [Next.js](https://nextjs.org/).
It can be run as a standalone web application or as a desktop application with [Tauri](https://tauri.app).

## Development

### Web Application (talks to `yams-server`)

```bash
mise run dev:frontend+server
```

### Desktop Application (embedded backend)

```bash
mise run dev:tauri+embedded
```

Tauri shell only (no Next.js): `mise run dev:tauri` (add `--remote` for server mode).

### Desktop Application (remote `yams-server`)

```bash
mise run dev:tauri+server
```

## Building

### Web Application

```bash
npm run build
```

### Desktop Application

```bash
mise run build:tauri
```

## Configuration

Resolution order: **config path → file → env**. `YAMS_DEV` is a UX flag (TanStack Query devtools); it does not change database or object-store paths.

### Tauri

Default file: `{ProjectDirs config_dir}/yams.json`. Override with `YAMS_CONFIG_PATH`.

`mode` is a tagged enum — each variant only has the fields it needs:

```json
{
  "mode": "embedded",
  "databaseUrl": "/path/to/yams.db",
  "objectStoreDir": "/path/to/objects",
  "dev": false
}
```

```json
{
  "mode": "remote",
  "remoteApiUrl": "http://127.0.0.1:3000/api",
  "dev": false
}
```

Env overlay (`YAMS_MODE`, `YAMS_DATABASE_URL`, `YAMS_OBJECT_STORE_DIR`, `YAMS_REMOTE_API_URL`, `YAMS_DEV`) must match the target mode; mixed keys are a load error.

The webview reads configuration **only** from the `frontend_config` Tauri command.

Dev files: [`config.embedded.dev.json`](config.embedded.dev.json), [`config.remote.dev.json`](config.remote.dev.json). Mise sets absolute database/object-store paths for embedded mode.

### Standalone browser

Always remote. Static defaults in [`src/yams-config.json`](src/yams-config.json) (imported at build time). Overlay with `NEXT_PUBLIC_YAMS_API_URL` and `NEXT_PUBLIC_YAMS_DEV`.
