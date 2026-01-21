#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::BTreeMap as Map;

use surrealdb::{Error, Response};
use surrealdb::sql::json;
use tauri::Manager;

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
    if check_database(database.clone()).await.unwrap_or(false) {
        return Ok(());
    }

    let mut database_lock = database.lock().await;

    // Inject new datastore in database
    let _ = database_lock.insert(
        Database::new_datastore(&config.local_database_location, database.session()).await?
    );

    Ok(())
}

#[tauri::command]
async fn check_database(
    database: tauri::State<'_, Database>
) -> Result<bool, ()> {
    if database.lock().await.is_some() {
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
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let (backend_config, frontend_config) = YAMSFileConfig::load();

            let database = tauri::async_runtime::block_on(
                Database::setup(&backend_config)
            ).expect("was not able to set up SurrealDB");

            app.manage(database);
            app.manage(backend_config);
            app.manage(frontend_config);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            setup_database, check_database, query_database, frontend_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
