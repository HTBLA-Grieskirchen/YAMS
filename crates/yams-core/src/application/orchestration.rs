use crate::{ResultReport, application::{ExecutionContext, UseCase}};
use error_stack::{IntoReport, Report};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("An error occurred while executing the use case")]
pub struct ExecutionError;

/// Wrappers to avoid overlapping impls; both run through the same orchestration path.
pub(crate) struct UseCaseOp<U>(pub U);
pub(crate) struct FnOp<F>(pub F);

pub(crate) trait OrchestrateFn<O, E>: Send {
    fn run<'a>(self, ctx: ExecutionContext<'a>) -> impl Future<Output = Result<O, E>>
    where
        Self: 'a;
}

impl<U, O> OrchestrateFn<O, Report<<U::Error as IntoReport>::Context>> for UseCaseOp<U>
where
    U: UseCase<O> + Send,
{
    async fn run<'a>(self, ctx: ExecutionContext<'a>) -> ResultReport<O, <U::Error as IntoReport>::Context>
    where
        Self: 'a,
    {
        self.0.perform(ctx).await.map_err(|err| err.into_report())
    }
}

impl<F, O, E> OrchestrateFn<O, Report<E>> for FnOp<F>
where
    F: for<'a> AsyncFnOnce(ExecutionContext<'a>) -> ResultReport<O, E> + Send,
    E: Send,
{
    fn run<'a>(self, ctx: ExecutionContext<'a>) -> impl Future<Output = ResultReport<O, E>>
    where
        Self: 'a,
    {
        async move { self.0(ctx).await }
    }
}
