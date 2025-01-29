use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Debug, Display)]
pub enum CustomError {
    SerializeError(String),
    ValidationError(String),
    InternalError(String),
    UnauthorizedError(String),
    NotFoundError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<'a> {
    pub message: &'a str,
}

impl ErrorResponse<'_> {
    pub fn new(message: &str) -> String {
        let error_response = ErrorResponse { message };
        to_string_pretty(&error_response).unwrap()
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::SerializeError(_) => StatusCode::BAD_REQUEST,
            CustomError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            CustomError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::UnauthorizedError(_) => StatusCode::UNAUTHORIZED,
            CustomError::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        match self {
            CustomError::SerializeError(message)
            | CustomError::ValidationError(message)
            | CustomError::InternalError(message)
            | CustomError::UnauthorizedError(message)
            | CustomError::NotFoundError(message) => {
                HttpResponse::build(status).body(ErrorResponse::new(message))
            }
        }
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            _ => CustomError::InternalError("Internal Server Error".to_string()),
        }
    }
}
