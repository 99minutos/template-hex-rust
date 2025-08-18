use crate::infrastructure::http::dto::OutputDto;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use opentelemetry::trace::TraceContextExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::domain::DomainWrapper;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericApiResponse<T>
where
    T: Serialize + OutputDto,
{
    pub success: bool,
    pub trace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

impl<T> IntoResponse for GenericApiResponse<T>
where
    T: Serialize + OutputDto,
{
    fn into_response(self) -> Response {
        let status_code = if self.success {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        let body = Json(json!(self));
        (status_code, body).into_response()
    }
}

impl<T> From<DomainWrapper<T>> for GenericApiResponse<T>
where
    T: Serialize + OutputDto,
{
    fn from(result: DomainWrapper<T>) -> Self {
        let trace_id = Span::current().context().span().span_context().trace_id();
        let trace_id = trace_id.to_string();

        match result {
            Ok(data) => Self {
                success: true,
                trace_id,
                data: Some(data),
                cause: None,
            },
            Err(err) => Self {
                success: false,
                trace_id,
                data: None,
                cause: Some(err.message().to_string()),
            },
        }
    }
}
