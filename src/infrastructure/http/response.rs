#![allow(dead_code)]
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
pub struct GenericApiResponse {
    pub success: bool,
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
        let body = Json(json!(self));
        (self.status, body).into_response()
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
                success: true,
                trace_id,
                data: serde_json::to_value(data).ok(),
                cause: None,
                status: StatusCode::OK,
            },
            Err(err) => {
                let status = Self::status_for_error(&err);
                Self {
                    success: false,
                    trace_id,
                    data: err.data().cloned(),
                    cause: Some(err.message().to_string()),
                    status,
                }
            }
        }
    }
}

impl GenericApiResponse {
    pub fn success<T: Serialize>(value: T) -> Self {
        Self {
            success: true,
            trace_id: Span::current()
                .context()
                .span()
                .span_context()
                .trace_id()
                .to_string(),
            data: serde_json::to_value(value).ok(),
            cause: None,
            status: StatusCode::OK,
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
    fn status_for_error(err: &DomainError) -> StatusCode {
        match err {
            DomainError::NotFound { .. } => StatusCode::NOT_FOUND,
            DomainError::Conflict { .. } => StatusCode::CONFLICT,
            DomainError::Validation { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            DomainError::Transient { .. } => StatusCode::SERVICE_UNAVAILABLE,
            DomainError::Unknown { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
