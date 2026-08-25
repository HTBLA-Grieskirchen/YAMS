use serde_json::{Value, json};

use super::{YamsApiTestClient, assert_status_ok, assert_status_rejected, base_app_builder};

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

#[pollster::test]
async fn klient_erstellen_returns_camelcase_utf8_json() {
    let api = YamsApiTestClient::new(base_app_builder().await.build());

    let (status, body) = api.post_json("/api/klient", klient_body(1001)).await;

    assert_status_ok(status);
    assert_eq!(body["vorname"], "Anna");
    assert_eq!(body["nachname"], "Muster");
    assert_eq!(body["email"], "anna@muster.de");
    assert_eq!(body["mobilnummer"], "1234567890");
    assert_eq!(body["kundennummer"], 1001);
    assert_eq!(body["adresse"]["postleitzahl"], "4711");
    assert_eq!(body["adresse"]["ländercode"], "DE");
    assert_eq!(body["adresse"]["straßeUndHausnummer"], "Hauptstraße 1");
    assert!(body["haustiere"].as_array().unwrap().is_empty());
    assert!(body.get("straßeUndHausnummer").is_none());
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_email() {
    let api = YamsApiTestClient::new(base_app_builder().await.build());
    let mut body = klient_body(1001);
    body["email"] = json!("not-an-email");

    let (status, _) = api.post_json("/api/klient", body).await;
    assert_status_rejected(status);
}

#[pollster::test]
async fn klient_erstellen_rejects_empty_name() {
    let api = YamsApiTestClient::new(base_app_builder().await.build());
    let mut body = klient_body(1001);
    body["vorname"] = json!("");

    let (status, _) = api.post_json("/api/klient", body).await;
    assert_status_rejected(status);
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_ländercode() {
    let api = YamsApiTestClient::new(base_app_builder().await.build());
    let mut body = klient_body(1001);
    body["adresse"]["ländercode"] = json!("US");

    let (status, _) = api.post_json("/api/klient", body).await;
    assert_status_rejected(status);
}
