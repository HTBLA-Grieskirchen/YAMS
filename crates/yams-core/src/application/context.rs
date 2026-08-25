use std::sync::Arc;

use crate::{
    application::uow::UnitOfWork,
    ports::{Clock, ObjectStore, PdfRenderer},
};

// This struct is created fresh for EVERY service execution
pub struct ExecutionContext<'a> {
    // 1. The UnitOfWork is OWNED by the Registry
    // It is public so UseCases can call `registry.uow.animals()...`
    pub uow: UnitOfWork<'a>,
    // 2. Other ports are SHARED (Arc)
    // We use getter methods
    pub(super) clock: Arc<dyn Clock>,
    pub(super) object_store: Arc<dyn ObjectStore>,
    pub(super) pdf_renderer: Arc<dyn PdfRenderer>,
}

impl ExecutionContext<'_> {
    pub fn clock(&self) -> &dyn Clock {
        self.clock.as_ref()
    }

    pub fn object_store(&self) -> &dyn ObjectStore {
        self.object_store.as_ref()
    }

    pub fn pdf_renderer(&self) -> &dyn PdfRenderer {
        self.pdf_renderer.as_ref()
    }

    pub fn to_locked<'b>(&'b self) -> ExecutionContext<'b> {
        ExecutionContext {
            uow: self.uow.locked(),
            clock: Arc::clone(&self.clock),
            object_store: Arc::clone(&self.object_store),
            pdf_renderer: Arc::clone(&self.pdf_renderer),
        }
    }

    pub fn to_shared<'b>(&'b mut self) -> ExecutionContext<'b> {
        ExecutionContext {
            uow: self.uow.shared(),
            clock: Arc::clone(&self.clock),
            object_store: Arc::clone(&self.object_store),
            pdf_renderer: Arc::clone(&self.pdf_renderer),
        }
    }
}
