use core::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug)]
pub enum HttpError {
    Custom(StatusCode, String, Option<Value>),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(_, msg, _) => write!(f, "{msg}"),
        }
    }
}

#[derive(Serialize)]
struct HttpErrorResponse {
    cause: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, message, data) = match self {
            Self::Custom(status, msg, data) => (status, msg, data),
        };

        let error_response = Json(HttpErrorResponse {
            cause: message,
            data,
        });

        (status, error_response).into_response()
    }
}

impl From<crate::domain::errors::DomainError> for HttpError {
    fn from(err: crate::domain::errors::DomainError) -> Self {
        match err {
            crate::domain::errors::DomainError::NotFound(m) => {
                HttpError::Custom(StatusCode::NOT_FOUND, m, None)
            }
            crate::domain::errors::DomainError::Conflict(m) => {
                HttpError::Custom(StatusCode::CONFLICT, m, None)
            }
            crate::domain::errors::DomainError::Validation(m) => {
                HttpError::Custom(StatusCode::BAD_REQUEST, m, None)
            }
            crate::domain::errors::DomainError::Transient(m) => {
                HttpError::Custom(StatusCode::BAD_GATEWAY, m, None)
            }
            crate::domain::errors::DomainError::Unknown(m) => {
                HttpError::Custom(StatusCode::INTERNAL_SERVER_ERROR, m, None)
            }
        }
    }
}
