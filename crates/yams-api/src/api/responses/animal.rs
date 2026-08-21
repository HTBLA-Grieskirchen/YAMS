use poem_openapi::payload::Json;

use crate::{errors::InternalServerError, schema::Animal};

#[cfg_attr(feature = "openapi", derive(poem_openapi::ApiResponse))]
pub enum CreateAnimalResponse {
    #[cfg_attr(feature = "openapi", oai(status = 200))]
    Ok(Json<Animal>),
    #[cfg_attr(feature = "openapi", oai(status = 500))]
    InternalError(Json<InternalServerError>),
}