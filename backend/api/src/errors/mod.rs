use thiserror::Error;

api_serde! {
    transparent;
    api_oai_derive_newtype!(#[derive(Error, Debug, Clone)] #[error("Internal Server Error occurred: {0}")] pub struct InternalServerError(pub String))
}

impl From<anyhow::Error> for InternalServerError {
    fn from(error: anyhow::Error) -> Self {
        Self(error.to_string())
    }
}
