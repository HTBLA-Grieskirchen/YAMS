use std::str::FromStr;

use poem::{Route, http::StatusCode, test::TestClient};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use yams_api::openapi_service;
use yams_core::App;
use yams_persistence::SQLiteInstance;

pub struct Api {
    client: TestClient<Route>,
}

impl Api {
    pub async fn new() -> Self {
        let mut sqlite = SQLiteInstance::in_temp_dir().await.unwrap();
        sqlite.migrate_to_latest().await.unwrap();

        let app = App::builder().uow_provider(Box::new(sqlite)).build();
        let service = openapi_service(app, std::iter::empty::<String>());
        let route = Route::new().nest("/api", service);
        Self {
            client: TestClient::new(route),
        }
    }

    pub async fn post_json(&self, path: &str, body: Value) -> (StatusCode, Value) {
        let resp = self.client.post(path).body_json(&body).send().await;
        json_response(resp).await
    }

    pub async fn get_json(&self, path: &str) -> (StatusCode, Value) {
        let resp = self.client.get(path).send().await;
        json_response(resp).await
    }
}

async fn json_response(resp: poem::test::TestResponse) -> (StatusCode, Value) {
    let status = resp.0.status();
    let content_type = resp
        .0
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string();
    let body = resp.0.into_body().into_string().await.unwrap_or_default();
    if status.is_success() {
        assert!(
            content_type.contains("application/json"),
            "expected JSON content type, got {content_type:?} for {status}"
        );
        let value: Value = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("invalid JSON for {status}: {err}; body={body}"));
        (status, value)
    } else if let Ok(value) = serde_json::from_str(&body) {
        (status, value)
    } else {
        (status, Value::String(body))
    }
}

pub fn klient_body(kundennummer: u64) -> Value {
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

pub fn assert_ok(status: StatusCode) {
    assert_eq!(status, StatusCode::OK, "expected 200 OK, got {status}");
}

pub fn assert_rejected(status: StatusCode) {
    assert!(
        !status.is_success(),
        "expected a rejected HTTP status, got {status}"
    );
}

pub fn json_decimal(value: &Value) -> Decimal {
    match value {
        Value::String(text) => Decimal::from_str(text).expect("decimal string"),
        Value::Number(number) => Decimal::from_str(&number.to_string()).expect("decimal number"),
        other => panic!("expected decimal JSON, got {other}"),
    }
}

pub fn assert_decimal_eq(value: &Value, expected: &str) {
    assert_eq!(json_decimal(value), Decimal::from_str(expected).unwrap());
}
