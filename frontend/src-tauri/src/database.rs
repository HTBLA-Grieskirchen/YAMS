use std::ops::Deref;
use std::sync::Arc;

use surrealdb::{Datastore, Error, Session};
use surrealdb::sql::{Object, Value};
use tauri::async_runtime::Mutex;

use crate::YAMSBackendConfig;

pub struct Database {
    store: Arc<Mutex<Option<Datastore>>>,
    session: Session,
}

impl Deref for Database {
    type Target = Arc<Mutex<Option<Datastore>>>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl Database {
    pub fn session(&self) -> &Session {
        &self.session
    }
}

impl Default for Database {
    fn default() -> Self {
        Database {
            store: Arc::new(Mutex::new(None)),
            session: Session::for_db("yams", "yams"),
        }
    }
}

impl Database {
    pub async fn setup(config: &YAMSBackendConfig) -> Result<Self, Error> {
        if config.uses_local_database {
            Self::new(&config.local_database_location).await
        } else {
            Ok(Self::default())
        }
    }

    pub async fn new(location: &str) -> Result<Self, Error> {
        let database = Database::default();

        // Create actual datastore
        let datastore = Database::new_datastore(location, database.session()).await?;

        let _ = database.lock().await.insert(datastore);
        Ok(database)
    }

    pub async fn new_datastore(location: &str, session: &Session) -> Result<Datastore, Error> {
        let datastore = Datastore::new(&location).await?;

        // Load sql script if datastore is new
        let mut import = true;
        let result = datastore.execute("INFO FOR DB", session, None, false).await?;
        if let Some(single_result) = result.first() {
            if let Ok(valid_result) = &single_result.result {
                if let Value::Object(Object(data)) = valid_result {
                    if let Some(Value::Object(Object(tb_data))) = data.get("tb") {
                        if !tb_data.is_empty() {
                            import = false;
                        }
                    }
                }
            }
        }

        if import {
            println!("Import data scheme into SurrealDB");
            datastore.execute(&Database::setup_script(), session, None, false).await?;
        }

        Ok(datastore)
    }

    pub fn setup_script() -> String {
        include_str!("../../../backend/setup.sql").to_string()
    }
}