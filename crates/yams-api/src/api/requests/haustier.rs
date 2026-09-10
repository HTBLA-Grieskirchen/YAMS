use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::{domain::KlientId, service::HaustierErstellen};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct HaustierErstellung {
    pub name: String,
    pub geburtstag: NaiveDate,
    pub tierart: String,
    pub beschreibung: String,
    pub klient_id: Uuid,
}

impl TryFrom<HaustierErstellung> for HaustierErstellen {
    type Error = std::convert::Infallible;
    fn try_from(value: HaustierErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            klient_id: KlientId(value.klient_id),
            name: value.name,
            geburtstag: value.geburtstag,
            tierart: value.tierart,
            beschreibung: value.beschreibung,
        })
    }
}
