use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::domain;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Behandlung {
    pub id: Uuid,
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: Decimal,
    pub mwst_prozentsatz: Decimal,
}

pub fn schema_behandlung_from_domain(behandlung: domain::Behandlung) -> Behandlung {
    Behandlung {
        id: behandlung.id.0,
        name: behandlung.name,
        beschreibung: behandlung.beschreibung,
        standardpreis: behandlung.standardpreis.value(),
        mwst_prozentsatz: behandlung.mwst_prozentsatz,
    }
}
