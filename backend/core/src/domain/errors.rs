pub trait DomainError: std::error::Error {}

#[derive(thiserror::Error, Debug)]
#[error("this should never happen")]
pub struct NoError {
    _private: (),
}

impl DomainError for NoError {}
