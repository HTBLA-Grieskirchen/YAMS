use poem_openapi::payload::Json;

use crate::{errors::InternalServerError, schema::Client};

#[cfg_attr(feature = "openapi", derive(poem_openapi::ApiResponse))]
pub enum CreateClientResponse {
    #[oai(status = 200)]
    Ok(Json<Client>),
    #[oai(status = 500)]
    InternalError(Json<InternalServerError>),
}
