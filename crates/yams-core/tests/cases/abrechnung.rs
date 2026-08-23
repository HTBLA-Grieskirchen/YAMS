use std::sync::Arc;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use yams_core::domain::{Adresse, Klient, Ländercode, Preis};
use yams_core::service::{
    BehandlungErstellen, KlientErstellen, LeistungAusBehandlungBuchen,
    LeistungAusProduktBuchen, LeistungManuellErfassen, ProduktErstellen,
    TagesabschlussDurchfuehren,
};

use super::super::base_app_builder;

struct AbrechnungSetup {
    app: Arc<yams_core::App>,
    klient1: Klient,
    klient2: Klient,
    abschlussdatum: NaiveDate,
}

async fn setup_abrechnung_fixture() -> AbrechnungSetup {
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
                ländercode: Ländercode::from_str("DE").unwrap(),
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
                ländercode: Ländercode::from_str("AT").unwrap(),
            },
        })
        .await
        .unwrap();

    let produkt = app
        .execute(ProduktErstellen {
            name: "Futter".into(),
            beschreibung: "Premium Futter".into(),
            einzelpreis: Preis::new(Decimal::new(25, 0)).unwrap(),
            mwst_prozentsatz: Decimal::new(19, 0),
        })
        .await
        .unwrap();

    let behandlung = app
        .execute(BehandlungErstellen {
            name: "Untersuchung".into(),
            beschreibung: "Allgemeine Untersuchung".into(),
            standardpreis: Preis::new(Decimal::new(50, 0)).unwrap(),
            mwst_prozentsatz: Decimal::new(19, 0),
        })
        .await
        .unwrap();

    let abschlussdatum = NaiveDate::from_ymd_opt(2026, 8, 23).unwrap();

    app.execute(LeistungAusProduktBuchen {
        produkt_id: produkt.id,
        klient_id: klient1.id.clone(),
        haustier_id: None,
        menge: Decimal::new(2, 0),
        leistungsdatum: abschlussdatum,
    })
    .await
    .unwrap();

    app.execute(LeistungAusBehandlungBuchen {
        behandlung_id: behandlung.id,
        klient_id: klient1.id.clone(),
        haustier_id: None,
        leistungsdatum: abschlussdatum,
        preis_override: None,
    })
    .await
    .unwrap();

    app.execute(LeistungManuellErfassen {
        klient_id: klient2.id.clone(),
        haustier_id: None,
        beschreibung: "Beratung".into(),
        betrag: Preis::new(Decimal::new(30, 0)).unwrap(),
        mwst_prozentsatz: Decimal::new(19, 0),
        leistungsdatum: abschlussdatum,
    })
    .await
    .unwrap();

    AbrechnungSetup {
        app,
        klient1,
        klient2,
        abschlussdatum,
    }
}

#[pollster::test]
async fn test_tagesabschluss() {
    let setup = setup_abrechnung_fixture().await;

    let rechnungen = setup
        .app
        .execute(TagesabschlussDurchfuehren {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    assert_eq!(rechnungen.len(), 2);

    let rechnung_klient1 = rechnungen
        .iter()
        .find(|r| r.klient_id() == &setup.klient1.id)
        .expect("rechnung for klient1");
    let rechnung_klient2 = rechnungen
        .iter()
        .find(|r| r.klient_id() == &setup.klient2.id)
        .expect("rechnung for klient2");

    assert_eq!(rechnung_klient1.positionen().len(), 2);
    assert_eq!(rechnung_klient2.positionen().len(), 1);
    assert_eq!(
        rechnung_klient1.gesamtbetrag_netto().value(),
        Decimal::new(100, 0) // 2×25 + 50
    );
    assert_eq!(
        rechnung_klient1.gesamtbetrag_brutto().value(),
        Decimal::new(119, 0)
    );
    assert_eq!(rechnung_klient2.gesamtbetrag_netto().value(), Decimal::new(30, 0));
    assert_eq!(
        rechnung_klient2.gesamtbetrag_brutto().value(),
        Decimal::new(357, 1)
    );
}

#[pollster::test]
async fn test_tagesabschluss_zweiter_lauf_ohne_offene_leistungen() {
    let setup = setup_abrechnung_fixture().await;

    setup
        .app
        .execute(TagesabschlussDurchfuehren {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    let zweiter_abschluss = setup
        .app
        .execute(TagesabschlussDurchfuehren {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    assert!(zweiter_abschluss.is_empty());
}
