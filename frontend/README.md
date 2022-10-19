# Frontend using React in Next.js and Tauri

The UI Frontend of YAMS is implemented as a TypeScript React App shipped with [Next.js](https://nextjs.org/).
It may be started as a web application, but may also be shipped as standalone desktop application with
[Tauri](https://tauri.app).

## Web Application

The web application uses a centralized remote data storage. It runs in the browser
and can be accessed from any device with a web browser.

### Prerequisites

- Installed [Node.js](https://nodejs.org) and [npm](https://www.npmjs.com/)

### Preparation

Run `npm install` when interacting with the project for the first time to download all dependencies.

### Development

To run the web application in development mode with hot reloading run:

`npm run dev`

Open [http://localhost:3000](http://localhost:3000) and make sure the data storage is up and running
locally on your machine as described in the [backend README](../backend/README.md).

### Deployment

To deploy the application in production mode run the following commands:

- `npm run build` to build the application
- `npm run start -p 8080` to serve the application on port 8080

The port can be changed accordingly.

Also make sure that the configured data storage is installed and running as described in the
[backend README](../backend/README.md).

### Usage

Open [http://localhost:8080](http://localhost:8080) to access the running application locally.   
To access it on other devices, open `http://<hostname>:8080`.

## Desktop Application

The desktop application is a bundle of the web application with native capabilities and an embedded
[SurrealDB](https://surrealdb.com) data storage, which is used by default, using Tauri. Although it is still possible to
connect to a remote data
storage.

### Prerequisites

- Installed [Rust](https://rust-lang.org) (recommended tool:
  [Rustup](https://tauri.app/v1/guides/getting-started/prerequisites))
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your system
- `patch` program in your $PATH environment variable
- Installed `clang` for [Rust bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)
- Installed `tauri-cli` using `cargo install tauri-cli`

### Development

To start the app in development mode with hot reloading run:

`cargo tauri dev`

This will start the web application and opens a Tauri window to display it.

It is recommended to export a `YAMS_DEV=1` environment variable in order to start the app in development mode.
This makes the app use paths optimized for the development process.

### Building

To build the app for distribution and production run following command:

`cargo tauri build`

This bundles everything into a single distributable binary for your host system. The bundles can be found under
[`src-tauri/target/release/bundle/`](src-tauri/target/release/bundle/).

### Usage

If configured to use a centralized data storage, also make sure that this storage is set up as described in the
[backend README](../backend/README.md).
