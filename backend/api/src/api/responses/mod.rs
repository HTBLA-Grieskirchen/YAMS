#[derive(ApiResponse)]
pub enum CreateAnimalResponse {
    #[oai(status = 200)]
    Ok(Json<Animal>),
    #[oai(status = 500)]
    InternalError(Json<InternalServerError>),
}
