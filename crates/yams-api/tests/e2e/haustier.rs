use poem::http::StatusCode;
use serde_json::{Value, json};

use super::{YamsApiTestClient, assert_status_ok, base_app_builder};

fn klient_body(kundennummer: u64) -> Value {
    json!({
        "vorname": "Anna",
        "nachname": "Muster",
        "geburtstag": "1990-01-01",
        "email": "anna@muster.de",
        "mobilnummer": "1234567890",
        "kundennummer": kundennummer,
        "einwilligung": true,
        "adresse": {
            "postleitzahl": "4711",
            "stadt": "Grieskirchen",
            "straßeUndHausnummer": "Hauptstraße 1",
            "ländercode": "DE"
        }
    })
}

#[test_log::test(pollster::test)]
async fn haustier_erstellen_is_listed_and_fetchable() {
    let api = YamsApiTestClient::new(base_app_builder().await);
    let (status, klient) = api.post_json("/api/klient", klient_body(2001)).await;
    assert_status_ok(status);
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
    assert_status_ok(status);
    assert_eq!(haustier["name"], "Bello");
    assert_eq!(haustier["klientId"], klient_id);

    let haustier_id = haustier["id"].as_str().unwrap();
    let (status, by_id) = api.get_json(&format!("/api/haustier/{haustier_id}")).await;
    assert_status_ok(status);
    assert_eq!(by_id["id"], haustier_id);

    let (status, alle) = api.get_json("/api/haustier").await;
    assert_status_ok(status);
    assert_eq!(alle.as_array().unwrap().len(), 1);
    assert_eq!(alle[0]["id"], haustier_id);
}

#[test_log::test(pollster::test)]
async fn haustier_erstellen_unknown_klient_is_not_found() {
    let api = YamsApiTestClient::new(base_app_builder().await);
    let (status, _) = api
        .post_json(
            "/api/haustier",
            json!({
                "name": "Bello",
                "geburtstag": "2020-06-15",
                "tierart": "Hund",
                "beschreibung": "Mischling",
                "klientId": "00000000-0000-0000-0000-000000000000",
            }),
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
