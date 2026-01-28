use std::sync::Arc;
use tauri::Manager;
use yams_core::app::App;
use yams_core::service::{
    AddressService, AnimalService, ClientService, EventService, RaceService, SeminarService,
};
use yams_persistence::adapter::SqliteAdapter;

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
        .setup(move |app| {
            // Initialize LibSQL Adapter using config
            let db_url = &backend_config.local_database_location;
            let adapter =
                tauri::async_runtime::block_on(async { SqliteAdapter::new(db_url).await })
                    .expect("failed to initialize LibSQL adapter");
            let adapter = Arc::new(adapter);

            let address_service = Arc::new(AddressService::new(adapter.clone()));
            let client_service = Arc::new(ClientService::new(adapter.clone()));
            let animal_service = Arc::new(AnimalService::new(adapter.clone()));
            let race_service = Arc::new(RaceService::new(adapter.clone()));
            let event_service = Arc::new(EventService::new(adapter.clone()));
            let seminar_service = Arc::new(SeminarService::new(adapter.clone()));

            let ctx = Arc::new(App::new(
                address_service,
                client_service,
                animal_service,
                race_service,
                event_service,
                seminar_service,
            ));

            app.manage(ctx);

            app.manage(backend_config);
            app.manage(_frontend_config);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            frontend_config,
            commands::get_addresses,
            commands::create_address
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
