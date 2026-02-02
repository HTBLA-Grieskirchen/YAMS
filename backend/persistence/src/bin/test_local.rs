use yams_persistence::SQLiteInstance;

use pollster;

#[pollster::main]
async fn main() {
    let mut instance = SQLiteInstance::local("test.db").await.unwrap();
    instance.migrate_to_latest().await.unwrap();
}