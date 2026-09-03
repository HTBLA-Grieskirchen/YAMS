use serde_json::json;

use super::{YamsApiTestClient, assert_status_ok, base_app_builder};

fn klient_body(kundennummer: u64) -> serde_json::Value {
    json!({
        "vorname": "Liste",
        "nachname": "Test",
        "geburtstag": "1990-01-01",
        "email": "liste@test.de",
        "mobilnummer": "1234567890",
        "kundennummer": kundennummer,
        "einwilligung": true,
        "adresse": {
            "postleitzahl": "4711",
            "stadt": "Grieskirchen",
            "straßeUndHausnummer": "Hauptstraße 1",
            "ländercode": "AT"
        }
    })
}

#[pollster::test]
async fn listen_endpoints_return_created_entities() {
    let api = YamsApiTestClient::new(base_app_builder().await);

    let (status, _) = api.post_json("/api/klient", klient_body(5001)).await;
    assert_status_ok(status);

    let (status, klienten) = api.get_json("/api/klient").await;
    assert_status_ok(status);
    assert_eq!(klienten.as_array().unwrap().len(), 1);

    let (status, produkte) = api.get_json("/api/produkt").await;
    assert_status_ok(status);
    assert!(produkte.as_array().unwrap().is_empty());

    let (status, rechnungen) = api.get_json("/api/rechnungen").await;
    assert_status_ok(status);
    assert!(rechnungen.as_array().unwrap().is_empty());
}
