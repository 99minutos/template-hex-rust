use crate::domain::error::Error as DomainError;
use crate::presentation::http::response::GenericApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Presentation layer error type that bridges Domain errors to HTTP responses.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation failed: {0}")]
    BadRequest(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Internal server error")]
    Internal(String),
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::ValidationError(msg) => ApiError::BadRequest(msg),
            DomainError::InvalidId(msg) => ApiError::BadRequest(msg),
            DomainError::NotFound(msg) => ApiError::NotFound(msg),
            DomainError::Conflict(msg) => ApiError::Conflict(msg),
            DomainError::Unauthorized(msg) => ApiError::Unauthorized(msg),
            DomainError::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                ApiError::Internal("Database error occurred".to_string())
            }
            DomainError::InternalError(msg) => {
                tracing::error!("Internal error: {:?}", msg);
                ApiError::Internal("Internal server error".to_string())
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        GenericApiResponse::<()>::error(message, status).into_response()
    }
}
