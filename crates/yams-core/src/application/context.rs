use std::sync::Arc;

use crate::{application::uow::UnitOfWork, ports::Clock};

// This struct is created fresh for EVERY service execution
pub struct ExecutionContext<'a> {
    // 1. The UnitOfWork is OWNED by the Registry
    // It is public so UseCases can call `registry.uow.animals()...`
    pub uow: UnitOfWork<'a>,
    // 2. Other ports are SHARED (Arc)
    // We use getter methods
    pub(super) clock: Arc<dyn Clock>,
}

impl ExecutionContext<'_> {
    pub fn clock(&self) -> &dyn Clock {
        self.clock.as_ref()
    }

    pub fn to_locked<'b>(&'b self) -> ExecutionContext<'b> {
        ExecutionContext {
            uow: self.uow.locked(),
            clock: Arc::clone(&self.clock),
        }
    }

    pub fn to_shared<'b>(&'b mut self) -> ExecutionContext<'b> {
        ExecutionContext {
            uow: self.uow.shared(),
            clock: Arc::clone(&self.clock),
        }
    }
}
