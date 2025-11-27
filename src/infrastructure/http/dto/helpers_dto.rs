use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{de::DeserializeOwned, Serialize};
use validator::Validate;

use super::{InputRequest, OutputResponse};
use crate::infrastructure::http::response::GenericApiResponse;

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub total: u64,
    pub data: Vec<T>,
}
impl<T> OutputResponse for PaginatedResponse<T> {}

impl<T: OutputResponse> OutputResponse for Vec<T> {}
impl OutputResponse for () {}

/// Wrapper para validar automáticamente el JSON de entrada.
#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[derive(Debug)]
pub enum ValidationError {
    JsonParse(JsonRejection),
    Validation(validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let (message, status) = match &self {
            ValidationError::JsonParse(rejection) => {
                (rejection.body_text(), StatusCode::BAD_REQUEST)
            }
            ValidationError::Validation(errors) => {
                (format!("{}", errors), StatusCode::UNPROCESSABLE_ENTITY)
            }
        };

        GenericApiResponse::<()>::from_error(&message, status).into_response()
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: InputRequest + DeserializeOwned + Validate,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state)
            .await
            .map_err(ValidationError::JsonParse)?;

        payload.validate().map_err(ValidationError::Validation)?;

        Ok(ValidatedJson(payload))
    }
}
