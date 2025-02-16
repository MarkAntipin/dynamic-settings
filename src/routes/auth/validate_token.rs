use actix_web::web;
use actix_web::HttpResponse;
use crate::errors::CustomError;
use crate::models::{MessageResponse, ValidateTokenRequest};

pub async fn validate_token(
    api_key: web::Data<String>,
    payload: web::Json<ValidateTokenRequest>,
) -> Result<HttpResponse, CustomError> {
    let token = payload.into_inner().token;
    if token != api_key.to_string() {
        return Err(CustomError::ForbiddenError("Invalid token".to_string()));
    }
    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Token is valid".to_string(),
    }))
}
