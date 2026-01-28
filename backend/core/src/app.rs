use crate::{
    service::{errors::ServiceError, ports::uow::UnitOfWorkProvider, *},
    use_cases::{UseCase, UseCaseError, UseCaseResult},
};

pub struct AppConfiguration {
    max_attempts: u32,
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
    pub async fn execute<U: UseCase<O>, O>(
        &self,
        use_case: U,
    ) -> ExecutionResult<O, U::DomainError, U::ServiceError> {
        for _attempt in 0..self.configuration.max_attempts {
            match self.try_execute(use_case.clone()).await {
                Ok(result) => return Ok(result),
                Err(UseCaseError::Service(e)) if !e.should_retry() => {
                    return Err(UseCaseError::Service(e));
                }
                Err(UseCaseError::Domain(e)) => return Err(UseCaseError::Domain(e)),
                _ => {}
            }
        }
        Err(UseCaseError::Service(ExecutionError::Orchestration(
            anyhow::anyhow!("max attempts reached"),
        )))
    }

    #[inline(always)]
    async fn try_execute<U: UseCase<O>, O>(
        &self,
        use_case: U,
    ) -> ExecutionResult<O, U::DomainError, U::ServiceError> {
        // 1. Create the UoW (Factory starts the TX behind the scenes) and Registry
        let uow = match self.uow_provider.begin().await {
            Ok(uow) => uow,
            Err(e) => {
                return Err(UseCaseError::Service(ExecutionError::Orchestration(
                    e.into(),
                )));
            }
        };
        let mut registry = Registry { uow };

        // 2. Run UseCase
        let result = use_case.perform(&mut registry).await;

        // 3. Decide what to do (rollback by default)
        match registry.uow.rollback().await {
            Ok(_) => result.map_err(|e| match e {
                UseCaseError::Domain(e) => UseCaseError::Domain(e),
                UseCaseError::Service(e) => UseCaseError::Service(ExecutionError::from(e)),
            }),
            Err(e) => {
                Err(UseCaseError::Service(ExecutionError::Orchestration(
                    e.into(),
                )))
            }
        }
    }
}

pub type ExecutionResult<O, D, I> = UseCaseResult<O, D, ExecutionError<I>>;

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError<I: ServiceError> {
    #[error(transparent)]
    Service(#[from] I),
    #[error("orchestration failed")]
    Orchestration(
        #[source]
        #[backtrace]
        anyhow::Error,
    ),
}

impl<I: ServiceError> ServiceError for ExecutionError<I> {
    fn should_retry(&self) -> bool {
        match self {
            ExecutionError::Service(e) => e.should_retry(),
            ExecutionError::Orchestration(_) => true,
        }
    }
}
