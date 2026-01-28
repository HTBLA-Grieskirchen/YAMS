use std::sync::Arc;
use tauri::State;
use yams_core::app::App;
use yams_core::models::{Address, NewAddress};

#[tauri::command]
pub async fn get_addresses(ctx: State<'_, Arc<App>>) -> Result<Vec<Address>, String> {
    ctx.address_service
        .get_all()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_address(
    address: NewAddress,
    ctx: State<'_, Arc<App>>,
) -> Result<Address, String> {
    ctx.address_service
        .create(address)
        .await
        .map_err(|e| e.to_string())
}
