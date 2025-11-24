#![allow(dead_code)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use opentelemetry::trace::TraceContextExt;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::domain::{self, DomainError, DomainWrapper};

#[derive(Debug)]
pub enum GenericApiResponse<T> {
    Ok {
        trace_id: String,
        data: T,
        status: StatusCode,
    },
    Err {
        trace_id: String,
        data: Option<serde_json::Value>,
        cause: Option<String>,
        status: StatusCode,
    },
}

impl<T> Serialize for GenericApiResponse<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GenericApiResponse", 3)?;
        match self {
            Self::Ok { trace_id, data, .. } => {
                state.serialize_field("trace_id", trace_id)?;
                state.serialize_field("data", data)?;
                state.serialize_field("cause", &None::<String>)?;
            }
            Self::Err {
                trace_id,
                data,
                cause,
                ..
            } => {
                state.serialize_field("trace_id", trace_id)?;
                state.serialize_field("data", data)?;
                state.serialize_field("cause", cause)?;
            }
        }
        state.end()
    }
}

impl<T> IntoResponse for GenericApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = match &self {
            Self::Ok { status, .. } => *status,
            Self::Err { status, .. } => *status,
        };
        (status, Json(self)).into_response()
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
            Ok(data) => Self::Ok {
                trace_id,
                data,
                status: StatusCode::OK,
            },
            Err(err) => {
                let status = Self::status_for_error(&err);
                Self::Err {
                    trace_id,
                    data: err.data().cloned(),
                    cause: Some(err.message().to_string()),
                    status,
                }
            }
        }
    }
}

impl From<DomainError> for GenericApiResponse<()> {
    fn from(err: DomainError) -> Self {
        let trace_id = Self::current_trace_id();
        let status = Self::status_for_error(&err);

        Self::Err {
            trace_id,
            data: err.data().cloned(),
            cause: Some(err.message().to_string()),
            status,
        }
    }
}

impl<T> GenericApiResponse<T> {
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
}

impl GenericApiResponse<()> {
    pub fn from_error(err: &str, status: StatusCode) -> Self {
        let trace_id = Self::current_trace_id();

        Self::Err {
            trace_id,
            data: None,
            cause: Some(err.to_string()),
            status,
        }
    }
}
