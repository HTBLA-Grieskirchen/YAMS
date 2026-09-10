use error_stack::{IntoReport, Report, ResultExt};

pub type ResultReport<T, E> = Result<T, Report<E>>;

pub trait ThreadSafeError: std::error::Error + Send + Sync + 'static {}

impl<E: std::error::Error + Send + Sync + 'static> ThreadSafeError for E {}

pub trait ErrorReportExt<T, E: ThreadSafeError> {
    fn contextualize<C: ThreadSafeError>(self, context: C) -> Result<T, Report<C>>;

    fn contextualize_with<C: ThreadSafeError>(
        self,
        context_fn: impl Fn(&E) -> C,
    ) -> Result<T, Report<C>>;
}

impl<T, E: ThreadSafeError> ErrorReportExt<T, E> for Result<T, E> {
    fn contextualize<C: ThreadSafeError>(self, context: C) -> Result<T, Report<C>> {
        self.map_err(IntoReport::into_report)
            .change_context(context)
    }

    fn contextualize_with<C: ThreadSafeError>(
        self,
        context_fn: impl Fn(&E) -> C,
    ) -> Result<T, Report<C>> {
        self.map_err(IntoReport::into_report).map_err(|e| {
            let new_context = context_fn(e.current_context());
            e.change_context(new_context)
        })
    }
}
