use crate::ResultReport;

use async_trait::async_trait;
mod pdf;
mod praxis;
mod use_cases;

pub use crate::application::ExecutionContext;
use error_stack::IntoReport;
pub use pdf::*;
pub use praxis::*;
pub use use_cases::*;

#[async_trait]
pub trait UseCase<Output> {
    type Error: IntoReport + Send;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> ResultReport<Output, <Self::Error as IntoReport>::Context>;
}
