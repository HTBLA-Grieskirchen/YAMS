use serde_json::json;

use super::support::{Api, assert_decimal_eq, assert_ok, assert_rejected, klient_body};

#[pollster::test]
async fn produkt_erstellen_rejects_negative_preis() {
    let api = Api::new().await;

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

    assert_rejected(status);
}

#[pollster::test]
async fn produkt_erstellen_rejects_mwst_greater_than_one() {
    let api = Api::new().await;

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

    assert_rejected(status);
}

#[pollster::test]
async fn tagesabschluss_returns_rechnungen_as_json() {
    let api = Api::new().await;
    let abschlussdatum = "2026-08-23";

    let (status, klient1) = api.post_json("/api/klient", klient_body(3001)).await;
    assert_ok(status);
    let mut klient2_body = klient_body(3002);
    klient2_body["vorname"] = json!("Bernd");
    klient2_body["nachname"] = json!("Test");
    klient2_body["email"] = json!("bernd@test.de");
    klient2_body["mobilnummer"] = json!("0987654321");
    let (status, klient2) = api.post_json("/api/klient", klient2_body).await;
    assert_ok(status);

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
    assert_ok(status);
    assert_decimal_eq(&produkt["mwst"], "0.19");

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
    assert_ok(status);

    let (status, leistung_produkt) = api
        .post_json(
            "/api/leistung/produkt",
            json!({
                "produktId": produkt["id"],
                "klientId": klient1["id"],
                "haustierId": null,
                "menge": "2",
                "leistungsdatum": abschlussdatum,
            }),
        )
        .await;
    assert_ok(status);
    assert_decimal_eq(&leistung_produkt["betrag"], "50");

    let (status, leistung_behandlung) = api
        .post_json(
            "/api/leistung/behandlung",
            json!({
                "behandlungId": behandlung["id"],
                "klientId": klient1["id"],
                "haustierId": null,
                "leistungsdatum": abschlussdatum,
                "preisOverride": null,
            }),
        )
        .await;
    assert_ok(status);
    assert_decimal_eq(&leistung_behandlung["betrag"], "50");

    let (status, _) = api
        .post_json(
            "/api/leistung/manuell",
            json!({
                "klientId": klient2["id"],
                "haustierId": null,
                "beschreibung": "Beratung",
                "betrag": "30.00",
                "mwst": "0.19",
                "leistungsdatum": abschlussdatum,
            }),
        )
        .await;
    assert_ok(status);

    let (status, rechnungen) = api
        .post_json(
            "/api/tagesabschluss",
            json!({ "abschlussdatum": abschlussdatum }),
        )
        .await;
    assert_ok(status);
    assert_eq!(rechnungen.as_array().unwrap().len(), 2);

    let rechnung_klient1 = rechnungen
        .as_array()
        .unwrap()
        .iter()
        .find(|rechnung| rechnung["klientId"] == klient1["id"])
        .expect("rechnung for klient1");
    let rechnung_klient2 = rechnungen
        .as_array()
        .unwrap()
        .iter()
        .find(|rechnung| rechnung["klientId"] == klient2["id"])
        .expect("rechnung for klient2");

    assert_eq!(rechnung_klient1["positionen"].as_array().unwrap().len(), 2);
    assert_decimal_eq(&rechnung_klient1["gesamtbetragBrutto"], "119");
    assert_eq!(rechnung_klient1["status"], "Offen");
    assert!(rechnung_klient1.get("mwstProzentsatz").is_none());
    assert_decimal_eq(&rechnung_klient1["positionen"][0]["mwst"], "0.19");
    assert!(rechnung_klient1["positionen"][0].get("stückzahl").is_some());

    assert_eq!(rechnung_klient2["positionen"].as_array().unwrap().len(), 1);
    assert_decimal_eq(&rechnung_klient2["gesamtbetragBrutto"], "35.7");

    let klient1_id = klient1["id"].as_str().unwrap();
    let (status, fetched) = api.get_json(&format!("/api/rechnung/{klient1_id}")).await;
    assert_ok(status);
    assert_eq!(fetched.as_array().unwrap().len(), 1);
    assert_eq!(fetched[0]["id"], rechnung_klient1["id"]);
}
