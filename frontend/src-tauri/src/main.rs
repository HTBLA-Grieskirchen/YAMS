use error_stack::Report;
use tauri::Manager;
use yams_api::YamsAppApi;
use yams_core::{App, ports::RepositoryError};
use yams_persistence::SQLiteInstance;

mod commands;
mod config;

use crate::config::{YAMSFileConfig, YAMSFrontendConfig};

#[tauri::command]
fn frontend_config(config: tauri::State<'_, YAMSFrontendConfig>) -> YAMSFrontendConfig {
    (*config).clone()
}

fn main() {
    let (backend_config, _frontend_config) = YAMSFileConfig::load();

    tauri::Builder::default()
        .setup(move |tauri_app| {
            // Initialize LibSQL Adapter using config
            let db_url = &backend_config.local_database_location;
            let db_instance = tauri::async_runtime::block_on(async {
                let mut sqlite = SQLiteInstance::local(db_url).await?;
                sqlite.migrate_to_latest().await?;
                Ok::<_, Report<RepositoryError>>(sqlite)
            })
            .expect("failed to initialize LibSQL adapter");

            let app = App::builder().uow_provider(Box::new(db_instance)).build();
            let api = YamsAppApi::new(app);

            tauri_app.manage(api.inner_app());
            tauri_app.manage(api);

            tauri_app.manage(backend_config);
            tauri_app.manage(_frontend_config);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            frontend_config,
            commands::klient_erstellen,
            commands::haustier_erstellen,
            commands::alle_haustiere,
            commands::haustier_by_id,
            commands::produkt_erstellen,
            commands::behandlung_erstellen,
            commands::leistung_aus_produkt_buchen,
            commands::leistung_aus_behandlung_buchen,
            commands::leistung_manuell_erfassen,
            commands::tagesabschluss_durchführen,
            commands::rechnungen_für_klient,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
