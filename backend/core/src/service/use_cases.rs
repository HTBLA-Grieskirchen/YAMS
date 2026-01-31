use crate::{
    application::OrchestratableError, domain::errors::DomainError,
    service::{Registry, errors::ServiceError},
};
use async_trait::async_trait;

pub mod animals;
pub mod client;

#[async_trait]
pub trait UseCase<Output>: Clone {
    type Error: OrchestratableError;

    async fn perform(
        self,
        registry: &mut Registry,
    ) -> Result<Output, Self::Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum UseCaseError<D, I> {
    Domain(
        #[source]
        #[backtrace]
        D,
    ),
    Service(
        #[source]
        #[backtrace]
        I,
    ),
}

pub trait DomainToUseCaseError: Sized {
    fn into_domain<I: ServiceError + Sized>(self) -> UseCaseError<Self, I>;
}

pub trait ServiceToUseCaseError: Sized {
    fn into_service<D: DomainError + Sized>(self) -> UseCaseError<D, Self>;
}

impl<D: DomainError + Sized> DomainToUseCaseError for D {
    fn into_domain<I: ServiceError + Sized>(self) -> UseCaseError<D, I> {
        UseCaseError::Domain(self)
    }
}

impl<I: ServiceError + Sized> ServiceToUseCaseError for I {
    fn into_service<D: DomainError + Sized>(self) -> UseCaseError<D, I> {
        UseCaseError::Service(self)
    }
}

pub type UseCaseResult<Output, DomainError, InfrastructureError> =
    Result<Output, UseCaseError<DomainError, InfrastructureError>>;
