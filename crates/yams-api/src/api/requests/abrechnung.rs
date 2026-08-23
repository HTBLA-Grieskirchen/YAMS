use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use http::StatusCode;
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::{
    domain::{BehandlungId, KlientId, Preis, ProduktId},
    service::{
        BehandlungErstellen, LeistungAusBehandlungBuchen, LeistungAusProduktBuchen,
        LeistungManuellErfassen, ProduktErstellen, TagesabschlussDurchfuehren,
    },
};

use crate::errors::ValidationError;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ProduktErstellung {
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: String,
}

impl TryFrom<ProduktErstellung> for ProduktErstellen {
    type Error = Report<ValidationError>;
    fn try_from(value: ProduktErstellung) -> Result<Self, Self::Error> {
        let einzelpreis = parse_preis(&value.einzelpreis)?;
        Ok(Self {
            name: value.name,
            beschreibung: value.beschreibung,
            einzelpreis,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BehandlungErstellung {
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: String,
}

impl TryFrom<BehandlungErstellung> for BehandlungErstellen {
    type Error = Report<ValidationError>;
    fn try_from(value: BehandlungErstellung) -> Result<Self, Self::Error> {
        let standardpreis = parse_preis(&value.standardpreis)?;
        Ok(Self {
            name: value.name,
            beschreibung: value.beschreibung,
            standardpreis,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungAusProduktErstellung {
    pub produkt_id: Uuid,
    pub klient_id: Uuid,
    pub haustier_id: Option<Uuid>,
    pub menge: String,
    pub leistungsdatum: NaiveDate,
}

impl TryFrom<LeistungAusProduktErstellung> for LeistungAusProduktBuchen {
    type Error = Report<ValidationError>;
    fn try_from(value: LeistungAusProduktErstellung) -> Result<Self, Self::Error> {
        let menge = parse_decimal(&value.menge)?;
        Ok(Self {
            produkt_id: ProduktId(value.produkt_id),
            klient_id: KlientId(value.klient_id),
            haustier_id: value.haustier_id.map(yams_core::domain::HaustierId),
            menge,
            leistungsdatum: value.leistungsdatum,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungAusBehandlungErstellung {
    pub behandlung_id: Uuid,
    pub klient_id: Uuid,
    pub haustier_id: Option<Uuid>,
    pub leistungsdatum: NaiveDate,
}

impl TryFrom<LeistungAusBehandlungErstellung> for LeistungAusBehandlungBuchen {
    type Error = Report<ValidationError>;
    fn try_from(value: LeistungAusBehandlungErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            behandlung_id: BehandlungId(value.behandlung_id),
            klient_id: KlientId(value.klient_id),
            haustier_id: value.haustier_id.map(yams_core::domain::HaustierId),
            leistungsdatum: value.leistungsdatum,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LeistungManuelleErstellung {
    pub klient_id: Uuid,
    pub haustier_id: Option<Uuid>,
    pub beschreibung: String,
    pub betrag: String,
    pub leistungsdatum: NaiveDate,
}

impl TryFrom<LeistungManuelleErstellung> for LeistungManuellErfassen {
    type Error = Report<ValidationError>;
    fn try_from(value: LeistungManuelleErstellung) -> Result<Self, Self::Error> {
        let betrag = parse_preis(&value.betrag)?;
        Ok(Self {
            klient_id: KlientId(value.klient_id),
            haustier_id: value.haustier_id.map(yams_core::domain::HaustierId),
            beschreibung: value.beschreibung,
            betrag,
            leistungsdatum: value.leistungsdatum,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TagesabschlussErstellung {
    pub abschlussdatum: Option<NaiveDate>,
}

impl From<TagesabschlussErstellung> for TagesabschlussDurchfuehren {
    fn from(value: TagesabschlussErstellung) -> Self {
        Self {
            abschlussdatum: value.abschlussdatum,
        }
    }
}

fn parse_preis(value: &str) -> Result<Preis, Report<ValidationError>> {
  let decimal = parse_decimal(value)?;
  Preis::new(decimal)
    .change_context(ValidationError)
    .attach_opaque(StatusCode::UNPROCESSABLE_ENTITY)
}

fn parse_decimal(value: &str) -> Result<Decimal, Report<ValidationError>> {
  value
    .parse()
    .change_context(ValidationError)
    .attach_opaque(StatusCode::UNPROCESSABLE_ENTITY)
}
