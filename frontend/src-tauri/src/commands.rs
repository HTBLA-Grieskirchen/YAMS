use std::sync::Arc;
use tauri::State;
use yams_core::models::Address;
use yams_core::ports::AddressRepository;
use yams_persistence::adapter::SqliteAdapter;

#[tauri::command]
pub async fn get_addresses(
    adapter: State<'_, Arc<SqliteAdapter>>,
) -> Result<Vec<Address>, String> {
    AddressRepository::find_all(&**adapter).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_address(
    address: Address,
    adapter: State<'_, Arc<SqliteAdapter>>,
) -> Result<Address, String> {
    AddressRepository::save(&**adapter, address).await
        .map_err(|e| e.to_string())
}
