use poem_openapi::{ApiResponse, NewType, Object};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(NewType, Serialize, Deserialize, Debug, Clone, Error)]
#[error("Internal Server Error occurred: {0}")]
pub struct InternalServerError(pub String);

impl From<anyhow::Error> for InternalServerError {
    fn from(error: anyhow::Error) -> Self {
        Self(error.to_string())
    }
}
