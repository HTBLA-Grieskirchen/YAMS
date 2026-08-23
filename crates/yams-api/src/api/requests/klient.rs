use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use http::StatusCode;
use yams_core::service::KlientErstellen;

use crate::{errors::ValidationError, schema::Adresse};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct KlientErstellung {
    pub vorname: String,
    pub nachname: String,
    pub geburtstag: NaiveDate,
    pub email: String,
    pub mobilnummer: String,
    pub kundennummer: i64,
    pub einwilligung: bool,
    pub adresse: Adresse,
}

impl TryFrom<KlientErstellung> for KlientErstellen {
    type Error = Report<ValidationError>;
    fn try_from(value: KlientErstellung) -> Result<Self, Self::Error> {
        Ok(Self {
            vorname: value.vorname,
            nachname: value.nachname,
            geburtstag: value.geburtstag,
            email: value
                .email
                .try_into()
                .change_context(ValidationError)
                .attach_opaque(StatusCode::UNPROCESSABLE_ENTITY)?,
            mobilnummer: value
                .mobilnummer
                .try_into()
                .change_context(ValidationError)
                .attach_opaque(StatusCode::UNPROCESSABLE_ENTITY)?,
            kundennummer: value.kundennummer,
            einwilligung: value.einwilligung,
            adresse: value
                .adresse
                .try_into()
                .change_context(ValidationError)
                .attach_opaque(StatusCode::UNPROCESSABLE_ENTITY)?,
        })
    }
}
