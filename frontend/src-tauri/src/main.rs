#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::collections::BTreeMap as Map;
use std::default::Default;

use surrealdb::{Error, Response};
use surrealdb::sql::json;
use tauri::{Builder, Wry};

use crate::config::{YAMSBackendConfig, YAMSFileConfig, YAMSFrontendConfig};
use crate::database::Database;

mod config;
mod database;

#[tauri::command]
async fn setup_database(
    database: tauri::State<'_, Database>,
    config: tauri::State<'_, YAMSBackendConfig>,
) -> Result<(), Error> {
    // Don't setup database if already exists
    let mut database_lock = if check_database(tauri::State::clone(&database)).await.unwrap_or(false) {
        return Ok(());
    } else {
        database.lock().await
    };

    // Inject new datastore in database
    let _ = database_lock.insert(
        Database::new_datastore(&config.local_database_location, database.session()).await?);

    Ok(())
}

#[tauri::command]
async fn check_database(
    database: tauri::State<'_, Database>
) -> Result<bool, ()> {
    if let Some(_) = &*database.lock().await {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn query_database(
    query: &str,
    vars: Option<Map<String, serde_json::Value>>,
    database: tauri::State<'_, Database>,
) -> Result<Vec<Response>, Error> {
    if let Some(datastore) = &*database.lock().await {
        let mut parse_failure: Option<Result<Vec<Response>, Error>> = None;

        let vars = vars.map(|map| {
            map.iter().filter_map(|(k, v)| {
                let json_str = serde_json::to_string(&v).unwrap();
                let value_result = json(&json_str);
                match value_result {
                    Ok(value) => Some((k.clone(), value)),
                    Err(err) => {
                        let _ = parse_failure.insert(Err(err));
                        None
                    }
                }
            }).collect()
        });

        if let Some(failure) = parse_failure { return failure; }
        Ok(datastore.execute(query, database.session(), vars, false).await?)
    } else {
        Err(Error::Ds("Not set up".to_string()))
    }
}

#[tauri::command]
fn frontend_config(
    config: tauri::State<'_, YAMSFrontendConfig>
) -> YAMSFrontendConfig {
    (*config).clone()
}

fn main() {
    let builder = tauri::Builder::default();

    let builder = load_initial_state(builder);
    let builder = add_invoke_handlers(builder);

    builder.run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn add_invoke_handlers(builder: Builder<Wry>) -> Builder<Wry> {
    builder.invoke_handler(tauri::generate_handler![
        setup_database, check_database, query_database, frontend_config
    ])
}

fn load_initial_state(builder: Builder<Wry>) -> Builder<Wry> {
    let (backend_config, frontend_config) = YAMSFileConfig::load();

    let database = tauri::async_runtime::block_on(
        Database::setup(&backend_config)
    ).expect("was not able to set up SurrealDB");


    builder.manage(database)
        .manage(backend_config)
        .manage(frontend_config)
}
