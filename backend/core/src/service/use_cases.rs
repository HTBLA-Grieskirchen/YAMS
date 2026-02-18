use crate::service::{ExecutionContext, errors::StableError};
use async_trait::async_trait;
use error_stack::Report;

pub mod animals;
pub mod client;

#[async_trait]
pub trait UseCase<Output> {
    type Error: StableError;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Output, Report<Self::Error>>;
}
