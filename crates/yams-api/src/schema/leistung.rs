use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::domain::{self, Leistung as DomainLeistung, LeistungOffen};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LeistungStatus {
    Offen,
    Abgerechnet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungQuelleProdukt {
    pub produkt_id: Uuid,
    pub menge: Decimal,
    pub einzelpreis: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungQuelleBehandlung {
    pub behandlung_id: Uuid,
    pub preis: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungQuelleManuell {
    pub preis: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungQuelleSeminar {
    pub termin_id: Uuid,
    pub buchung_id: Uuid,
    pub teilnahmegebühr_basis: Decimal,
    pub rabatt: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Union))]
#[cfg_attr(feature = "openapi", oai(discriminator_name = "typ", one_of = true))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "typ", rename_all = "camelCase"))]
pub enum LeistungQuelle {
    Produkt(LeistungQuelleProdukt),
    Behandlung(LeistungQuelleBehandlung),
    Manuell(LeistungQuelleManuell),
    Seminar(LeistungQuelleSeminar),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Leistung {
    pub id: Uuid,
    pub klient_id: Uuid,
    pub haustier_id: Option<Uuid>,
    pub beschreibung: String,
    pub betrag: Decimal,
    pub leistungsdatum: NaiveDate,
    pub status: LeistungStatus,
    pub quelle: LeistungQuelle,
    pub rechnung_id: Option<Uuid>,
}

pub fn schema_leistung_from_domain(leistung: LeistungOffen) -> Leistung {
    schema_leistung_from_domain_leistung(DomainLeistung::Offen(leistung))
}

pub fn schema_leistung_from_domain_leistung(leistung: DomainLeistung) -> Leistung {
    match leistung {
        DomainLeistung::Offen(leistung) => Leistung {
            id: leistung.id().0,
            klient_id: leistung.klient_id().0,
            haustier_id: leistung.haustier_id().as_ref().map(|id| id.0),
            beschreibung: leistung.beschreibung().to_string(),
            betrag: leistung.betrag().value(),
            leistungsdatum: leistung.leistungsdatum(),
            status: LeistungStatus::Offen,
            quelle: schema_quelle_from_domain(leistung.quelle()),
            rechnung_id: None,
        },
        DomainLeistung::Abgerechnet(leistung) => Leistung {
            id: leistung.id().0,
            klient_id: leistung.klient_id().0,
            haustier_id: leistung.haustier_id().as_ref().map(|id| id.0),
            beschreibung: leistung.beschreibung().to_string(),
            betrag: leistung.betrag().value(),
            leistungsdatum: leistung.leistungsdatum(),
            status: LeistungStatus::Abgerechnet,
            quelle: schema_quelle_from_domain(leistung.quelle()),
            rechnung_id: Some(leistung.rechnung_id().0),
        },
    }
}

fn schema_quelle_from_domain(quelle: &domain::LeistungQuelle) -> LeistungQuelle {
    match quelle {
        domain::LeistungQuelle::Produkt {
            produkt_id,
            menge,
            einzelpreis,
            ..
        } => LeistungQuelle::Produkt(LeistungQuelleProdukt {
            produkt_id: produkt_id.0,
            menge: menge.value(),
            einzelpreis: einzelpreis.value(),
        }),
        domain::LeistungQuelle::Behandlung {
            behandlung_id,
            preis,
            ..
        } => LeistungQuelle::Behandlung(LeistungQuelleBehandlung {
            behandlung_id: behandlung_id.0,
            preis: preis.value(),
        }),
        domain::LeistungQuelle::Manuell { preis, .. } => {
            LeistungQuelle::Manuell(LeistungQuelleManuell {
                preis: preis.value(),
            })
        }
        domain::LeistungQuelle::Seminar {
            termin_id,
            buchung_id,
            teilnahmegebühr_basis,
            rabatt,
            ..
        } => LeistungQuelle::Seminar(LeistungQuelleSeminar {
            termin_id: termin_id.0,
            buchung_id: buchung_id.0,
            teilnahmegebühr_basis: teilnahmegebühr_basis.value(),
            rabatt: rabatt.value(),
        }),
    }
}
