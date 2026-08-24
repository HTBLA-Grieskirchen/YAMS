use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::{
    domain::{BehandlungId, KlientId, Menge, Preis, ProduktId, Ratio},
    service::{
        BehandlungErstellen, LeistungAusBehandlungBuchen, LeistungAusProduktBuchen,
        LeistungManuellErfassen, ProduktErstellen, TagesabschlussDurchführen,
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
    pub einzelpreis: Decimal,
    pub mwst: Decimal,
}

impl TryFrom<ProduktErstellung> for ProduktErstellen {
    type Error = Report<ValidationError>;
    fn try_from(value: ProduktErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            beschreibung: value.beschreibung,
            einzelpreis: parse_preis(value.einzelpreis)?,
            mwst: parse_ratio(value.mwst)?,
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
    pub standardpreis: Decimal,
    pub mwst: Decimal,
}

impl TryFrom<BehandlungErstellung> for BehandlungErstellen {
    type Error = Report<ValidationError>;
    fn try_from(value: BehandlungErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            beschreibung: value.beschreibung,
            standardpreis: parse_preis(value.standardpreis)?,
            mwst: parse_ratio(value.mwst)?,
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
    pub menge: Decimal,
    pub leistungsdatum: NaiveDate,
}

impl TryFrom<LeistungAusProduktErstellung> for LeistungAusProduktBuchen {
    type Error = Report<ValidationError>;
    fn try_from(value: LeistungAusProduktErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            produkt_id: ProduktId(value.produkt_id),
            klient_id: KlientId(value.klient_id),
            haustier_id: value.haustier_id.map(yams_core::domain::HaustierId),
            menge: parse_menge(value.menge)?,
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
    pub preis_override: Option<Decimal>,
}

impl TryFrom<LeistungAusBehandlungErstellung> for LeistungAusBehandlungBuchen {
    type Error = Report<ValidationError>;
    fn try_from(value: LeistungAusBehandlungErstellung) -> Result<Self, Self::Error> {
        let preis_override = match value.preis_override {
            Some(preis) => Some(parse_preis(preis)?),
            None => None,
        };
        Ok(Self {
            behandlung_id: BehandlungId(value.behandlung_id),
            klient_id: KlientId(value.klient_id),
            haustier_id: value.haustier_id.map(yams_core::domain::HaustierId),
            leistungsdatum: value.leistungsdatum,
            preis_override,
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
    pub betrag: Decimal,
    pub mwst: Decimal,
    pub leistungsdatum: NaiveDate,
}

impl TryFrom<LeistungManuelleErstellung> for LeistungManuellErfassen {
    type Error = Report<ValidationError>;
    fn try_from(value: LeistungManuelleErstellung) -> Result<Self, Self::Error> {
        let betrag = parse_preis(value.betrag)?;
        Ok(Self {
            klient_id: KlientId(value.klient_id),
            haustier_id: value.haustier_id.map(yams_core::domain::HaustierId),
            beschreibung: value.beschreibung,
            betrag,
            mwst: parse_ratio(value.mwst)?,
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

impl From<TagesabschlussErstellung> for TagesabschlussDurchführen {
    fn from(value: TagesabschlussErstellung) -> Self {
        Self {
            abschlussdatum: value.abschlussdatum,
        }
    }
}

fn parse_preis(decimal: Decimal) -> Result<Preis, Report<ValidationError>> {
    Preis::new(decimal).change_context(ValidationError)
}

fn parse_ratio(decimal: Decimal) -> Result<Ratio, Report<ValidationError>> {
    Ratio::new(decimal).change_context(ValidationError)
}

fn parse_menge(decimal: Decimal) -> Result<Menge, Report<ValidationError>> {
    Menge::new(decimal).change_context(ValidationError)
}
