use super::support::{api, klient_erstellung};
use yams_api::schema::Ländercode;

#[pollster::test]
async fn klient_erstellen_returns_schema_fields() {
    let api = api().await;

    let klient = api.klient_erstellen(klient_erstellung(1001)).await.unwrap();

    assert_eq!(klient.vorname, "Anna");
    assert_eq!(klient.nachname, "Muster");
    assert_eq!(klient.kundennummer, 1001);
    assert_eq!(klient.adresse.postleitzahl, "4711");
    assert_eq!(klient.adresse.ländercode.0, "DE");
    assert!(klient.haustiere.is_empty());
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_email() {
    let api = api().await;
    let mut body = klient_erstellung(1001);
    body.email = "not-an-email".into();

    assert!(api.klient_erstellen(body).await.is_err());
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_mobilnummer() {
    let api = api().await;
    let mut body = klient_erstellung(1001);
    body.mobilnummer = "123".into();

    assert!(api.klient_erstellen(body).await.is_err());
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_ländercode() {
    let api = api().await;
    let mut body = klient_erstellung(1001);
    body.adresse.ländercode = Ländercode("US".into());

    assert!(api.klient_erstellen(body).await.is_err());
}
