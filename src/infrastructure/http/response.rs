#![allow(dead_code)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use opentelemetry::trace::TraceContextExt;
use serde::Serialize;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::domain::{self, DomainError, DomainWrapper};

#[derive(Debug, Serialize)]
pub struct GenericApiResponse {
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(skip_serializing)]
    status: StatusCode,
}

impl IntoResponse for GenericApiResponse {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl<T> From<DomainWrapper<T>> for GenericApiResponse
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
                trace_id,
                data: serde_json::to_value(data).ok(),
                cause: None,
                status: StatusCode::OK,
            },
            Err(err) => {
                let status = Self::status_for_error(&err);
                Self {
                    trace_id,
                    data: err.data().cloned(),
                    cause: Some(err.message().to_string()),
                    status,
                }
            }
        }
    }
}

impl From<DomainError> for GenericApiResponse {
    fn from(err: DomainError) -> Self {
        let trace_id = Self::current_trace_id();
        let status = Self::status_for_error(&err);

        Self {
            trace_id,
            data: err.data().cloned(),
            cause: Some(err.message().to_string()),
            status,
        }
    }
}

impl GenericApiResponse {
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
    fn status_for_error(err: &domain::DomainError) -> StatusCode {
        match err.kind() {
            domain::ErrorKind::NotFound => StatusCode::NOT_FOUND,
            domain::ErrorKind::Conflict => StatusCode::CONFLICT,
            domain::ErrorKind::Validation => StatusCode::UNPROCESSABLE_ENTITY,
            domain::ErrorKind::Database(_) => StatusCode::SERVICE_UNAVAILABLE,
            domain::ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            domain::ErrorKind::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn from_error(err: &str, status: StatusCode) -> Self {
        let trace_id = Span::current()
            .context()
            .span()
            .span_context()
            .trace_id()
            .to_string();

        Self {
            trace_id,
            data: None,
            cause: Some(err.to_string()),
            status,
        }
    }
}
