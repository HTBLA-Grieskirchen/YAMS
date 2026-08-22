use thiserror::Error;

#[derive(Debug, Error)]
#[error("Validation error")]
pub struct ValidationError;
