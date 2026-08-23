mod common;

use std::sync::Arc;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use yams_core::domain::{Adresse, Klient, Ländercode, Preis};
use yams_core::service::{
    KlientErstellen, LeistungAusProduktBuchen, ProduktErstellen, TagesabschlussDurchführen,
};
use yams_core::App;

use common::fakes::{FakeUnitOfWorkProvider, FixedClock};

#[pollster::test]
async fn test_tagesabschluss_tage_nach_leistungen_mit_fake_clock() {
    let leistungsdatum = NaiveDate::from_ymd_opt(2026, 8, 20).unwrap();
    let heute = NaiveDate::from_ymd_opt(2026, 8, 23).unwrap();
    let clock = Arc::new(FixedClock::new(
        heute.and_hms_opt(12, 0, 0).unwrap().and_utc(),
    ));
    let app = Arc::new(
        App::builder()
            .uow_provider(Box::new(FakeUnitOfWorkProvider::empty()))
            .clock(clock)
            .build(),
    );

    let klient = app
        .execute(KlientErstellen {
            vorname: "Clara".into(),
            nachname: "Spät".into(),
            geburtstag: NaiveDate::from_ymd_opt(1992, 3, 3).unwrap(),
            email: "clara@spät.de".try_into().unwrap(),
            mobilnummer: "1112223333".try_into().unwrap(),
            kundennummer: 1003,
            einwilligung: true,
            adresse: Adresse {
                postleitzahl: "4711".into(),
                stadt: "Grieskirchen".into(),
                straße_und_hausnummer: "Hauptstraße 3".into(),
                ländercode: Ländercode::from_str("DE").unwrap(),
            },
        })
        .await
        .unwrap();

    let produkt = app
        .execute(ProduktErstellen {
            name: "Snack".into(),
            beschreibung: "Leckerli".into(),
            einzelpreis: Preis::new(Decimal::new(10, 0)).unwrap(),
            mwst_prozentsatz: Decimal::new(19, 0),
        })
        .await
        .unwrap();

    let produkt_id = produkt.id.clone();

    app.execute(LeistungAusProduktBuchen {
        produkt_id,
        klient_id: klient.id.clone(),
        haustier_id: None,
        menge: Decimal::ONE,
        leistungsdatum,
    })
    .await
    .unwrap();

    let rechnungen_verzögert = app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(leistungsdatum),
        })
        .await
        .unwrap();

    assert_eq!(rechnungen_verzögert.len(), 1);
    assert_eq!(rechnungen_verzögert[0].klient_id(), &klient.id);
    assert_eq!(rechnungen_verzögert[0].rechnungsdatum(), leistungsdatum);

    app.execute(LeistungAusProduktBuchen {
        produkt_id: produkt.id,
        klient_id: klient.id.clone(),
        haustier_id: None,
        menge: Decimal::ONE,
        leistungsdatum: heute,
    })
    .await
    .unwrap();

    let rechnungen_heute = app
        .execute(TagesabschlussDurchführen { abschlussdatum: None })
        .await
        .unwrap();

    assert_eq!(rechnungen_heute.len(), 1);
    assert_eq!(rechnungen_heute[0].klient_id(), &klient.id);
    assert_eq!(rechnungen_heute[0].rechnungsdatum(), heute);
}
