use std::sync::Arc;

use chrono::{TimeZone, Utc};
use rust_decimal::Decimal;
use yams_core::domain::{
    Adresse, Klient, Ländercode, Preis, Ratio, Seminar, SeminarOrt, SeminarTermin, Zeitraum,
};
use yams_core::service::{
    KlientErstellen, SeminarBuchungAnlegen, SeminarBuchungStornieren, SeminarErstellen,
    SeminarTerminAbsagen, SeminarTerminAktualisieren, SeminarTerminAlsAbgehaltenMarkieren,
    SeminarTerminPlanen, SeminarUmsatzPrognoseBisDatum, SeminarUmsatzVorschau,
    TagesabschlussDurchführen,
};

use super::super::base_app_builder;

fn mwst_20() -> Ratio {
    Ratio::new(Decimal::new(20, 2)).unwrap()
}

fn preis(euros: i64) -> Preis {
    Preis::new(Decimal::new(euros, 0)).unwrap()
}

fn zeitraum() -> Zeitraum {
    Zeitraum::neu(
        Utc.with_ymd_and_hms(2026, 8, 25, 10, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 8, 25, 16, 0, 0).unwrap(),
    )
    .unwrap()
}

fn adresse() -> Adresse {
    Adresse {
        postleitzahl: "4711".into(),
        stadt: "Grieskirchen".into(),
        straße_und_hausnummer: "Hauptstraße 1".into(),
        ländercode: Ländercode::from_str("AT").unwrap(),
    }
}

async fn klient(app: &yams_core::App, kundennummer: u64, email: &str) -> Klient {
    app.execute(KlientErstellen {
        vorname: "Anna".into(),
        nachname: "Muster".into(),
        geburtstag: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        email: email.try_into().unwrap(),
        mobilnummer: "1234567890".try_into().unwrap(),
        kundennummer,
        einwilligung: true,
        adresse: adresse(),
    })
    .await
    .unwrap()
}

async fn seminar(app: &yams_core::App) -> Seminar {
    app.execute(SeminarErstellen {
        titel: "Hufseminar".into(),
        beschreibung: "Einführung".into(),
        teilnahmegebühr_basis: preis(100),
        mwst: mwst_20(),
        standarddauer: None,
    })
    .await
    .unwrap()
}

async fn termin(app: &yams_core::App, seminar: &Seminar, max: Option<u32>) -> SeminarTermin {
    app.execute(SeminarTerminPlanen {
        seminar_id: seminar.id().clone(),
        zeitraum: zeitraum(),
        ort: SeminarOrt::neu(Some("Hof".into()), None),
        max_teilnehmer: max,
    })
    .await
    .unwrap()
}

#[pollster::test]
async fn seminar_buchung_rabatt_wird_abgehalten_und_abgerechnet() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let klient = klient(&app, 2001, "anna@test.de").await;
    let termin = termin(&app, &seminar, None).await;

    let gebucht = app
        .execute(SeminarBuchungAnlegen {
            termin_id: termin.id().clone(),
            klient_id: klient.id().clone(),
            rabatt: Ratio::new(Decimal::new(20, 2)).unwrap(),
        })
        .await
        .unwrap();
    assert!(matches!(gebucht, SeminarTermin::Geplant(_)));
    assert_eq!(gebucht.buchungen().len(), 1);

    let abgehalten = app
        .execute(SeminarTerminAlsAbgehaltenMarkieren {
            termin_id: termin.id().clone(),
        })
        .await
        .unwrap();
    let SeminarTermin::Abgehalten(abgehalten) = abgehalten else {
        panic!("expected abgehalten");
    };
    let buchung_id = abgehalten.buchungen()[0].id().clone();
    assert!(abgehalten.leistung_fuer_buchung(&buchung_id).is_some());

    let rechnungen = app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(zeitraum().ende().date_naive()),
        })
        .await
        .unwrap();
    assert_eq!(rechnungen.len(), 1);
    assert_eq!(
        rechnungen[0].positionen()[0].einzelpreis().value(),
        Decimal::new(80, 0)
    );
    assert_eq!(
        rechnungen[0].gesamtbetrag_brutto().value(),
        Decimal::new(96, 0)
    );
}

#[pollster::test]
async fn stornierte_buchung_wird_nicht_abgerechnet() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let klient = klient(&app, 2002, "bernd@test.de").await;
    let termin = termin(&app, &seminar, None).await;

    let gebucht = app
        .execute(SeminarBuchungAnlegen {
            termin_id: termin.id().clone(),
            klient_id: klient.id().clone(),
            rabatt: Ratio::zero(),
        })
        .await
        .unwrap();
    let buchung_id = gebucht.buchungen()[0].id().clone();

    app.execute(SeminarBuchungStornieren {
        termin_id: termin.id().clone(),
        buchung_id,
    })
    .await
    .unwrap();

    let abgehalten = app
        .execute(SeminarTerminAlsAbgehaltenMarkieren {
            termin_id: termin.id().clone(),
        })
        .await
        .unwrap();
    let SeminarTermin::Abgehalten(abgehalten) = abgehalten else {
        panic!("expected abgehalten");
    };
    assert!(abgehalten.leistungen().is_empty());
}

#[pollster::test]
async fn absage_blockiert_abgehalten() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarTerminAbsagen {
        termin_id: termin.id().clone(),
        grund: "zu wenig tn".into(),
    })
    .await
    .unwrap();

    let err = app
        .execute(SeminarTerminAlsAbgehaltenMarkieren {
            termin_id: termin.id().clone(),
        })
        .await
        .unwrap_err();
    assert!(format!("{err:?}").contains("nicht geplant"));
}

#[pollster::test]
async fn zweites_abgehalten_schlägt_fehl() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarTerminAlsAbgehaltenMarkieren {
        termin_id: termin.id().clone(),
    })
    .await
    .unwrap();

    let err = app
        .execute(SeminarTerminAlsAbgehaltenMarkieren {
            termin_id: termin.id().clone(),
        })
        .await
        .unwrap_err();
    assert!(format!("{err:?}").contains("nicht geplant"));
}

#[pollster::test]
async fn kapazität_und_doppelbuchung() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let a = klient(&app, 2003, "a@test.de").await;
    let b = klient(&app, 2004, "b@test.de").await;
    let termin = termin(&app, &seminar, Some(1)).await;

    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: a.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();

    let dup = app
        .execute(SeminarBuchungAnlegen {
            termin_id: termin.id().clone(),
            klient_id: a.id().clone(),
            rabatt: Ratio::zero(),
        })
        .await
        .unwrap_err();
    assert!(format!("{dup:?}").contains("invarianten"));

    let full = app
        .execute(SeminarBuchungAnlegen {
            termin_id: termin.id().clone(),
            klient_id: b.id().clone(),
            rabatt: Ratio::zero(),
        })
        .await
        .unwrap_err();
    assert!(format!("{full:?}").contains("invarianten"));
}

#[pollster::test]
async fn aktualisieren_nur_geplant() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let termin = termin(&app, &seminar, Some(4)).await;

    let neuer = Zeitraum::neu(
        Utc.with_ymd_and_hms(2026, 8, 26, 9, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 8, 26, 12, 0, 0).unwrap(),
    )
    .unwrap();

    let updated = app
        .execute(SeminarTerminAktualisieren {
            termin_id: termin.id().clone(),
            zeitraum: neuer.clone(),
            ort: SeminarOrt::neu(Some("Halle".into()), None),
            max_teilnehmer: Some(8),
        })
        .await
        .unwrap();
    assert_eq!(updated.zeitraum().beginn(), neuer.beginn());

    app.execute(SeminarTerminAbsagen {
        termin_id: termin.id().clone(),
        grund: "absage".into(),
    })
    .await
    .unwrap();

    let err = app
        .execute(SeminarTerminAktualisieren {
            termin_id: termin.id().clone(),
            zeitraum: neuer,
            ort: SeminarOrt::neu(None, None),
            max_teilnehmer: None,
        })
        .await
        .unwrap_err();
    assert!(format!("{err:?}").contains("nicht geplant"));
}

#[pollster::test]
async fn umsatz_vorschau_und_prognose() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let a = klient(&app, 2005, "c@test.de").await;
    let b = klient(&app, 2006, "d@test.de").await;
    let geplant = termin(&app, &seminar, None).await;

    app.execute(SeminarBuchungAnlegen {
        termin_id: geplant.id().clone(),
        klient_id: a.id().clone(),
        rabatt: Ratio::new(Decimal::new(20, 2)).unwrap(),
    })
    .await
    .unwrap();
    app.execute(SeminarBuchungAnlegen {
        termin_id: geplant.id().clone(),
        klient_id: b.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();

    let vorschau = app
        .execute(SeminarUmsatzVorschau {
            termin_id: geplant.id().clone(),
        })
        .await
        .unwrap();
    assert_eq!(vorschau.teilnehmer_anzahl, 2);
    assert_eq!(vorschau.gesamt_netto.value(), Decimal::new(180, 0));
    assert_eq!(vorschau.gesamt_brutto.value(), Decimal::new(216, 0));

    let später = app
        .execute(SeminarTerminPlanen {
            seminar_id: seminar.id().clone(),
            zeitraum: Zeitraum::neu(
                Utc.with_ymd_and_hms(2026, 9, 1, 10, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2026, 9, 1, 16, 0, 0).unwrap(),
            )
            .unwrap(),
            ort: SeminarOrt::neu(None, None),
            max_teilnehmer: None,
        })
        .await
        .unwrap();
    app.execute(SeminarBuchungAnlegen {
        termin_id: später.id().clone(),
        klient_id: a.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();

    let prognose = app
        .execute(SeminarUmsatzPrognoseBisDatum {
            stichtag: chrono::NaiveDate::from_ymd_opt(2026, 8, 25).unwrap(),
        })
        .await
        .unwrap();
    assert_eq!(prognose.termine.len(), 1);
    assert_eq!(prognose.gesamt_netto.value(), Decimal::new(180, 0));
}
