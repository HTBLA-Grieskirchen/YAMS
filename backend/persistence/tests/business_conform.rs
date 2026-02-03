use yams_core::{App, application::AppConfiguration};
use yams_persistence::SQLiteInstance;

use crate::cases::TestAdapters;

#[path = "../../core/tests/cases/mod.rs"]
mod cases;

pub async fn make_testing_app() -> (App, TestAdapters) {
    let mut sqlite_instance = SQLiteInstance::in_memory().await.unwrap();
    sqlite_instance.migrate_to_latest().await;

    (
        App {
            uow_provider: Box::new(sqlite_instance),
            configuration: AppConfiguration {
                ..Default::default()
            },
        },
        TestAdapters {},
    )
}
