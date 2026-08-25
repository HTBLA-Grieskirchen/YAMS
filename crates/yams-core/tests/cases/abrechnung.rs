use std::sync::Arc;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use yams_core::domain::{Adresse, Klient, Ländercode, Menge, Preis, Ratio};
use yams_core::service::{
    BehandlungErstellen, KlientErstellen, LeistungAusBehandlungBuchen, LeistungAusProduktBuchen,
    LeistungManuellErfassen, ProduktErstellen, TagesabschlussDurchführen,
};

use super::super::base_app_builder;
use yams_core::ports::rechnung_object_key;
use yams_core::service::rechnungsdokument;
use yams_fakes::{FAKE_PDF, FakeObjectStore, FakePdfRenderer, FixedClock};

fn mwst_19() -> Ratio {
    Ratio::new(Decimal::new(19, 2)).unwrap()
}

fn menge(n: i64) -> Menge {
    Menge::new(Decimal::new(n, 0)).unwrap()
}

struct AbrechnungSetup {
    app: Arc<yams_core::App>,
    klient1: Klient,
    klient2: Klient,
    abschlussdatum: NaiveDate,
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

async fn setup_abrechnung_fixture() -> AbrechnungSetup {
    setup_abrechnung_on(Arc::new(base_app_builder().await.build())).await
}

async fn setup_abrechnung_on(app: Arc<yams_core::App>) -> AbrechnungSetup {
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
                straße_und_hausnummer: "Hauptstraße 1".into(),
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
                straße_und_hausnummer: "Nebenstraße 2".into(),
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
            mwst: mwst_19(),
        })
        .await
        .unwrap();

    let behandlung = app
        .execute(BehandlungErstellen {
            name: "Untersuchung".into(),
            beschreibung: "Allgemeine Untersuchung".into(),
            standardpreis: Preis::new(Decimal::new(50, 0)).unwrap(),
            mwst: mwst_19(),
        })
        .await
        .unwrap();

    let abschlussdatum = NaiveDate::from_ymd_opt(2026, 8, 23).unwrap();

    let produkt_id = produkt.id().clone();

    app.execute(LeistungAusProduktBuchen {
        produkt_id,
        klient_id: klient1.id().clone(),
        haustier_id: None,
        menge: menge(2),
        leistungsdatum: abschlussdatum,
    })
    .await
    .unwrap();

    app.execute(LeistungAusBehandlungBuchen {
        behandlung_id: behandlung.id().clone(),
        klient_id: klient1.id().clone(),
        haustier_id: None,
        leistungsdatum: abschlussdatum,
        preis_override: None,
    })
    .await
    .unwrap();

    app.execute(LeistungManuellErfassen {
        klient_id: klient2.id().clone(),
        haustier_id: None,
        beschreibung: "Beratung".into(),
        betrag: Preis::new(Decimal::new(30, 0)).unwrap(),
        mwst: mwst_19(),
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
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    assert_eq!(rechnungen.len(), 2);

    let rechnung_klient1 = rechnungen
        .iter()
        .find(|r| r.klient_id() == setup.klient1.id())
        .expect("rechnung for klient1");
    let rechnung_klient2 = rechnungen
        .iter()
        .find(|r| r.klient_id() == setup.klient2.id())
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
    assert_eq!(
        rechnung_klient2.gesamtbetrag_netto().value(),
        Decimal::new(30, 0)
    );
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
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    let zweiter_abschluss = setup
        .app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    assert!(zweiter_abschluss.is_empty());
}

#[pollster::test]
async fn test_tagesabschluss_tage_nach_leistungen_mit_fake_clock() {
    let leistungsdatum = NaiveDate::from_ymd_opt(2026, 8, 20).unwrap();
    let heute = NaiveDate::from_ymd_opt(2026, 8, 23).unwrap();
    let clock = Arc::new(FixedClock::new(
        heute.and_hms_opt(12, 0, 0).unwrap().and_utc(),
    ));
    let app = Arc::new(base_app_builder().await.clock(clock).build());

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
            mwst: mwst_19(),
        })
        .await
        .unwrap();

    let produkt_id = produkt.id().clone();

    app.execute(LeistungAusProduktBuchen {
        produkt_id,
        klient_id: klient.id().clone(),
        haustier_id: None,
        menge: Menge::one(),
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
    assert_eq!(rechnungen_verzögert[0].klient_id(), klient.id());
    assert_eq!(rechnungen_verzögert[0].rechnungsdatum(), leistungsdatum);

    app.execute(LeistungAusProduktBuchen {
        produkt_id: produkt.id().clone(),
        klient_id: klient.id().clone(),
        haustier_id: None,
        menge: Menge::one(),
        leistungsdatum: heute,
    })
    .await
    .unwrap();

    let rechnungen_heute = app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: None,
        })
        .await
        .unwrap();

    assert_eq!(rechnungen_heute.len(), 1);
    assert_eq!(rechnungen_heute[0].klient_id(), klient.id());
    assert_eq!(rechnungen_heute[0].rechnungsdatum(), heute);
}

#[pollster::test]
async fn test_tagesabschluss_ignores_leistungen_on_other_dates() {
    let setup = setup_abrechnung_fixture().await;
    let other_day = NaiveDate::from_ymd_opt(2026, 8, 22).unwrap();

    let rechnungen = setup
        .app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(other_day),
        })
        .await
        .unwrap();

    assert!(rechnungen.is_empty());
}

#[pollster::test]
async fn test_tagesabschluss_empty_day_returns_no_rechnungen() {
    let app = Arc::new(base_app_builder().await.build());
    let rechnungen = app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(NaiveDate::from_ymd_opt(2026, 8, 23).unwrap()),
        })
        .await
        .unwrap();
    assert!(rechnungen.is_empty());
}

#[pollster::test]
async fn test_tagesabschluss_incremental_reclose_bills_only_new_leistung() {
    let setup = setup_abrechnung_fixture().await;

    let first = setup
        .app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();
    let max_nummer = first.iter().map(|r| r.rechnungsnummer()).max().unwrap();

    let produkt = setup
        .app
        .execute(ProduktErstellen {
            name: "Nachbuchung".into(),
            beschreibung: "Extra".into(),
            einzelpreis: Preis::new(Decimal::new(10, 0)).unwrap(),
            mwst: mwst_19(),
        })
        .await
        .unwrap();

    setup
        .app
        .execute(LeistungAusProduktBuchen {
            produkt_id: produkt.id().clone(),
            klient_id: setup.klient1.id().clone(),
            haustier_id: None,
            menge: Menge::one(),
            leistungsdatum: setup.abschlussdatum,
        })
        .await
        .unwrap();

    let second = setup
        .app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    assert_eq!(second.len(), 1);
    assert_eq!(second[0].klient_id(), setup.klient1.id());
    assert_eq!(second[0].positionen().len(), 1);
    assert!(second[0].rechnungsnummer() > max_nummer);
}

#[pollster::test]
async fn tagesabschluss_schreibt_pdfs_und_ruft_renderer_mit_rechnungsdaten() {
    let (app, store, renderer) = app_with_pdf_fakes().await;
    let setup = setup_abrechnung_on(app).await;

    let rechnungen = setup
        .app
        .execute(TagesabschlussDurchführen {
            abschlussdatum: Some(setup.abschlussdatum),
        })
        .await
        .unwrap();

    assert_eq!(rechnungen.len(), 2);
    let calls = renderer.calls();
    assert_eq!(calls.len(), 2);

    for rechnung in &rechnungen {
        let klient = if rechnung.klient_id() == setup.klient1.id() {
            &setup.klient1
        } else {
            &setup.klient2
        };
        let expected = rechnungsdokument(rechnung, klient);
        assert!(
            calls.contains(&expected),
            "missing renderer call for rechnung {}",
            rechnung.rechnungsnummer()
        );
        assert_eq!(
            store.stored(&rechnung_object_key(rechnung.id())).as_deref(),
            Some(FAKE_PDF)
        );
    }
}
