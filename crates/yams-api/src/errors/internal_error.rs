use thiserror::Error;

#[derive(Debug, Error)]
#[error("Internal server error")]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct InternalServerError;

impl From<InternalServerError> for String {
    fn from(value: InternalServerError) -> Self {
        format!("{}", value)
    }
}
