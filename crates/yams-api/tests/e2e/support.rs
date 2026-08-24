use chrono::NaiveDate;
use yams_api::{
    YamsAppApi,
    requests::KlientErstellung,
    schema::{Adresse, Ländercode},
};
use yams_core::App;
use yams_persistence::SQLiteInstance;

pub async fn api() -> YamsAppApi {
    let mut sqlite = SQLiteInstance::in_temp_dir().await.unwrap();
    sqlite.migrate_to_latest().await.unwrap();

    YamsAppApi::new(App::builder().uow_provider(Box::new(sqlite)).build())
}

pub fn adresse(ländercode: &str) -> Adresse {
    Adresse {
        postleitzahl: "4711".into(),
        stadt: "Grieskirchen".into(),
        straße_und_hausnummer: "Hauptstraße 1".into(),
        ländercode: Ländercode(ländercode.into()),
    }
}

pub fn klient_erstellung(kundennummer: u64) -> KlientErstellung {
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
