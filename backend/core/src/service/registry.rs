use crate::application::uow::UnitOfWork;

// This struct is created fresh for EVERY request
pub struct Registry {
    // 1. The UnitOfWork is OWNED by the Registry
    // It is public so UseCases can call `registry.uow.animals()...`
    pub uow: UnitOfWork,
    // 2. Other ports are SHARED (Arc)
    // We use getter methods or public fields to access them
    //email: Arc<dyn EmailPort>,
}

impl Registry {
    // Helper accessors
    // pub fn email(&self) -> &dyn EmailPort {
    //     self.email.as_ref()
    // }
}
