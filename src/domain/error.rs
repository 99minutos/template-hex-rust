use crate::presentation::http::response::GenericApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Entity Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid Identifier: {0}")]
    InvalidId(String),

    #[error("Database Error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Internal Error: {0}")]
    InternalError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Error::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Error::InvalidId(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Error::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            Error::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            Error::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            Error::InternalError(msg) => {
                tracing::error!("Internal error: {:?}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        GenericApiResponse::<()>::error(message, status).into_response()
    }
}
