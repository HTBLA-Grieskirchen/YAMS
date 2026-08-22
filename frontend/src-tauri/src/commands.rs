use std::sync::Arc;
use tauri::State;
use yams_api::YamsAppApi;
use yams_api::requests::AnimalCreation;
use yams_api::schema::Animal;
use yams_core::App;

#[tauri::command]
pub async fn get_animals(ctx: State<'_, YamsAppApi>) -> Result<Vec<Animal>, String> {
    ctx.get_all_animals().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_animal(
    creation: AnimalCreation,
    ctx: State<'_, YamsAppApi>,
) -> Result<Animal, String> {
    ctx.create_animal(creation).await.map_err(|e| e.to_string())
}
