use tauri::State;
use yams_api::YamsAppApi;
use yams_api::requests::HaustierErstellung;
use yams_api::schema::Haustier;

#[tauri::command]
pub async fn alle_haustiere(ctx: State<'_, YamsAppApi>) -> Result<Vec<Haustier>, String> {
    ctx.alle_haustiere().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn haustier_erstellen(
    erstellung: HaustierErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Haustier, String> {
    ctx.haustier_erstellen(erstellung)
        .await
        .map_err(|e| e.to_string())
}
