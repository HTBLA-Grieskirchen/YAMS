use crate::{ResultReport, application::ExecutionContext};

use async_trait::async_trait;
mod use_cases;

use error_stack::IntoReport;
pub use use_cases::*;

#[async_trait]
pub trait UseCase<Output> {
    type Error: IntoReport + Send;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> ResultReport<Output, <Self::Error as IntoReport>::Context>;
}
