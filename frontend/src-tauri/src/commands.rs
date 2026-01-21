use tauri::State;
use yams_core::models::{Address, NewAddress};
use yams_core::services::AddressService;

#[tauri::command]
pub async fn get_addresses(
    service: State<'_, AddressService>,
) -> Result<Vec<Address>, String> {
    service.get_all().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_address(
    address: NewAddress,
    service: State<'_, AddressService>,
) -> Result<Address, String> {
    service.create(address).await
        .map_err(|e| e.to_string())
}
