use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain;

use crate::schema::{Adresse, EmailAdresse, Haustier, Mobilnummer, schema_haustier_from_domain};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Klient {
    pub id: Uuid,
    pub vorname: String,
    pub nachname: String,
    pub geburtstag: NaiveDate,
    pub email: EmailAdresse,
    pub mobilnummer: Mobilnummer,
    pub kundennummer: u64,
    pub einwilligung: bool,
    pub adresse: Adresse,
    pub haustiere: Vec<Haustier>,
}

pub fn schema_klient_from_domain(
    klient: domain::Klient,
    haustiere: Vec<domain::Haustier>,
) -> Klient {
    Klient {
        id: klient.id().0,
        vorname: klient.vorname().to_string(),
        nachname: klient.nachname().to_string(),
        geburtstag: klient.geburtstag(),
        email: klient.email().clone().into(),
        mobilnummer: klient.mobilnummer().clone().into(),
        kundennummer: klient.kundennummer(),
        einwilligung: klient.einwilligung(),
        adresse: klient.adresse().clone().into(),
        haustiere: haustiere
            .into_iter()
            .map(schema_haustier_from_domain)
            .collect(),
    }
}
