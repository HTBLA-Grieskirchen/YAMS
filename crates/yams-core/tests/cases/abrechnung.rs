use std::sync::Arc;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use yams_core::domain::{Adresse, Ländercode, Preis};
use yams_core::service::{
    BehandlungErstellen, KlientErstellen, LeistungAusBehandlungBuchen,
    LeistungAusProduktBuchen, LeistungManuellErfassen, ProduktErstellen,
    TagesabschlussDurchfuehren,
};

use super::super::base_app_builder;

#[pollster::test]
async fn test_tagesabschluss() {
    let app = Arc::new(base_app_builder().await.build());

    let klient1 = app
        .execute(KlientErstellen {
            vorname: "Anna".into(),
            nachname: "Muster".into(),
            geburtstag: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            email: "anna@muster.de".try_into().unwrap(),
            mobilnummer: "1234567890".try_into().unwrap(),
            kundennummer: 1001,
            einwilligung: true,
            adresse: Adresse {
                postleitzahl: "4711".into(),
                stadt: "Grieskirchen".into(),
                strasse_und_hausnummer: "Hauptstraße 1".into(),
                ländercode: Ländercode::new("DE").unwrap(),
            },
        })
        .await
        .unwrap();

    let klient2 = app
        .execute(KlientErstellen {
            vorname: "Bernd".into(),
            nachname: "Test".into(),
            geburtstag: NaiveDate::from_ymd_opt(1985, 5, 15).unwrap(),
            email: "bernd@test.de".try_into().unwrap(),
            mobilnummer: "0987654321".try_into().unwrap(),
            kundennummer: 1002,
            einwilligung: true,
            adresse: Adresse {
                postleitzahl: "4712".into(),
                stadt: "Grieskirchen".into(),
                strasse_und_hausnummer: "Nebenstraße 2".into(),
                ländercode: Ländercode::new("AT").unwrap(),
            },
        })
        .await
        .unwrap();

    let produkt = app
        .execute(ProduktErstellen {
            name: "Futter".into(),
            beschreibung: "Premium Futter".into(),
            einzelpreis: Preis::new(Decimal::new(25, 0)).unwrap(),
        })
        .await
        .unwrap();

    let behandlung = app
        .execute(BehandlungErstellen {
            name: "Untersuchung".into(),
            beschreibung: "Allgemeine Untersuchung".into(),
            standardpreis: Preis::new(Decimal::new(50, 0)).unwrap(),
        })
        .await
        .unwrap();

    let abschlussdatum = NaiveDate::from_ymd_opt(2026, 8, 23).unwrap();

    let klient1_id = klient1.id.clone();
    let klient2_id = klient2.id.clone();

    app.execute(LeistungAusProduktBuchen {
        produkt_id: produkt.id,
        klient_id: klient1_id.clone(),
        haustier_id: None,
        menge: Decimal::new(2, 0),
        leistungsdatum: abschlussdatum,
    })
        .await
        .unwrap();

    app.execute(LeistungAusBehandlungBuchen {
        behandlung_id: behandlung.id,
        klient_id: klient1_id.clone(),
        haustier_id: None,
        leistungsdatum: abschlussdatum,
    })
        .await
        .unwrap();

    app.execute(LeistungManuellErfassen {
        klient_id: klient2_id.clone(),
        haustier_id: None,
        beschreibung: "Beratung".into(),
        betrag: Preis::new(Decimal::new(30, 0)).unwrap(),
        leistungsdatum: abschlussdatum,
    })
        .await
        .unwrap();

    let rechnungen = app
        .execute(TagesabschlussDurchfuehren {
            abschlussdatum: Some(abschlussdatum),
        })
        .await
        .unwrap();

    assert_eq!(rechnungen.len(), 2);

    let rechnung_klient1 = rechnungen
        .iter()
        .find(|r| r.klient_id == klient1_id)
        .expect("rechnung for klient1");
    let rechnung_klient2 = rechnungen
        .iter()
        .find(|r| r.klient_id == klient2_id)
        .expect("rechnung for klient2");

    assert_eq!(rechnung_klient1.positionen.len(), 2);
    assert_eq!(rechnung_klient2.positionen.len(), 1);
    assert_eq!(
        rechnung_klient1.gesamtbetrag.value(),
        Decimal::new(100, 0) // 2×25 + 50
    );
    assert_eq!(rechnung_klient2.gesamtbetrag.value(), Decimal::new(30, 0));

    // Second Tagesabschluss should produce no new rechnungen
    let zweiter_abschluss = app
        .execute(TagesabschlussDurchfuehren {
            abschlussdatum: Some(abschlussdatum),
        })
        .await
        .unwrap();
    assert!(zweiter_abschluss.is_empty());
}
