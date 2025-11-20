use crate::{domain::DomainError, infrastructure::http::response::GenericApiResponse};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Domain(err) => GenericApiResponse::from(err).into_response(),
            AppError::Unexpected(err) => {
                tracing::error!("Unexpected error: {:#}", err);
                GenericApiResponse::from_error(
                    "Internal Server Error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response()
            }
        }
    }
}
