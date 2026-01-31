use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use opentelemetry::trace::TraceContextExt;
use serde::Serialize;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct GenericApiResponse<T> {
    #[schema(example = "0af7651916cd43dd8448eb211c80319c")]
    pub trace_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "Something went wrong")]
    pub cause: Option<String>,

    #[serde(skip)]
    #[schema(value_type = u16, example = 200)]
    pub status: StatusCode,
}

impl<T> IntoResponse for GenericApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl<T> GenericApiResponse<T> {

    fn get_current_trace_id() -> String {
        let context = tracing::Span::current().context();
        let span = context.span();
        let span_context = span.span_context();

        if span_context.is_valid() {
            format!("{:032x}", span_context.trace_id())
        } else {
            "00000000000000000000000000000000".to_string()
        }
    }

    pub fn success(data: T) -> Self {
        Self {
            trace_id: Self::get_current_trace_id(),
            data: Some(data),
            cause: None,
            status: StatusCode::OK,
        }
    }

    pub fn error(cause: String, status: StatusCode) -> Self {
        Self {
            trace_id: Self::get_current_trace_id(),
            data: None,
            cause: Some(cause),
            status,
        }
    }
}
