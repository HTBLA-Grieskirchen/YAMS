use error_stack::Report;
use http::StatusCode;
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject,
    payload::{Json, PlainText},
    types::ToJSON,
};
use yams_core::{App, ThreadSafeError};

use crate::{
    api::YamsAppApi,
    errors::{InternalServerError, StructuredError},
    requests::{AnimalCreation, ClientCreation},
    schema::{Animal, Client},
};

pub struct YamsApiSpec {
    app_api: YamsAppApi,
}

impl YamsApiSpec {
    pub fn new(app_api: YamsAppApi) -> Self {
        Self { app_api }
    }
}

impl From<YamsAppApi> for YamsApiSpec {
    fn from(app_api: YamsAppApi) -> Self {
        Self::new(app_api)
    }
}

impl From<App> for YamsApiSpec {
    fn from(app: App) -> Self {
        Self::new(YamsAppApi::new(app))
    }
}

#[derive(poem_openapi::ApiResponse)]
pub enum TypicalJsonResponse<T: ToJSON> {
    #[oai(status = 200)]
    Ok(Json<T>),
    #[oai(status_range = "4XX")]
    ClientError(StatusCode, Json<StructuredError>),
    #[oai(status = 500)]
    InternalError(PlainText<InternalServerError>),
}

impl<T: ToJSON, C: ThreadSafeError> From<Result<T, Report<C>>> for TypicalJsonResponse<T> {
    fn from(result: Result<T, Report<C>>) -> Self {
        match result {
            Ok(value) => TypicalJsonResponse::Ok(Json(value)),
            Err(error) => {
                // extract StatusCode from error, default to 400
                let status = error
                    .request_value::<StatusCode>()
                    .next()
                    .unwrap_or(StatusCode::BAD_REQUEST);
                if status.is_server_error() {
                    return TypicalJsonResponse::InternalError(PlainText(InternalServerError));
                }
                TypicalJsonResponse::ClientError(status, Json(error.into()))
            }
        }
    }
}

#[OpenApi]
impl YamsApiSpec {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/client", method = "post")]
    async fn create_client(&self, body: Json<ClientCreation>) -> TypicalJsonResponse<Client> {
        self.app_api.create_client(body.0).await.into()
    }

    #[oai(path = "/animal", method = "post")]
    async fn create_animal(&self, body: Json<AnimalCreation>) -> TypicalJsonResponse<Animal> {
        self.app_api.create_animal(body.0).await.into()
    }

    #[oai(path = "/animal", method = "get")]
    async fn get_animals(&self) -> TypicalJsonResponse<Vec<Animal>> {
        self.app_api.get_all_animals().await.into()
    }
}

pub fn openapi_service(
    app: App,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<YamsApiSpec, ()> {
    let mut service = OpenApiService::new(
        YamsApiSpec::from(app),
        "YAMS API",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
