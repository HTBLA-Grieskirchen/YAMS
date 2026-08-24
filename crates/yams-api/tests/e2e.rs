use chrono::NaiveDate;
use rust_decimal::Decimal;
use yams_api::{
    YamsAppApi,
    requests::{
        BehandlungErstellung, HaustierErstellung, KlientErstellung,
        LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung,
        ProduktErstellung, TagesabschlussErstellung,
    },
    schema::{Adresse, Ländercode, RechnungStatus},
};
use yams_core::App;
use yams_fakes::FakeUnitOfWorkProvider;

fn api() -> YamsAppApi {
    let app = App::builder()
        .uow_provider(Box::new(FakeUnitOfWorkProvider::empty()))
        .build();
    YamsAppApi::new(app)
}

fn adresse(ländercode: &str) -> Adresse {
    Adresse {
        postleitzahl: "4711".into(),
        stadt: "Grieskirchen".into(),
        straße_und_hausnummer: "Hauptstraße 1".into(),
        ländercode: Ländercode(ländercode.into()),
    }
}

fn klient_erstellung(kundennummer: u64) -> KlientErstellung {
    KlientErstellung {
        vorname: "Anna".into(),
        nachname: "Muster".into(),
        geburtstag: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        email: "anna@muster.de".into(),
        mobilnummer: "1234567890".into(),
        kundennummer,
        einwilligung: true,
        adresse: adresse("DE"),
    }
}

#[pollster::test]
async fn klient_erstellen_returns_schema_fields() {
    let api = api();

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
    let api = api();
    let mut body = klient_erstellung(1001);
    body.email = "not-an-email".into();

    assert!(api.klient_erstellen(body).await.is_err());
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_mobilnummer() {
    let api = api();
    let mut body = klient_erstellung(1001);
    body.mobilnummer = "123".into();

    assert!(api.klient_erstellen(body).await.is_err());
}

#[pollster::test]
async fn klient_erstellen_rejects_invalid_ländercode() {
    let api = api();
    let mut body = klient_erstellung(1001);
    body.adresse.ländercode = Ländercode("US".into());

    assert!(api.klient_erstellen(body).await.is_err());
}

#[pollster::test]
async fn produkt_erstellen_rejects_negative_preis() {
    let api = api();

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
async fn haustier_erstellen_is_listed_and_fetchable() {
    let api = api();
    let klient = api.klient_erstellen(klient_erstellung(2001)).await.unwrap();

    let haustier = api
        .haustier_erstellen(HaustierErstellung {
            name: "Bello".into(),
            geburtstag: NaiveDate::from_ymd_opt(2020, 6, 15).unwrap(),
            tierart: "Hund".into(),
            beschreibung: "Mischling".into(),
            klient_id: klient.id,
        })
        .await
        .unwrap();

    assert_eq!(haustier.name, "Bello");
    assert_eq!(haustier.klient_id, klient.id);

    let by_id = api.haustier_by_id(haustier.id).await.unwrap();
    assert_eq!(by_id.id, haustier.id);

    let alle = api.alle_haustiere().await.unwrap();
    assert_eq!(alle.len(), 1);
    assert_eq!(alle[0].id, haustier.id);
}

#[pollster::test]
async fn tagesabschluss_returns_rechnungen_through_api() {
    let api = api();
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
