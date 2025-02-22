use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use fjall;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Debug, Display)]
pub enum CustomError {
    SerializeError(String),
    ValidationError(String),
    InternalError(String),
    UnauthorizedError(String),
    ForbiddenError(String),
    NotFoundError(String),
    ConflictError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<'a> {
    pub message: &'a str,
}

impl ErrorResponse<'_> {
    pub fn to_json(message: &str) -> String {
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
            CustomError::ForbiddenError(_) => StatusCode::FORBIDDEN,
            CustomError::NotFoundError(_) => StatusCode::NOT_FOUND,
            CustomError::ConflictError(_) => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        match self {
            CustomError::SerializeError(message)
            | CustomError::ValidationError(message)
            | CustomError::InternalError(message)
            | CustomError::UnauthorizedError(message)
            | CustomError::ForbiddenError(message)
            | CustomError::ConflictError(message)
            | CustomError::NotFoundError(message) => {
                HttpResponse::build(status).body(ErrorResponse::to_json(message))
            }
        }
    }
}

impl From<fjall::Error> for CustomError {
    fn from(_: fjall::Error) -> Self {
        CustomError::InternalError("Internal Server Error".to_string())
    }
}
