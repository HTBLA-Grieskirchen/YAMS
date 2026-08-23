use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RechnungStatus {
    Offen,
    Bezahlt,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Rechnungsposition {
    pub beschreibung: String,
    pub betrag: String,
    pub leistung_id: Uuid,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Rechnung {
    pub id: Uuid,
    pub rechnungsnummer: i64,
    pub klient_id: Uuid,
    pub rechnungsdatum: NaiveDate,
    pub positionen: Vec<Rechnungsposition>,
    pub gesamtbetrag: String,
    pub status: RechnungStatus,
}

pub fn schema_rechnung_from_domain(rechnung: domain::Rechnung) -> Rechnung {
    Rechnung {
        id: rechnung.id.0,
        rechnungsnummer: rechnung.rechnungsnummer,
        klient_id: rechnung.klient_id.0,
        rechnungsdatum: rechnung.rechnungsdatum,
        positionen: rechnung
            .positionen
            .into_iter()
            .map(|position| Rechnungsposition {
                beschreibung: position.beschreibung,
                betrag: position.betrag.value().to_string(),
                leistung_id: position.leistung_id.0,
            })
            .collect(),
        gesamtbetrag: rechnung.gesamtbetrag.value().to_string(),
        status: match rechnung.status {
            domain::RechnungStatus::Offen => RechnungStatus::Offen,
            domain::RechnungStatus::Bezahlt => RechnungStatus::Bezahlt,
        },
    }
}
