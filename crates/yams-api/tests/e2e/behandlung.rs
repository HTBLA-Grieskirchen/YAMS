use poem::http::StatusCode;
use serde_json::json;

use super::{YamsApiTestClient, assert_status_ok, base_app_builder, json_decimal};
use rust_decimal::Decimal;

#[pollster::test]
async fn behandlung_erstellen_returns_mwst_ratio() {
    let api = YamsApiTestClient::new(base_app_builder().await);

    let (status, behandlung) = api
        .post_json(
            "/api/behandlung",
            json!({
                "name": "Untersuchung",
                "beschreibung": "Allgemeine Untersuchung",
                "standardpreis": "50.00",
                "mwst": "0.19",
            }),
        )
        .await;

    assert_status_ok(status);
    assert_eq!(json_decimal(&behandlung["mwst"]), Decimal::new(19, 2));
}

#[pollster::test]
async fn behandlung_erstellen_rejects_empty_name() {
    let api = YamsApiTestClient::new(base_app_builder().await);

    let (status, _) = api
        .post_json(
            "/api/behandlung",
            json!({
                "name": "",
                "beschreibung": "Allgemeine Untersuchung",
                "standardpreis": "50.00",
                "mwst": "0.19",
            }),
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}
