use serde_json::json;

use super::support::{Api, assert_ok, klient_body};

#[pollster::test]
async fn haustier_erstellen_is_listed_and_fetchable() {
    let api = Api::new().await;
    let (status, klient) = api.post_json("/api/klient", klient_body(2001)).await;
    assert_ok(status);
    let klient_id = klient["id"].as_str().unwrap();

    let (status, haustier) = api
        .post_json(
            "/api/haustier",
            json!({
                "name": "Bello",
                "geburtstag": "2020-06-15",
                "tierart": "Hund",
                "beschreibung": "Mischling",
                "klientId": klient_id,
            }),
        )
        .await;
    assert_ok(status);
    assert_eq!(haustier["name"], "Bello");
    assert_eq!(haustier["klientId"], klient_id);

    let haustier_id = haustier["id"].as_str().unwrap();
    let (status, by_id) = api.get_json(&format!("/api/haustier/{haustier_id}")).await;
    assert_ok(status);
    assert_eq!(by_id["id"], haustier_id);

    let (status, alle) = api.get_json("/api/haustier").await;
    assert_ok(status);
    assert_eq!(alle.as_array().unwrap().len(), 1);
    assert_eq!(alle[0]["id"], haustier_id);
}
