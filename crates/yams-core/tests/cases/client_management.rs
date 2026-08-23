use std::sync::Arc;

use super::super::base_app_builder;
use chrono::Utc;
use yams_core::domain::{Adresse, Ländercode};
use yams_core::ports::RepositoryError;
use yams_core::service::KlientErstellen;

#[pollster::test]
async fn test_klient() {
    let app = base_app_builder().await.build();
    let app = Arc::new(app);

    let case = KlientErstellen {
        vorname: "Testname".into(),
        nachname: "Testname Last".into(),
        geburtstag: Utc::now().date_naive(),
        email: "test@test.com".try_into().unwrap(),
        mobilnummer: "1234567890".try_into().unwrap(),
        kundennummer: 1234567890,
        einwilligung: false,
        adresse: Adresse {
            postleitzahl: "12345".into(),
            stadt: "Testcity".into(),
            straße_und_hausnummer: "Teststreet 12".into(),
            ländercode: Ländercode::from_str("DE").unwrap(),
        },
    };

    let handles: Vec<_> = (0..100)
        .map(|i| {
            let app_clone = app.clone();
            let mut case_clone = case.clone();
            case_clone.kundennummer = 1000 + i;
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1));
                pollster::block_on(async move { app_clone.execute(case_clone).await.unwrap() })
            })
        })
        .collect();

    let result = app.execute(case).await.unwrap();
    let parallel_results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    assert_eq!(parallel_results.len(), 100);

    let klienten = app
        .execute_fn::<_, _, RepositoryError>(async |ctx| {
            let mut klienten = vec![ctx.uow.klienten().find_by_id(result.id).await?];
            for res in parallel_results {
                klienten.push(ctx.uow.klienten().find_by_id(res.id).await?);
            }
            Ok(klienten)
        })
        .await
        .unwrap();
    assert_eq!(klienten.len(), 101);
    assert_eq!(klienten[0].vorname, "Testname");
}
