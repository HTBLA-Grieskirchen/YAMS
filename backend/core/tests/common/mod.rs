pub mod fakes;

use std::sync::Arc;

use crate::common::fakes::{FakeDatastore, FakeUnitOfWorkProvider};
use yams_core::App;

pub struct TestAdapters {
    pub datastore: Arc<FakeDatastore>,
}

pub async fn make_testing_app() -> (App, TestAdapters) {
    let uow_provider = FakeUnitOfWorkProvider::empty();
    let adapters = TestAdapters {
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
