use async_trait::async_trait;
use yams_api::openapi_service;
use yams_core::{
    App,
    ports::RepositoryResult,
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

struct Unimplemented;

#[async_trait]
impl UnitOfWorkProvider for Unimplemented {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        unimplemented!()
    }
}

fn main() {
    let dummy_app = App::builder().uow_provider(Box::new(Unimplemented)).build();

    let api_service = openapi_service(dummy_app, std::iter::empty::<String>());
    println!("{}", api_service.spec());
}
