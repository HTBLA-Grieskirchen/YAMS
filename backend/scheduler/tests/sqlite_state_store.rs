use std::sync::Arc;

use scheduler::{JobState, StateStore};
use yams_sqlite::SQLiteInstance;

use yams_scheduler::SQLiteStateStore;

#[tokio::test]
async fn scheduler_store_migrates_without_domain_schema() {
    let sqlite = Arc::new(SQLiteInstance::in_temp_dir().await.unwrap());
    let mut store = SQLiteStateStore::new(sqlite);
    store.migrate_to_latest().await.unwrap();

    let state = JobState {
        job_id: "tagesabschluss".into(),
        trigger_count: 1,
        last_run_at: None,
        last_success_at: None,
        next_run_at: None,
        last_error: None,
    };
    store.save(&state).await.unwrap();
    let loaded = store.load("tagesabschluss").await.unwrap().unwrap();
    assert_eq!(loaded.trigger_count, 1);
}
