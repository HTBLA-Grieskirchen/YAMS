# Frontend using React in Next.js and Tauri

The UI Frontend of YAMS is implemented as a TypeScript React App with [Next.js](https://nextjs.org/).
It can be run as a standalone web application or as a desktop application with [Tauri](https://tauri.app).

## Development

### Web Application

To run the web application in development mode:
```bash
npm install
npm run dev
```

### Desktop Application

To start the Tauri application:
```bash
mise run dev:tauri
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

The application can be configured via a `yamsconfig.json` file. 

Example for standalone mode (remote database):
```json
{
  "remoteDatabaseLocation": "http://127.0.0.1:3000/api"
}
```

By default, the desktop app uses an embedded SQLite database.
