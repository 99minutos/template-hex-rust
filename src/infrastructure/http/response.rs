use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use opentelemetry::trace::TraceContextExt;
use serde::Serialize;
use serde_json::json;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::domain::{DomainError, DomainWrapper};

#[derive(Debug, Serialize)]
pub struct GenericApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(skip_serializing)]
    status: StatusCode,
}

impl<T> IntoResponse for GenericApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Json(json!(self));
        (self.status, body).into_response()
    }
}

impl<T> From<DomainWrapper<T>> for GenericApiResponse<T>
where
    T: Serialize,
{
    fn from(result: DomainWrapper<T>) -> Self {
        let trace_id = Span::current()
            .context()
            .span()
            .span_context()
            .trace_id()
            .to_string();

        match result {
            Ok(data) => Self {
                success: true,
                trace_id,
                data: Some(data),
                cause: None,
                status: StatusCode::OK,
            },
            Err(err) => {
                let status = Self::status_for_error(&err);
                Self {
                    success: false,
                    trace_id,
                    data: None,
                    cause: Some(err.message().to_string()),
                    status,
                }
            }
        }
    }
}

impl<T> From<T> for GenericApiResponse<T>
where
    T: Serialize,
{
    fn from(value: T) -> Self {
        Self {
            success: true,
            trace_id: Span::current()
                .context()
                .span()
                .span_context()
                .trace_id()
                .to_string(),
            data: Some(value),
            cause: None,
            status: StatusCode::OK,
        }
    }
}

impl<T> GenericApiResponse<T>
where
    T: Serialize,
{
    #[inline]
    fn current_trace_id() -> String {
        Span::current()
            .context()
            .span()
            .span_context()
            .trace_id()
            .to_string()
    }

    #[inline]
    fn status_for_error(err: &DomainError) -> StatusCode {
        match err {
            DomainError::NotFound(_) => StatusCode::NOT_FOUND,
            DomainError::Conflict(_) => StatusCode::CONFLICT,
            DomainError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            DomainError::Transient(_) => StatusCode::SERVICE_UNAVAILABLE,
            DomainError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
