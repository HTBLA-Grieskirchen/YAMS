pub mod fakes;

use std::sync::{Arc, Mutex};

use crate::common::fakes::{FakeDatastore, FakeUnitOfWorkProvider, UoWEvent};
use yams_core::App;

pub struct TestAdapters {
    pub uow_log: Arc<Mutex<Vec<UoWEvent>>>,
    pub datastore: Arc<FakeDatastore>,
}

pub async fn make_testing_app() -> (App, TestAdapters) {
    let uow_provider = FakeUnitOfWorkProvider::empty();
    let adapters = TestAdapters {
        uow_log: Arc::clone(&uow_provider.log),
        datastore: Arc::clone(&uow_provider.datastore),
    };
    (
        App {
            configuration: Default::default(),
            uow_provider: Box::new(uow_provider),
        },
        adapters,
    )
}
