# Backend using SurrealDB

The backend is implemented using SurrealDB and its native Rest capabilities. This
allows us to directly access the DB with authentication and saves us from having to write
a custom backend.

## Installation

The DB can be installed locally using the [`install.sh`](install.sh) script. The script
installs the DB from the official online sources, starts it in a screen session and loads the
provided [`setup.sql`](setup.sql).

### Prerequisites

- Linux OS
- `bash` installed
- `screen` installed

Alternatively the DB can be installed as described [here](https://surrealdb.com/install). It can then
manually be run in the background (like [this](start.sh)) and the setup can be loaded using
[this command](setup.sh).

## Miscellaneous

The [SurrealQL script](setup.sql) which configures and prepares the DB to be used with this project
can be used in any way (f.e. to setup dev environments). It represents the [Data Model](model.jpg)
in a way understandable for SurrealDB.