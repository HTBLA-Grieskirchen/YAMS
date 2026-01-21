use std::sync::Arc;
use tauri::Manager;
use yams_core::context::YamsContext;
use yams_core::services::{AddressService, ClientService, AnimalService, RaceService, EventService, SeminarService};
use yams_persistence::adapter::SqliteAdapter;

mod config;
mod commands;

use crate::config::{YAMSFileConfig, YAMSFrontendConfig, YAMSBackendConfig};

#[tauri::command]
fn frontend_config(
    config: tauri::State<'_, YAMSFrontendConfig>,
) -> YAMSFrontendConfig {
    (*config).clone()
}

fn main() {
    let (backend_config, _frontend_config) = YAMSFileConfig::load();

    tauri::Builder::default()
        .setup(move |app| {
            // Initialize LibSQL Adapter using config
            let db_url = &backend_config.local_database_location;
            let adapter = tauri::async_runtime::block_on(async {
                SqliteAdapter::new(db_url).await
            }).expect("failed to initialize LibSQL adapter");
            let adapter = Arc::new(adapter);

            let ctx = Arc::new(YamsContext::new(
                adapter.clone(),
                adapter.clone(),
                adapter.clone(),
                adapter.clone(),
                adapter.clone(),
                adapter.clone(),
            ));

            let address_service = AddressService::new(ctx.clone());
            let client_service = ClientService::new(ctx.clone());
            let animal_service = AnimalService::new(ctx.clone());
            let race_service = RaceService::new(ctx.clone());
            let event_service = EventService::new(ctx.clone());
            let seminar_service = SeminarService::new(ctx.clone());

            app.manage(ctx);
            app.manage(address_service);
            app.manage(client_service);
            app.manage(animal_service);
            app.manage(race_service);
            app.manage(event_service);
            app.manage(seminar_service);
            
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
