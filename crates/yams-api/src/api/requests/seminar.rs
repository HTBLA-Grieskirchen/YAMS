use chrono::{DateTime, TimeDelta, Utc};
use error_stack::{Report, ResultExt};
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::{
    domain::{
        KlientId, Preis, Ratio, SeminarBuchungId, SeminarId, SeminarOrt, SeminarTerminId, Zeitraum,
    },
    service::{
        SeminarBuchungAnlegen, SeminarErstellen, SeminarTerminAbsagen, SeminarTerminAktualisieren,
        SeminarTerminAlsAbgehaltenMarkieren, SeminarTerminPlanen,
    },
};

use crate::{errors::ValidationError, schema::Adresse};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarErstellung {
    pub titel: String,
    pub beschreibung: String,
    pub teilnahmegebühr_basis: Decimal,
    pub mwst: Decimal,
    pub standarddauer_ms: Option<i64>,
}

impl TryFrom<SeminarErstellung> for SeminarErstellen {
    type Error = Report<ValidationError>;
    fn try_from(value: SeminarErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            titel: value.titel,
            beschreibung: value.beschreibung,
            teilnahmegebühr_basis: parse_preis(value.teilnahmegebühr_basis)?,
            mwst: parse_ratio(value.mwst)?,
            standarddauer: value.standarddauer_ms.map(TimeDelta::milliseconds),
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarOrtErstellung {
    pub ort_name: Option<String>,
    pub adresse: Option<Adresse>,
}

impl TryFrom<SeminarOrtErstellung> for SeminarOrt {
    type Error = Report<ValidationError>;
    fn try_from(value: SeminarOrtErstellung) -> Result<Self, Self::Error> {
        let adresse = match value.adresse {
            Some(adresse) => Some(adresse.try_into().change_context(ValidationError)?),
            None => None,
        };
        Ok(SeminarOrt::neu(value.ort_name, adresse))
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarTerminErstellung {
    pub seminar_id: Uuid,
    pub beginn: DateTime<Utc>,
    pub ende: DateTime<Utc>,
    pub ort: SeminarOrtErstellung,
    pub max_teilnehmer: Option<u32>,
}

impl TryFrom<SeminarTerminErstellung> for SeminarTerminPlanen {
    type Error = Report<ValidationError>;
    fn try_from(value: SeminarTerminErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            seminar_id: SeminarId(value.seminar_id),
            zeitraum: Zeitraum::neu(value.beginn, value.ende).change_context(ValidationError)?,
            ort: value.ort.try_into()?,
            max_teilnehmer: value.max_teilnehmer,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarTerminAktualisierung {
    pub beginn: DateTime<Utc>,
    pub ende: DateTime<Utc>,
    pub ort: SeminarOrtErstellung,
    pub max_teilnehmer: Option<u32>,
}

impl SeminarTerminAktualisierung {
    pub fn into_use_case(
        self,
        termin_id: Uuid,
    ) -> Result<SeminarTerminAktualisieren, Report<ValidationError>> {
        Ok(SeminarTerminAktualisieren {
            termin_id: SeminarTerminId(termin_id),
            zeitraum: Zeitraum::neu(self.beginn, self.ende).change_context(ValidationError)?,
            ort: self.ort.try_into()?,
            max_teilnehmer: self.max_teilnehmer,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarBuchungErstellung {
    pub klient_id: Uuid,
    pub rabatt: Decimal,
}

impl SeminarBuchungErstellung {
    pub fn into_use_case(
        self,
        termin_id: Uuid,
    ) -> Result<SeminarBuchungAnlegen, Report<ValidationError>> {
        Ok(SeminarBuchungAnlegen {
            termin_id: SeminarTerminId(termin_id),
            klient_id: KlientId(self.klient_id),
            rabatt: parse_ratio(self.rabatt)?,
        })
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarTerminAbsage {
    pub grund: String,
}

impl SeminarTerminAbsage {
    pub fn into_use_case(self, termin_id: Uuid) -> SeminarTerminAbsagen {
        SeminarTerminAbsagen {
            termin_id: SeminarTerminId(termin_id),
            grund: self.grund,
        }
    }
}

pub fn abgehalten_use_case(termin_id: Uuid) -> SeminarTerminAlsAbgehaltenMarkieren {
    SeminarTerminAlsAbgehaltenMarkieren {
        termin_id: SeminarTerminId(termin_id),
    }
}

pub fn buchung_id(id: Uuid) -> SeminarBuchungId {
    SeminarBuchungId(id)
}

fn parse_preis(decimal: Decimal) -> Result<Preis, Report<ValidationError>> {
    Preis::new(decimal).change_context(ValidationError)
}

fn parse_ratio(decimal: Decimal) -> Result<Ratio, Report<ValidationError>> {
    Ratio::new(decimal).change_context(ValidationError)
}
