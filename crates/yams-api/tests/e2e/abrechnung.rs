use rust_decimal::Decimal;
use serde_json::{Value, json};

use super::{
    YamsApiTestClient, assert_status_ok, base_app_builder, json_decimal,
};

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
async fn tagesabschluss_returns_rechnungen_as_json() {
    let api = YamsApiTestClient::new(base_app_builder().await.build());
    let abschlussdatum = "2026-08-23";

    let (status, klient1) = api.post_json("/api/klient", klient_body(3001)).await;
    assert_status_ok(status);
    let mut klient2_body = klient_body(3002);
    klient2_body["vorname"] = json!("Bernd");
    klient2_body["nachname"] = json!("Test");
    klient2_body["email"] = json!("bernd@test.de");
    klient2_body["mobilnummer"] = json!("0987654321");
    let (status, klient2) = api.post_json("/api/klient", klient2_body).await;
    assert_status_ok(status);

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
    assert_status_ok(status);
    assert_eq!(json_decimal(&leistung_produkt["betrag"]), Decimal::new(50, 0));

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
    assert_status_ok(status);
    assert_eq!(
        json_decimal(&leistung_behandlung["betrag"]),
        Decimal::new(50, 0)
    );

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
    assert_status_ok(status);

    let (status, rechnungen) = api
        .post_json(
            "/api/tagesabschluss",
            json!({ "abschlussdatum": abschlussdatum }),
        )
        .await;
    assert_status_ok(status);
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
    assert_eq!(
        json_decimal(&rechnung_klient1["gesamtbetragBrutto"]),
        Decimal::new(119, 0)
    );
    assert_eq!(rechnung_klient1["status"], "Offen");
    assert!(rechnung_klient1.get("mwstProzentsatz").is_none());
    assert_eq!(
        json_decimal(&rechnung_klient1["positionen"][0]["mwst"]),
        Decimal::new(19, 2)
    );
    assert!(rechnung_klient1["positionen"][0].get("stückzahl").is_some());

    assert_eq!(rechnung_klient2["positionen"].as_array().unwrap().len(), 1);
    assert_eq!(
        json_decimal(&rechnung_klient2["gesamtbetragBrutto"]),
        Decimal::new(357, 1)
    );

    let klient1_id = klient1["id"].as_str().unwrap();
    let (status, fetched) = api.get_json(&format!("/api/rechnung/{klient1_id}")).await;
    assert_status_ok(status);
    assert_eq!(fetched.as_array().unwrap().len(), 1);
    assert_eq!(fetched[0]["id"], rechnung_klient1["id"]);
}
