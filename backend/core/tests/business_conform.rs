mod cases;
mod common;

use cases::TestAdapters;
use yams_core::App;

use crate::common::fakes::FakeUnitOfWorkProvider;

pub async fn make_testing_app() -> (App, TestAdapters) {
    let uow_provider = FakeUnitOfWorkProvider::empty();
    let adapters = TestAdapters {};
    (
        App {
            configuration: Default::default(),
            uow_provider: Box::new(uow_provider),
        },
        adapters,
    )
}
