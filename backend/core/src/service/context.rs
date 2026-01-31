use crate::application::uow::UnitOfWork;

// This struct is created fresh for EVERY request
pub struct ExecutionContext<'a> {
    // 1. The UnitOfWork is OWNED by the Registry
    // It is public so UseCases can call `registry.uow.animals()...`
    pub uow: UnitOfWork<'a>,
    // 2. Other ports are SHARED (Arc)
    // We use getter methods or public fields to access them
    //email: Arc<dyn EmailPort>,
}

impl ExecutionContext<'_> {
    // Helper accessors
    // pub fn email(&self) -> &dyn EmailPort {
    //     self.email.as_ref()
    // }

    pub fn to_locked<'b>(&'b self) -> ExecutionContext<'b> {
        ExecutionContext {
            uow: self.uow.locked(),
        }
    }

    pub fn to_shared<'b>(&'b mut self) -> ExecutionContext<'b> {
        ExecutionContext {
            uow: self.uow.shared(),
        }
    }
}
