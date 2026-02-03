use crate::service::{ExecutionContext, UseCase};

/// Wrappers to avoid overlapping impls; both run through the same orchestration path.
pub(crate) struct UseCaseOp<U>(pub U);
pub(crate) struct FnOp<F>(pub F);

pub(crate) trait OrchestrateFn<O, E>: Send {
    fn run<'a>(self, ctx: ExecutionContext<'a>) -> impl Future<Output = Result<O, E>>
    where
        Self: 'a;
}

impl<U, O> OrchestrateFn<O, U::Error> for UseCaseOp<U>
where
    U: UseCase<O> + Send,
    U::Error: Send,
{
    fn run<'a>(self, ctx: ExecutionContext<'a>) -> impl Future<Output = Result<O, U::Error>>
    where
        Self: 'a,
    {
        self.0.perform(ctx)
    }
}

impl<F, O, E> OrchestrateFn<O, E> for FnOp<F>
where
    F: for<'a> AsyncFnOnce(ExecutionContext<'a>) -> Result<O, E> + Send,
    E: Send,
{
    fn run<'a>(self, ctx: ExecutionContext<'a>) -> impl Future<Output = Result<O, E>>
    where
        Self: 'a,
    {
        async move { self.0(ctx).await }
    }
}
