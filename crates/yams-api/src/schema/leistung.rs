use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungQuelleBehandlung {
    pub behandlung_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeistungQuelleManuell {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Union))]
#[cfg_attr(feature = "openapi", oai(discriminator_name = "typ", one_of = true))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "typ", rename_all = "camelCase"))]
pub enum LeistungQuelle {
    Produkt(LeistungQuelleProdukt),
    Behandlung(LeistungQuelleBehandlung),
    Manuell(LeistungQuelleManuell),
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
    pub betrag: String,
    pub leistungsdatum: NaiveDate,
    pub status: LeistungStatus,
    pub quelle: LeistungQuelle,
    pub rechnung_id: Option<Uuid>,
}

pub fn schema_leistung_from_domain(leistung: domain::Leistung) -> Leistung {
    Leistung {
        id: leistung.id.0,
        klient_id: leistung.klient_id.0,
        haustier_id: leistung.haustier_id.map(|id| id.0),
        beschreibung: leistung.beschreibung,
        betrag: leistung.betrag.value().to_string(),
        leistungsdatum: leistung.leistungsdatum,
        status: match leistung.status {
            domain::LeistungStatus::Offen => LeistungStatus::Offen,
            domain::LeistungStatus::Abgerechnet => LeistungStatus::Abgerechnet,
        },
        quelle: match leistung.quelle {
            domain::LeistungQuelle::Produkt(id) => LeistungQuelle::Produkt(LeistungQuelleProdukt {
                produkt_id: id.0,
            }),
            domain::LeistungQuelle::Behandlung(id) => {
                LeistungQuelle::Behandlung(LeistungQuelleBehandlung {
                    behandlung_id: id.0,
                })
            }
            domain::LeistungQuelle::Manuell => LeistungQuelle::Manuell(LeistungQuelleManuell {}),
        },
        rechnung_id: leistung.rechnung_id.map(|id| id.0),
    }
}
