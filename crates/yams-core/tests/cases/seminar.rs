use std::sync::Arc;

use chrono::{TimeZone, Utc};
use rust_decimal::Decimal;
use yams_core::domain::{
    Adresse, Klient, Leistung, LeistungId, Ländercode, Preis, Ratio, Seminar, SeminarOrt,
    SeminarTermin, Zeitraum,
};
use yams_core::ports::RepositoryError;
use yams_core::service::{
    KlientErstellen, SeminarBuchungAnlegen, SeminarBuchungStornieren, SeminarErstellen,
    SeminarTerminAbsagen, SeminarTerminAktualisieren, SeminarTerminAlsAbgehaltenMarkieren,
    SeminarTerminPlanen, SeminarUmsatzPrognoseBisDatum, SeminarUmsatzVorschau,
    TagesabschlussDurchführen,
};

use super::super::base_app_builder;
use yams_core::service::{teilnahme_dokument, teilnahme_object_key};
use yams_fakes::{FAKE_PDF, FakeObjectStore, FakePdfRenderer};

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

async fn app_with_pdf_fakes() -> (Arc<yams_core::App>, FakeObjectStore, FakePdfRenderer) {
    let store = FakeObjectStore::new();
    let renderer = FakePdfRenderer::new();
    let app = Arc::new(
        base_app_builder()
            .await
            .object_store(Arc::new(store.clone()))
            .pdf_renderer(Arc::new(renderer.clone()))
            .build(),
    );
    (app, store, renderer)
}

async fn termin(app: &yams_core::App, seminar: &Seminar, max: Option<u32>) -> SeminarTermin {
    SeminarTermin::from(
        app.execute(SeminarTerminPlanen {
            seminar_id: seminar.id().clone(),
            zeitraum: zeitraum(),
            ort: SeminarOrt::neu(Some("Hof".into()), None),
            max_teilnehmer: max,
        })
        .await
        .unwrap(),
    )
}

async fn load_leistung(app: &yams_core::App, id: LeistungId) -> Leistung {
    app.execute_fn::<_, _, RepositoryError>(async move |ctx| {
        let uow = ctx.enter().await?;
        let result = uow.leistungen().find_by_id(id).await;
        uow.finish(result, RepositoryError::OperationFailed)
            .await
            .map(|versioned| versioned.into_data())
    })
    .await
    .unwrap()
}

fn assert_offen_mwst(leistung: &Leistung, expected: Decimal) {
    match leistung {
        Leistung::Offen(offen) => assert_eq!(offen.quelle().mwst().value(), expected),
        other => panic!("expected offen, got {other:?}"),
    }
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
    let leistung_id = abgehalten
        .leistung_fuer_buchung(&buchung_id)
        .expect("buchung must map to leistung")
        .clone();
    assert_offen_mwst(&load_leistung(&app, leistung_id).await, Decimal::new(20, 2));

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

#[pollster::test]
async fn abgehalten_maps_every_confirmed_buchung() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let a = klient(&app, 2010, "map-a@test.de").await;
    let b = klient(&app, 2011, "map-b@test.de").await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: a.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();
    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: b.id().clone(),
        rabatt: Ratio::new(Decimal::new(20, 2)).unwrap(),
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

    let bestätigt: Vec<_> = abgehalten
        .bestätigte_buchungen()
        .map(|buchung| buchung.id().clone())
        .collect();
    assert_eq!(bestätigt.len(), 2);
    assert_eq!(abgehalten.leistungen().len(), 2);
    for buchung_id in &bestätigt {
        let leistung_id = abgehalten
            .leistung_fuer_buchung(buchung_id)
            .expect("buchung must map to leistung")
            .clone();
        assert_offen_mwst(&load_leistung(&app, leistung_id).await, Decimal::new(20, 2));
    }
}

#[pollster::test]
async fn zwei_klienten_erzeugen_zwei_rechnungen() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let a = klient(&app, 2012, "inv-a@test.de").await;
    let b = klient(&app, 2013, "inv-b@test.de").await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: a.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();
    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: b.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();

    app.execute(SeminarTerminAlsAbgehaltenMarkieren {
        termin_id: termin.id().clone(),
    })
    .await
    .unwrap();

    let rechnungen = app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(zeitraum().ende().date_naive()),
        })
        .await
        .unwrap();
    assert_eq!(rechnungen.len(), 2);
}

#[pollster::test]
async fn voller_rabatt_ergibt_null_betrag() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let klient = klient(&app, 2014, "gratis@test.de").await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: klient.id().clone(),
        rabatt: Ratio::one(),
    })
    .await
    .unwrap();

    app.execute(SeminarTerminAlsAbgehaltenMarkieren {
        termin_id: termin.id().clone(),
    })
    .await
    .unwrap();

    let rechnungen = app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(zeitraum().ende().date_naive()),
        })
        .await
        .unwrap();
    assert_eq!(rechnungen.len(), 1);
    assert_eq!(
        rechnungen[0].positionen()[0].einzelpreis().value(),
        Decimal::ZERO
    );
    assert_eq!(rechnungen[0].gesamtbetrag_brutto().value(), Decimal::ZERO);
}

#[pollster::test]
async fn storno_erlaubt_erneute_buchung_desselben_klienten() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let klient = klient(&app, 2015, "rebook@test.de").await;
    let termin = termin(&app, &seminar, Some(1)).await;

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

    let erneut = app
        .execute(SeminarBuchungAnlegen {
            termin_id: termin.id().clone(),
            klient_id: klient.id().clone(),
            rabatt: Ratio::zero(),
        })
        .await
        .unwrap();
    assert_eq!(
        erneut
            .buchungen()
            .iter()
            .filter(|buchung| buchung.ist_bestätigt())
            .count(),
        1
    );

    let abgehalten = app
        .execute(SeminarTerminAlsAbgehaltenMarkieren {
            termin_id: termin.id().clone(),
        })
        .await
        .unwrap();
    let SeminarTermin::Abgehalten(abgehalten) = abgehalten else {
        panic!("expected abgehalten");
    };
    assert_eq!(abgehalten.leistungen().len(), 1);
}

#[pollster::test]
async fn abgehalten_schreibt_teilnahme_pdf_für_bestätigte_buchungen() {
    let (app, store, renderer) = app_with_pdf_fakes().await;
    let seminar = seminar(&app).await;
    let a = klient(&app, 2100, "pdf-a@test.de").await;
    let b = klient(&app, 2101, "pdf-b@test.de").await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: a.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();
    let gebucht_b = app
        .execute(SeminarBuchungAnlegen {
            termin_id: termin.id().clone(),
            klient_id: b.id().clone(),
            rabatt: Ratio::zero(),
        })
        .await
        .unwrap();
    let b_buchung_id = gebucht_b
        .buchungen()
        .iter()
        .find(|buchung| buchung.klient_id() == b.id())
        .unwrap()
        .id()
        .clone();
    app.execute(SeminarBuchungStornieren {
        termin_id: termin.id().clone(),
        buchung_id: b_buchung_id.clone(),
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

    let bestätigt: Vec<_> = abgehalten.bestätigte_buchungen().cloned().collect();
    assert_eq!(bestätigt.len(), 1);
    assert_eq!(bestätigt[0].klient_id(), a.id());

    let expected = teilnahme_dokument(&abgehalten, &seminar, &bestätigt[0], &a);
    assert_eq!(renderer.calls(), vec![expected]);
    assert_eq!(
        store
            .stored(&teilnahme_object_key(abgehalten.id(), bestätigt[0].id()))
            .as_deref(),
        Some(FAKE_PDF)
    );
    assert!(
        store
            .stored(&teilnahme_object_key(abgehalten.id(), &b_buchung_id))
            .is_none()
    );
}

#[pollster::test]
async fn aktualisieren_blockiert_nach_abgehalten() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarTerminAlsAbgehaltenMarkieren {
        termin_id: termin.id().clone(),
    })
    .await
    .unwrap();

    let err = app
        .execute(SeminarTerminAktualisieren {
            termin_id: termin.id().clone(),
            zeitraum: zeitraum(),
            ort: SeminarOrt::neu(None, None),
            max_teilnehmer: None,
        })
        .await
        .unwrap_err();
    assert!(format!("{err:?}").contains("nicht geplant"));
}

#[pollster::test]
async fn vorschau_abgehalten_zählt_nur_offene_leistungen() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let klient = klient(&app, 2016, "open@test.de").await;
    let termin = termin(&app, &seminar, None).await;

    app.execute(SeminarBuchungAnlegen {
        termin_id: termin.id().clone(),
        klient_id: klient.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();

    app.execute(SeminarTerminAlsAbgehaltenMarkieren {
        termin_id: termin.id().clone(),
    })
    .await
    .unwrap();

    let offen = app
        .execute(SeminarUmsatzVorschau {
            termin_id: termin.id().clone(),
        })
        .await
        .unwrap();
    assert_eq!(offen.teilnehmer_anzahl, 1);
    assert_eq!(offen.gesamt_netto.value(), Decimal::new(100, 0));

    app.execute(TagesabschlussDurchführen {
        abschlussdatum: Some(zeitraum().ende().date_naive()),
    })
    .await
    .unwrap();

    let danach = app
        .execute(SeminarUmsatzVorschau {
            termin_id: termin.id().clone(),
        })
        .await
        .unwrap();
    assert_eq!(danach.teilnehmer_anzahl, 0);
    assert_eq!(danach.gesamt_netto.value(), Decimal::ZERO);
}

#[pollster::test]
async fn prognose_enthält_abgehalten_offen_und_schließt_abgesagt_aus() {
    let app = Arc::new(base_app_builder().await.build());
    let seminar = seminar(&app).await;
    let a = klient(&app, 2017, "prog-a@test.de").await;
    let b = klient(&app, 2018, "prog-b@test.de").await;

    let gehalten = termin(&app, &seminar, None).await;
    app.execute(SeminarBuchungAnlegen {
        termin_id: gehalten.id().clone(),
        klient_id: a.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();
    app.execute(SeminarTerminAlsAbgehaltenMarkieren {
        termin_id: gehalten.id().clone(),
    })
    .await
    .unwrap();

    let abgesagt = termin(&app, &seminar, None).await;
    app.execute(SeminarBuchungAnlegen {
        termin_id: abgesagt.id().clone(),
        klient_id: b.id().clone(),
        rabatt: Ratio::zero(),
    })
    .await
    .unwrap();
    app.execute(SeminarTerminAbsagen {
        termin_id: abgesagt.id().clone(),
        grund: "zu wenig tn".into(),
    })
    .await
    .unwrap();

    let stichtag = zeitraum().ende().date_naive();
    let vor_abschluss = app
        .execute(SeminarUmsatzPrognoseBisDatum { stichtag })
        .await
        .unwrap();
    assert_eq!(vor_abschluss.termine.len(), 1);
    assert_eq!(vor_abschluss.termine[0].termin_id, gehalten.id().clone());
    assert_eq!(vor_abschluss.gesamt_netto.value(), Decimal::new(100, 0));

    app.execute(TagesabschlussDurchführen {
        abschlussdatum: Some(stichtag),
    })
    .await
    .unwrap();

    let nach_abschluss = app
        .execute(SeminarUmsatzPrognoseBisDatum { stichtag })
        .await
        .unwrap();
    assert!(nach_abschluss.termine.is_empty());
    assert_eq!(nach_abschluss.gesamt_netto.value(), Decimal::ZERO);
}
