use chrono::NaiveDate;
use rust_decimal::Decimal;

use super::support::{api, klient_erstellung};
use yams_api::{
    requests::{
        BehandlungErstellung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung,
        LeistungManuelleErstellung, ProduktErstellung, TagesabschlussErstellung,
    },
    schema::RechnungStatus,
};

#[pollster::test]
async fn produkt_erstellen_rejects_negative_preis() {
    let api = api().await;

    let err = api
        .produkt_erstellen(ProduktErstellung {
            name: "Futter".into(),
            beschreibung: "Premium Futter".into(),
            einzelpreis: Decimal::new(-1, 0),
            mwst_prozentsatz: Decimal::new(19, 0),
        })
        .await;

    assert!(err.is_err());
}

#[pollster::test]
async fn tagesabschluss_returns_rechnungen_through_api() {
    let api = api().await;
    let abschlussdatum = NaiveDate::from_ymd_opt(2026, 8, 23).unwrap();

    let klient1 = api.klient_erstellen(klient_erstellung(3001)).await.unwrap();
    let mut klient2_body = klient_erstellung(3002);
    klient2_body.vorname = "Bernd".into();
    klient2_body.nachname = "Test".into();
    klient2_body.email = "bernd@test.de".into();
    klient2_body.mobilnummer = "0987654321".into();
    let klient2 = api.klient_erstellen(klient2_body).await.unwrap();

    let produkt = api
        .produkt_erstellen(ProduktErstellung {
            name: "Futter".into(),
            beschreibung: "Premium Futter".into(),
            einzelpreis: Decimal::new(25, 0),
            mwst_prozentsatz: Decimal::new(19, 0),
        })
        .await
        .unwrap();
    let behandlung = api
        .behandlung_erstellen(BehandlungErstellung {
            name: "Untersuchung".into(),
            beschreibung: "Allgemeine Untersuchung".into(),
            standardpreis: Decimal::new(50, 0),
            mwst_prozentsatz: Decimal::new(19, 0),
        })
        .await
        .unwrap();

    let leistung_produkt = api
        .leistung_aus_produkt_buchen(LeistungAusProduktErstellung {
            produkt_id: produkt.id,
            klient_id: klient1.id,
            haustier_id: None,
            menge: Decimal::new(2, 0),
            leistungsdatum: abschlussdatum,
        })
        .await
        .unwrap();
    assert_eq!(leistung_produkt.betrag, Decimal::new(50, 0));

    let leistung_behandlung = api
        .leistung_aus_behandlung_buchen(LeistungAusBehandlungErstellung {
            behandlung_id: behandlung.id,
            klient_id: klient1.id,
            haustier_id: None,
            leistungsdatum: abschlussdatum,
            preis_override: None,
        })
        .await
        .unwrap();
    assert_eq!(leistung_behandlung.betrag, Decimal::new(50, 0));

    api.leistung_manuell_erfassen(LeistungManuelleErstellung {
        klient_id: klient2.id,
        haustier_id: None,
        beschreibung: "Beratung".into(),
        betrag: Decimal::new(30, 0),
        mwst_prozentsatz: Decimal::new(19, 0),
        leistungsdatum: abschlussdatum,
    })
    .await
    .unwrap();

    let rechnungen = api
        .tagesabschluss_durchführen(TagesabschlussErstellung {
            abschlussdatum: Some(abschlussdatum),
        })
        .await
        .unwrap();

    assert_eq!(rechnungen.len(), 2);

    let rechnung_klient1 = rechnungen
        .iter()
        .find(|r| r.klient_id == klient1.id)
        .expect("rechnung for klient1");
    let rechnung_klient2 = rechnungen
        .iter()
        .find(|r| r.klient_id == klient2.id)
        .expect("rechnung for klient2");

    assert_eq!(rechnung_klient1.positionen.len(), 2);
    assert_eq!(rechnung_klient1.gesamtbetrag_brutto, Decimal::new(119, 0));
    assert_eq!(rechnung_klient1.status, RechnungStatus::Offen);
    assert_eq!(rechnung_klient2.positionen.len(), 1);
    assert_eq!(rechnung_klient2.gesamtbetrag_brutto, Decimal::new(357, 1));

    let fetched = api.rechnungen_für_klient(klient1.id).await.unwrap();
    assert_eq!(fetched.len(), 1);
    assert_eq!(fetched[0].id, rechnung_klient1.id);
}
