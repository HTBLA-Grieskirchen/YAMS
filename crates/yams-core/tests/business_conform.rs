mod cases;
mod common;

use cases::TestAdapters;
use yams_core::App;

use crate::common::fakes::FakeUnitOfWorkProvider;

pub async fn make_testing_app(adapters: &TestAdapters) -> App {
    let uow_provider = FakeUnitOfWorkProvider::empty();
    App::builder()
        .uow_provider(Box::new(uow_provider))
        .maybe_clock(adapters.clock.clone())
        .build()
}
