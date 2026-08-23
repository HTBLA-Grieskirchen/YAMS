use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Haustier {
    pub id: Uuid,
    pub klient_id: Uuid,
    pub name: String,
    pub tierart: String,
    pub geburtstag: NaiveDate,
    pub beschreibung: String,
}

pub fn schema_haustier_from_domain(haustier: domain::Haustier) -> Haustier {
    Haustier {
        id: haustier.id.0,
        klient_id: haustier.klient_id.0,
        name: haustier.name,
        tierart: haustier.tierart,
        geburtstag: haustier.geburtstag,
        beschreibung: haustier.beschreibung,
    }
}
