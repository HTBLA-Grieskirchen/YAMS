use std::sync::Arc;

use bon::Builder;
use error_stack::IntoReport;
use error_stack::{Report, ResultExt};

use crate::adapters::SystemClock;
use crate::application::uow::UnitOfWork;
use crate::ports::Clock;
use crate::service::UseCase;
use crate::uow::UnitOfWorkProvider;

mod orchestration;
pub mod uow;

mod context;
pub mod ports;
pub use context::ExecutionContext;
use orchestration::*;

mod errors;
pub use errors::ErrorReportExt;
pub use errors::ResultReport;
pub use errors::ThreadSafeError;

#[derive(Builder)]
pub struct App {
    pub uow_provider: Box<dyn UnitOfWorkProvider>,
    #[builder(default = Arc::new(SystemClock))]
    pub clock: Arc<dyn Clock>,
}

impl App {
    #[inline(always)]
    pub async fn execute<U: UseCase<O> + Send, O>(
        &self,
        use_case: U,
    ) -> ResultReport<O, ExecutionError> {
        self.orchestrate(UseCaseOp(use_case)).await
    }

    pub async fn execute_fn<F, O, E: ThreadSafeError>(
        &self,
        f: F,
    ) -> ResultReport<O, ExecutionError>
    where
        for<'a> F: AsyncFnOnce(ExecutionContext<'a>) -> ResultReport<O, E> + Send,
        E: Send,
    {
        self.orchestrate(FnOp(f)).await
    }

    async fn orchestrate<T, O, E: IntoReport>(&self, op: T) -> ResultReport<O, ExecutionError>
    where
        T: OrchestrateFn<O, E>,
    {
        // 1. Create the UoW (Factory starts TX behind the scenes)
        let mut uow = self
            .uow_provider
            .begin()
            .await
            .map(UnitOfWork::new)
            .change_context(ExecutionError)?;

        // 2. Run the operation
        let ctx = self.new_execution_context(uow.shared());
        let result = op.run(ctx).await;

        // 3. Decide what to do
        match result {
            Ok(output) => {
                uow.commit().await.change_context(ExecutionError)?;
                Ok(output)
            }
            Err(e) => {
                let mut e = e
                    .into_report()
                    .change_context(ExecutionError)
                    .attach("UseCase failed")
                    .expand();
                if let Err(rollback_error) = uow.rollback().await {
                    e.push(
                        rollback_error
                            .change_context(ExecutionError)
                            .attach("Rollback failed"),
                    );
                }
                Err(e.change_context(ExecutionError))
            }
        }
    }

    fn new_execution_context<'a>(&self, uow: UnitOfWork<'a>) -> ExecutionContext<'a> {
        ExecutionContext {
            uow,
            clock: Arc::clone(&self.clock),
        }
    }
}
