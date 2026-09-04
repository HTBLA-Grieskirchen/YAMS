//! Regression: parallel `execute_fn` must not fail opening connections with WAL
//! while another UoW is committing/closing (checkpoint / vacuum lock window).

use std::sync::Arc;

use chrono::Utc;
use yams_core::App;
use yams_core::domain::{Adresse, Ländercode, klient::NeuerKlient};
use yams_core::ports::RepositoryError;
use yams_persistence::SQLiteInstance;

async fn app() -> Arc<App> {
    let mut sqlite = SQLiteInstance::in_temp_dir().await.unwrap();
    sqlite.migrate_to_latest().await.unwrap();
    Arc::new(App::builder().uow_provider(Box::new(sqlite)).build())
}

fn neuer_klient(kundennummer: u64) -> NeuerKlient {
    NeuerKlient::neu(
        "Parallel".to_string(),
        "Race".to_string(),
        Utc::now().date_naive(),
        "parallel@test.com".try_into().unwrap(),
        "1234567890".try_into().unwrap(),
        kundennummer,
        false,
        Adresse {
            postleitzahl: "12345".into(),
            stadt: "Testcity".into(),
            straße_und_hausnummer: "Teststreet 1".into(),
            ländercode: Ländercode::from_str("DE").unwrap(),
        },
    )
}

#[test_log::test(pollster::test)]
async fn parallel_execute_fn_survives_wal_connection_init() {
    let app = app().await;

    // Pair a writer (commit + connection drop/checkpoint) with an opener that
    // immediately begins a new UoW. Pre-fix, each open re-ran
    // `PRAGMA journal_mode=WAL` and flaked with "database is locked".
    // Keep the loop modest: enough overlap to stress locks, not dominate CI time.
    let mut handles = Vec::with_capacity(128);
    for round in 0..64u64 {
        let writer_app = Arc::clone(&app);
        let opener_app = Arc::clone(&app);
        handles.push(std::thread::spawn(move || {
            pollster::block_on(async move {
                writer_app
                    .execute_fn::<_, _, RepositoryError>(async |ctx| {
                        let uow = ctx.enter().await?;
                        let result = uow.klienten().create(neuer_klient(10_000 + round)).await;
                        uow.finish(result, RepositoryError::OperationFailed).await
                    })
                    .await
                    .expect("writer execute_fn must not fail with database locked");
            })
        }));
        handles.push(std::thread::spawn(move || {
            pollster::block_on(async move {
                opener_app
                    .execute_fn::<_, _, RepositoryError>(async |ctx| {
                        let uow = ctx.enter().await?;
                        let result = uow.klienten().create(neuer_klient(50_000 + round)).await;
                        uow.finish(result, RepositoryError::OperationFailed).await
                    })
                    .await
                    .expect("opener execute_fn must not fail with database locked / WAL init");
            })
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
}
