#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use directories::ProjectDirs;
use include_path::include_path_str;
use surrealdb::{Datastore, Error, Response, Session};
use surrealdb::sql::{Object, Value};
use tauri::async_runtime::Mutex;

#[derive(Default)]
struct Database(Arc<Mutex<Option<Datastore>>>);

impl Deref for Database {
    type Target = Arc<Mutex<Option<Datastore>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[tauri::command]
async fn setup_database(
    database: tauri::State<'_, Database>,
    session: tauri::State<'_, Session>,
    dirs: tauri::State<'_, ProjectDirs>,
) -> Result<(), Error> {
    // Don't setup database if already exists
    let mut database_lock = if check_database(tauri::State::clone(&database)).await.unwrap_or(false) {
        return Ok(());
    } else {
        database.lock().await
    };

    // Determine storage location based on env
    let in_dev: &str = &std::env::var("YAMS_DEV").unwrap_or("0".to_string());
    let location = match in_dev {
        "1" => format!("file:{}", ["..", "..", "backend", "surreal_data"]
            .iter().collect::<PathBuf>().to_str().unwrap()),
        _ => format!("file:{}", dirs.data_dir().join("surreal_data").to_str().unwrap())
    };

    // Create actual datastore
    let datastore = Datastore::new(&location).await?;
    // Set datastore
    let datastore = database_lock.insert(datastore);

    // Load sql script if datastore is new
    let mut import = true;
    let result = datastore.execute("INFO FOR DB", &session, None, false).await?;
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
        let sql_script = include_path_str!("..", "..", "..", "backend", "setup.sql");
        println!("Import data scheme into SurrealDB");
        datastore.execute(sql_script, &session, None, false).await?;
    };
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
    database: tauri::State<'_, Database>,
    session: tauri::State<'_, Session>,
) -> Result<Vec<Response>, Error> {
    if let Some(datastore) = &*database.lock().await {
        Ok(datastore.execute(query, &session, None, false).await?)
    } else {
        Err(Error::Ds("Not set up".to_string()))
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Database::default())
        .manage(Session::for_db("yams", "yams"))
        .manage(ProjectDirs::from("at", "HTL Grieskirchen", "YAMS")
            .expect("unsupported OS"))
        .invoke_handler(tauri::generate_handler![setup_database, check_database, query_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
