use serde_json::json;

use super::support::{Api, assert_ok, assert_rejected, klient_body};

#[pollster::test]
async fn klient_erstellen_returns_camelcase_utf8_json() {
    let api = Api::new().await;

    let (status, body) = api.post_json("/api/klient", klient_body(1001)).await;

    assert_ok(status);
    assert_eq!(body["vorname"], "Anna");
    assert_eq!(body["nachname"], "Muster");
    assert_eq!(body["kundennummer"], 1001);
    assert_eq!(body["adresse"]["postleitzahl"], "4711");
    assert_eq!(body["adresse"]["ländercode"], "DE");
    assert_eq!(body["adresse"]["straßeUndHausnummer"], "Hauptstraße 1");
    assert!(body["haustiere"].as_array().unwrap().is_empty());
    assert!(body.get("straßeUndHausnummer").is_none());
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_email() {
    let api = Api::new().await;
    let mut body = klient_body(1001);
    body["email"] = json!("not-an-email");

    let (status, _) = api.post_json("/api/klient", body).await;
    assert_rejected(status);
}

#[pollster::test]
async fn klient_erstellen_rejects_empty_name() {
    let api = Api::new().await;
    let mut body = klient_body(1001);
    body["vorname"] = json!("");

    let (status, _) = api.post_json("/api/klient", body).await;
    assert_rejected(status);
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_ländercode() {
    let api = Api::new().await;
    let mut body = klient_body(1001);
    body["adresse"]["ländercode"] = json!("US");

    let (status, _) = api.post_json("/api/klient", body).await;
    assert_rejected(status);
}
