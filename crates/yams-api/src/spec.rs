use poem_openapi::{LicenseObject, OpenApi, OpenApiService, ServerObject, payload::Json};

use crate::{YamsApi, requests::AnimalCreation, requests::ClientCreation, responses::CreateAnimalResponse, responses::CreateClientResponse};

pub struct YamsApiSpec<I: YamsApi>(I);

impl<I: YamsApi> YamsApiSpec<I> {
    pub fn new(inner: I) -> Self {
        Self(inner)
    }
}

impl<I: YamsApi> From<I> for YamsApiSpec<I> {
    fn from(inner: I) -> Self {
        Self::new(inner)
    }
}

#[OpenApi]
impl<I: YamsApi> YamsApiSpec<I> {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/client", method = "post")]
    async fn create_client(&self, body: Json<ClientCreation>) -> CreateClientResponse {
        self.0.create_client(body.0).await
    }

    #[oai(path = "/animal", method = "post")]
    async fn create_animal(&self, body: Json<AnimalCreation>) -> CreateAnimalResponse {
        self.0.create_animal(body.0).await
    }
}

pub fn openapi_service<A: YamsApi>(
    api: A,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<YamsApiSpec<A>, ()> {
    let mut service = OpenApiService::new(
        YamsApiSpec::from(api),
        "YAMS API",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
