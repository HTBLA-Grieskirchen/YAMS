use yams_core::App;
use yams_persistence::SQLiteInstance;

use crate::cases::TestAdapters;

#[path = "../../yams-core/tests/cases/mod.rs"]
mod cases;

pub async fn make_testing_app(adapters: &TestAdapters) -> App {
    let mut sqlite_instance = SQLiteInstance::in_temp_dir().await.unwrap();
    sqlite_instance.migrate_to_latest().await.unwrap();

    App::builder()
        .uow_provider(Box::new(sqlite_instance))
        .maybe_clock(adapters.clock.clone())
        .build()
}
