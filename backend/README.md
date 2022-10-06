# Backend using SurrealDB

The backend is implemented using SurrealDB and its embedded or native Rest capabilities. This
allows us to directly access the DB with authentication and saves us from having to write
a custom backend.

## Server

If the DB is intended to be used for centralized data storage - thus requiring it to be installed
on a remote server - the following chapters help to set up the system on the server.

### Installation

The DB can be installed locally using the provided scripts.

#### Steps

- Change working directory to where this README resides
- Execute `./scripts/install.sh` to run the [install script](scripts/install.sh). 

The script installs the DB from the official online sources, starts it in a screen session and imports 
the provided [setup.sql](setup.sql) file.

#### Prerequisites

- Linux OS
- `bash` installed
- `screen` installed

Alternatively the DB can be installed as described [here](https://surrealdb.com/install). It can then
manually be run in the background (like [this](scripts/start.sh)) and the setup can be loaded using
[this command](scripts/setup.sh).

## Embedded

When the whole software runs locally as desktop application, SurrealDB is provided
as embedded library in the Tauri backend code. It also uses the [`setup.sql`](setup.sql)
script to initialize the DB upon first use.

## Miscellaneous

The [`setup.sql`](setup.sql) SurrealQL script, which configures and prepares the DB to be used with this project,
can be used in any way (f.e. to setup dev environments). It represents the [Data Model](resources/backend_model.jpg)
in a way understandable for SurrealDB.
