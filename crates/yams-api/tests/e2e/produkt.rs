use poem::http::StatusCode;
use serde_json::json;

use super::{YamsApiTestClient, assert_status_ok, base_app_builder, json_decimal};
use rust_decimal::Decimal;

#[test_log::test(pollster::test)]
async fn produkt_erstellen_returns_mwst_ratio() {
    let api = YamsApiTestClient::new(base_app_builder().await);

    let (status, produkt) = api
        .post_json(
            "/api/produkt",
            json!({
                "name": "Futter",
                "beschreibung": "Premium Futter",
                "einzelpreis": "25.00",
                "mwst": "0.19",
            }),
        )
        .await;

    assert_status_ok(status);
    assert_eq!(json_decimal(&produkt["mwst"]), Decimal::new(19, 2));
}

#[test_log::test(pollster::test)]
async fn produkt_erstellen_rejects_negative_preis() {
    let api = YamsApiTestClient::new(base_app_builder().await);

    let (status, _) = api
        .post_json(
            "/api/produkt",
            json!({
                "name": "Futter",
                "beschreibung": "Premium Futter",
                "einzelpreis": "-1.00",
                "mwst": "0.19",
            }),
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[test_log::test(pollster::test)]
async fn produkt_erstellen_rejects_mwst_greater_than_one() {
    let api = YamsApiTestClient::new(base_app_builder().await);

    let (status, _) = api
        .post_json(
            "/api/produkt",
            json!({
                "name": "Futter",
                "beschreibung": "Premium Futter",
                "einzelpreis": "25.00",
                "mwst": "1.01",
            }),
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}
