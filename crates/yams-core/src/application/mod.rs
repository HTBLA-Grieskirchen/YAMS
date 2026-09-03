use std::sync::Arc;

use crate::adapters::{BlankPdfRenderer, InMemoryObjectStore, SystemClock};
use crate::ports::{Clock, ObjectStore, PdfRenderer};
use crate::service::UseCase;
use crate::uow::UnitOfWorkProvider;
use error_stack::IntoReport;
use tracing::instrument;

pub mod uow;

mod context;
pub mod ports;
pub use context::ExecutionContext;

mod instrumented;

mod errors;
pub use errors::ErrorReportExt;
pub use errors::ResultReport;
pub use errors::ThreadSafeError;

#[derive(bon::Builder)]
pub struct App {
    pub uow_provider: Box<dyn UnitOfWorkProvider>,
    #[builder(default = Arc::new(SystemClock))]
    pub clock: Arc<dyn Clock>,
    #[builder(default = Arc::new(InMemoryObjectStore::new()))]
    pub object_store: Arc<dyn ObjectStore>,
    #[builder(default = Arc::new(BlankPdfRenderer))]
    pub pdf_renderer: Arc<dyn PdfRenderer>,
}

pub use app_builder::{SetClock, SetUowProvider};

impl App {
    #[instrument(
        skip(self, use_case),
        fields(use_case = std::any::type_name::<U>()),
        err(Debug)
    )]
    pub async fn execute<U: UseCase<O> + Send, O>(
        &self,
        use_case: U,
    ) -> ResultReport<O, <U::Error as IntoReport>::Context> {
        use_case.perform(self.new_execution_context()).await
    }

    #[instrument(skip(self, f), fields(execute_fn = true), err(Debug))]
    pub async fn execute_fn<F, O, E: ThreadSafeError>(&self, f: F) -> ResultReport<O, E>
    where
        for<'a> F: AsyncFnOnce(ExecutionContext<'a>) -> ResultReport<O, E> + Send,
        E: Send,
    {
        f(self.new_execution_context()).await
    }

    fn new_execution_context(&self) -> ExecutionContext<'_> {
        ExecutionContext::root(
            self.uow_provider.as_ref(),
            Arc::clone(&self.clock),
            Arc::clone(&self.object_store),
            Arc::clone(&self.pdf_renderer),
        )
    }
}
