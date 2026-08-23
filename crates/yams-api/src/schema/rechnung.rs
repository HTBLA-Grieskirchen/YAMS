use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain::{self, rechnung::Rechnung as DomainRechnung, GeladeneRechnung, RechnungOffen};

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
    pub einzelpreis: String,
    #[cfg_attr(feature = "openapi", oai(rename = "stückzahl"))]
    #[cfg_attr(feature = "serde", serde(rename = "stückzahl"))]
    pub stueckzahl: String,
    pub mwst_prozentsatz: String,
    pub gesamtpreis_netto: String,
    pub gesamtpreis_brutto: String,
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
    pub gesamtbetrag_brutto: String,
    pub status: RechnungStatus,
}

pub fn schema_rechnung_from_domain(rechnung: RechnungOffen) -> Rechnung {
    schema_rechnung_from_geladene(GeladeneRechnung::Offen(rechnung))
}

pub fn schema_rechnung_from_geladene(rechnung: GeladeneRechnung) -> Rechnung {
    match rechnung {
        GeladeneRechnung::Offen(rechnung) => schema_rechnung_common(&rechnung, RechnungStatus::Offen),
        GeladeneRechnung::Bezahlt(rechnung) => {
            schema_rechnung_common(&rechnung, RechnungStatus::Bezahlt)
        }
    }
}

fn schema_rechnung_common<S>(rechnung: &DomainRechnung<S>, status: RechnungStatus) -> Rechnung {
    Rechnung {
        id: rechnung.id().0,
        rechnungsnummer: rechnung.rechnungsnummer(),
        klient_id: rechnung.klient_id().0,
        rechnungsdatum: rechnung.rechnungsdatum(),
        positionen: rechnung
            .positionen()
            .iter()
            .map(schema_position_from_domain)
            .collect(),
        gesamtbetrag_brutto: rechnung.gesamtbetrag_brutto().value().to_string(),
        status,
    }
}

fn schema_position_from_domain(position: &domain::Rechnungsposition) -> Rechnungsposition {
    Rechnungsposition {
        beschreibung: position.beschreibung().to_string(),
        einzelpreis: position.einzelpreis().value().to_string(),
        stueckzahl: position.stückzahl().to_string(),
        mwst_prozentsatz: position.mwst_prozentsatz().to_string(),
        gesamtpreis_netto: position.gesamtpreis_netto().value().to_string(),
        gesamtpreis_brutto: position.gesamtpreis_brutto().value().to_string(),
        leistung_id: position.leistung_id().0,
    }
}
