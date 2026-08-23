use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::domain::{self, Rechnung as DomainRechnung, RechnungIn, RechnungOffen};

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
    pub einzelpreis: Decimal,
    pub stückzahl: Decimal,
    pub mwst_prozentsatz: Decimal,
    pub gesamtpreis_netto: Decimal,
    pub gesamtpreis_brutto: Decimal,
    pub leistung_id: Uuid,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Rechnung {
    pub id: Uuid,
    pub rechnungsnummer: u64,
    pub klient_id: Uuid,
    pub rechnungsdatum: NaiveDate,
    pub positionen: Vec<Rechnungsposition>,
    pub gesamtbetrag_brutto: Decimal,
    pub status: RechnungStatus,
    pub bezahlt_datum: Option<NaiveDate>,
}

pub fn schema_rechnung_from_domain(rechnung: RechnungOffen) -> Rechnung {
    schema_rechnung_from_domain_rechnung(DomainRechnung::Offen(rechnung))
}

pub fn schema_rechnung_from_domain_rechnung(rechnung: DomainRechnung) -> Rechnung {
    match rechnung {
        DomainRechnung::Offen(rechnung) => schema_rechnung_common(&rechnung, RechnungStatus::Offen, None),
        DomainRechnung::Bezahlt(rechnung) => schema_rechnung_common(
            &rechnung,
            RechnungStatus::Bezahlt,
            Some(rechnung.bezahlt_datum()),
        ),
    }
}

fn schema_rechnung_common<S>(
    rechnung: &RechnungIn<S>,
    status: RechnungStatus,
    bezahlt_datum: Option<NaiveDate>,
) -> Rechnung {
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
        gesamtbetrag_brutto: rechnung.gesamtbetrag_brutto().value(),
        status,
        bezahlt_datum,
    }
}

fn schema_position_from_domain(position: &domain::Rechnungsposition) -> Rechnungsposition {
    Rechnungsposition {
        beschreibung: position.beschreibung().to_string(),
        einzelpreis: position.einzelpreis().value(),
        stückzahl: position.stückzahl(),
        mwst_prozentsatz: position.mwst_prozentsatz(),
        gesamtpreis_netto: position.gesamtpreis_netto().value(),
        gesamtpreis_brutto: position.gesamtpreis_brutto().value(),
        leistung_id: position.leistung_id().0,
    }
}
