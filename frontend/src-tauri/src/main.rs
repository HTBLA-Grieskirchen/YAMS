use std::sync::Arc;
use yams_core::context::YamsContext;
use yams_core::services::AddressService;
use yams_persistence::adapter::SqliteAdapter;

mod config;
mod database;
mod commands;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize LibSQL Adapter
            let adapter = tauri::async_runtime::block_on(async {
                SqliteAdapter::new("yams.db").await
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

            app.manage(ctx);
            app.manage(address_service);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_addresses,
            commands::create_address
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
