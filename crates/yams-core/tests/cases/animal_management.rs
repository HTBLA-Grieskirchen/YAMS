use std::sync::Arc;

use super::super::base_app_builder;
use chrono::Utc;
use yams_core::{
    domain::{Adresse, Ländercode},
    service::{HaustierErstellen, KlientErstellen, VieleHaustiereErstellen},
};

#[pollster::test]
async fn test_haustier() {
    let app = base_app_builder().await.build();
    let app = Arc::new(app);

    let create_klient = KlientErstellen {
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
            strasse_und_hausnummer: "Teststreet 12".into(),
            ländercode: Ländercode::from_str("DE").unwrap(),
        },
    };

    let klient = app.execute(create_klient).await.unwrap();
    let haustier_amount = 10;

    let results: Vec<_> = (0..haustier_amount)
        .map(|i| {
            let app_clone = app.clone();
            let klient_id = klient.id.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1));
                pollster::block_on(async move {
                    let add_haustier = HaustierErstellen {
                        klient_id,
                        name: format!("Testhaustier {}", i).into(),
                        geburtstag: Utc::now().date_naive(),
                        tierart: "Testspecies".into(),
                        beschreibung: "Testdescription".into(),
                    };
                    app_clone.execute(add_haustier).await.unwrap()
                })
            })
        })
        .map(|handle| handle.join().unwrap())
        .collect();

    assert_eq!(results.len(), haustier_amount);

    let cmd = VieleHaustiereErstellen {
        haustiere: (0..haustier_amount)
            .map(|i| HaustierErstellen {
                klient_id: klient.id.clone(),
                name: format!("Testhaustier {}", i).into(),
                geburtstag: Utc::now().date_naive(),
                tierart: "Testspecies".into(),
                beschreibung: "Testdescription".into(),
            })
            .collect(),
    };

    let batch_results = app.execute(cmd).await.unwrap();
    assert_eq!(batch_results.len(), 10);

    let haustiere = app
        .execute_fn(async |ctx| ctx.uow.haustiere().find_by_klient_id(klient.id).await)
        .await
        .unwrap();
    assert_eq!(haustiere.len(), haustier_amount * 2);
}
