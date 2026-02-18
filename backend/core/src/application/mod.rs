use error_stack::Report;

use crate::{
    application::uow::UnitOfWork,
    ports::repos::UnitOfWorkProvider,
    service::{
        errors::{MarkShouldRetry, PersistenceError, StableError},
        *,
    },
    use_cases::UseCase,
};

mod orchestration;
pub mod uow;

use orchestration::*;

pub struct AppConfiguration {
    pub max_attempts: u32,
}

impl Default for AppConfiguration {
    fn default() -> Self {
        Self { max_attempts: 3 }
    }
}

pub struct App {
    pub configuration: AppConfiguration,
    pub uow_provider: Box<dyn UnitOfWorkProvider>,
}

impl App {
    pub async fn execute<U: UseCase<O> + Clone + Send, O>(
        &self,
        use_case: U,
    ) -> Result<O, ExecutionError<U::Error>>
    where
        U::Error: Send,
    {
        let mut last_error = None;
        for _ in 1..=self.configuration.max_attempts {
            match self.try_execute(use_case.clone()).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let retry = match &e {
                        ExecutionError::UseCase(e) => {
                            e.request_ref::<MarkShouldRetry>().count() > 0
                        }
                        ExecutionError::Orchestration(e) => {
                            e.request_ref::<MarkShouldRetry>().count() > 0
                        }
                    };
                    last_error = Some(e);
                    if !retry {
                        break;
                    }
                }
            }
        }
        Err(last_error.unwrap())
    }

    #[inline(always)]
    pub async fn try_execute<U: UseCase<O> + Send, O>(
        &self,
        use_case: U,
    ) -> Result<O, ExecutionError<U::Error>>
    where
        U::Error: Send,
    {
        self.orchestrate(UseCaseOp(use_case)).await
    }

    pub async fn execute_fn<F, O, E: StableError>(&self, f: F) -> Result<O, ExecutionError<E>>
    where
        for<'a> F: AsyncFnOnce(ExecutionContext<'a>) -> Result<O, Report<E>> + Send,
        E: Send,
    {
        self.orchestrate(FnOp(f)).await
    }

    async fn orchestrate<T, O, E: StableError>(&self, op: T) -> Result<O, ExecutionError<E>>
    where
        T: OrchestrateFn<O, E>,
    {
        // 1. Create the UoW (Factory starts TX behind the scenes)
        let mut uow = self
            .uow_provider
            .begin()
            .await
            .map(UnitOfWork::new)
            .map_err(|e| ExecutionError::Orchestration(e))?;

        // 2. Run the operation
        let ctx = ExecutionContext { uow: uow.shared() };
        let result = op.run(ctx).await;

        // 3. Decide what to do
        match result {
            Ok(output) => {
                uow.commit()
                    .await
                    .map_err(|e| ExecutionError::Orchestration(e))?;
                Ok(output)
            }
            Err(e) => {
                uow.rollback()
                    .await
                    .map_err(|e| ExecutionError::Orchestration(e))?;
                Err(ExecutionError::UseCase(e))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError<E>
where
    E: StableError,
{
    // The UseCase failed logic (e.g., EmailTaken or DB Error inside logic)
    #[error(transparent)]
    UseCase(Report<E>),

    // The App failed to orchestrate (e.g., Commit failed)
    Orchestration(Report<PersistenceError>),
}
