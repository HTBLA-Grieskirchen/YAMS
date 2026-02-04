use crate::{
    application::uow::UnitOfWork, ports::repos::UnitOfWorkProvider, service::*, use_cases::UseCase,
};
use anyhow::anyhow;

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
        for _attempt in 0..self.configuration.max_attempts {
            match self.try_execute(use_case.clone()).await {
                Ok(result) => return Ok(result),
                Err(ExecutionError::UseCase(e)) if e.should_retry() => {
                    println!("Error during execution: {e:?}");
                }
                Err(e) => return Err(e),
            }
        }
        Err(ExecutionError::Orchestration(
            OrchestrationError::MaxAttemptsReached,
        ))
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

    pub async fn execute_fn<F, O, E>(&self, f: F) -> Result<O, ExecutionError<E>>
    where
        for<'a> F: AsyncFnOnce(ExecutionContext<'a>) -> Result<O, E> + Send,
        E: Send,
    {
        self.orchestrate(FnOp(f)).await
    }

    async fn orchestrate<T, O, E>(&self, op: T) -> Result<O, ExecutionError<E>>
    where
        T: OrchestrateFn<O, E>,
    {
        // 1. Create the UoW (Factory starts TX behind the scenes)
        let mut uow = self
            .uow_provider
            .begin()
            .await
            .map(UnitOfWork::new)
            .map_err(|e| OrchestrationError::Orchestration(anyhow!(e)))?;

        // 2. Run the operation
        let ctx = ExecutionContext { uow: uow.shared() };
        let result = op.run(ctx).await;

        // 3. Decide what to do
        match result {
            Ok(output) => {
                uow.commit()
                    .await
                    .map_err(|e| OrchestrationError::Orchestration(anyhow!(e)))?;
                Ok(output)
            }
            Err(e) => {
                uow.rollback()
                    .await
                    .map_err(|e| OrchestrationError::Orchestration(anyhow!(e)))?;
                Err(ExecutionError::UseCase(e))
            }
        }
    }
}

pub trait OrchestratableError {
    fn should_retry(&self) -> bool;
}

#[derive(thiserror::Error, Debug)]
pub enum OrchestrationError {
    #[error("max attempts reached")]
    MaxAttemptsReached,
    #[error(transparent)]
    Orchestration(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError<E> {
    // The UseCase failed logic (e.g., EmailTaken or DB Error inside logic)
    #[error(transparent)]
    UseCase(E),

    // The App failed to orchestrate (e.g., Commit failed)
    #[error(transparent)]
    Orchestration(#[from] OrchestrationError),
}
