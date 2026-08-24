use chrono::NaiveDate;

use super::support::{api, klient_erstellung};
use yams_api::requests::HaustierErstellung;

#[pollster::test]
async fn haustier_erstellen_is_listed_and_fetchable() {
    let api = api().await;
    let klient = api.klient_erstellen(klient_erstellung(2001)).await.unwrap();

    let haustier = api
        .haustier_erstellen(HaustierErstellung {
            name: "Bello".into(),
            geburtstag: NaiveDate::from_ymd_opt(2020, 6, 15).unwrap(),
            tierart: "Hund".into(),
            beschreibung: "Mischling".into(),
            klient_id: klient.id,
        })
        .await
        .unwrap();

    assert_eq!(haustier.name, "Bello");
    assert_eq!(haustier.klient_id, klient.id);

    let by_id = api.haustier_by_id(haustier.id).await.unwrap();
    assert_eq!(by_id.id, haustier.id);

    let alle = api.alle_haustiere().await.unwrap();
    assert_eq!(alle.len(), 1);
    assert_eq!(alle[0].id, haustier.id);
}
