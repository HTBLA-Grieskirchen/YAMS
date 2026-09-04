use poem::http::StatusCode;
use rust_decimal::Decimal;
use serde_json::{Value, json};

use super::{YamsApiTestClient, assert_status_ok, base_app_builder, json_decimal};

fn klient_body(kundennummer: u64, email: &str) -> Value {
    json!({
        "vorname": "Anna",
        "nachname": "Muster",
        "geburtstag": "1990-01-01",
        "email": email,
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

fn seminar_body() -> Value {
    json!({
        "titel": "Hufseminar",
        "beschreibung": "Einführung",
        "teilnahmegebührBasis": "100.00",
        "mwst": "0.20",
        "standarddauerMs": null
    })
}

fn termin_body(seminar_id: &Value) -> Value {
    json!({
        "seminarId": seminar_id,
        "beginn": "2026-08-25T10:00:00Z",
        "ende": "2026-08-25T16:00:00Z",
        "ort": { "ortName": "Hof", "adresse": null },
        "maxTeilnehmer": 8
    })
}

#[test_log::test(pollster::test)]
async fn seminar_flow_books_holds_and_forecasts() {
    let api = YamsApiTestClient::new(base_app_builder().await);

    let (status, seminar) = api.post_json("/api/seminar", seminar_body()).await;
    assert_status_ok(status);
    assert_eq!(seminar["titel"], "Hufseminar");

    let (status, fetched) = api
        .get_json(&format!("/api/seminar/{}", seminar["id"].as_str().unwrap()))
        .await;
    assert_status_ok(status);
    assert_eq!(fetched["id"], seminar["id"]);

    let (status, klient) = api
        .post_json("/api/klient", klient_body(4001, "anna@seminar.de"))
        .await;
    assert_status_ok(status);

    let (status, termin) = api
        .post_json("/api/seminar-termin", termin_body(&seminar["id"]))
        .await;
    assert_status_ok(status);
    assert_eq!(termin["status"], "Geplant");

    let (status, gebucht) = api
        .post_json(
            &format!(
                "/api/seminar-termin/{}/buchung",
                termin["id"].as_str().unwrap()
            ),
            json!({
                "klientId": klient["id"],
                "rabatt": "0.20"
            }),
        )
        .await;
    assert_status_ok(status);
    assert_eq!(gebucht["buchungen"].as_array().unwrap().len(), 1);

    let (status, umsatz) = api
        .get_json(&format!(
            "/api/seminar-termin/{}/umsatz",
            termin["id"].as_str().unwrap()
        ))
        .await;
    assert_status_ok(status);
    assert_eq!(json_decimal(&umsatz["gesamtNetto"]), Decimal::new(80, 0));
    assert_eq!(json_decimal(&umsatz["gesamtBrutto"]), Decimal::new(96, 0));

    let (status, prognose) = api
        .get_json("/api/seminar-prognose?stichtag=2026-08-25")
        .await;
    assert_status_ok(status);
    assert_eq!(prognose["termine"].as_array().unwrap().len(), 1);

    let (status, abgehalten) = api
        .post(&format!(
            "/api/seminar-termin/{}/abgehalten",
            termin["id"].as_str().unwrap()
        ))
        .await;
    assert_status_ok(status);
    assert_eq!(abgehalten["status"], "Abgehalten");
    assert!(abgehalten["buchungen"][0]["leistungId"].is_string());

    let buchung_id = abgehalten["buchungen"][0]["id"].as_str().unwrap();
    let termin_id = termin["id"].as_str().unwrap();
    let (status, pdf, content_type) = api
        .get_bytes(&format!(
            "/api/seminar-termin/{termin_id}/buchung/{buchung_id}/teilnahmebestätigung"
        ))
        .await;
    assert_status_ok(status);
    assert!(content_type.contains("application/pdf"));
    assert!(pdf.starts_with(b"%PDF"));
    assert!(
        pdf.len() > 200,
        "expected Typst PDF, got {} bytes",
        pdf.len()
    );
}

#[test_log::test(pollster::test)]
async fn seminar_rejects_empty_titel() {
    let api = YamsApiTestClient::new(base_app_builder().await);
    let (status, _) = api
        .post_json(
            "/api/seminar",
            json!({
                "titel": "  ",
                "beschreibung": "x",
                "teilnahmegebührBasis": "10.00",
                "mwst": "0",
                "standarddauerMs": null
            }),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[test_log::test(pollster::test)]
async fn absage_blocks_abgehalten_via_api() {
    let api = YamsApiTestClient::new(base_app_builder().await);
    let (status, seminar) = api.post_json("/api/seminar", seminar_body()).await;
    assert_status_ok(status);
    let (status, termin) = api
        .post_json("/api/seminar-termin", termin_body(&seminar["id"]))
        .await;
    assert_status_ok(status);

    let (status, _) = api
        .post_json(
            &format!(
                "/api/seminar-termin/{}/absagen",
                termin["id"].as_str().unwrap()
            ),
            json!({ "grund": "zu wenig tn" }),
        )
        .await;
    assert_status_ok(status);

    let (status, _) = api
        .post(&format!(
            "/api/seminar-termin/{}/abgehalten",
            termin["id"].as_str().unwrap()
        ))
        .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[test_log::test(pollster::test)]
async fn unknown_seminar_is_not_found() {
    let api = YamsApiTestClient::new(base_app_builder().await);
    let (status, _) = api
        .get_json("/api/seminar/00000000-0000-0000-0000-000000000000")
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
