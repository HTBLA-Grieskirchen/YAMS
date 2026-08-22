use yams_core::{App, application::{AppBuilder, SetUowProvider}};
use yams_persistence::SQLiteInstance;

#[path = "../../yams-core/tests/cases/mod.rs"]
mod cases;

pub async fn base_app_builder() -> AppBuilder<SetUowProvider> {
    let mut sqlite_instance = SQLiteInstance::in_temp_dir().await.unwrap();
    sqlite_instance.migrate_to_latest().await.unwrap();

    App::builder()
        .uow_provider(Box::new(sqlite_instance))
}
