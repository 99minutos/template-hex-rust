use std::task::{Context, Poll};

use axum::{
    http::{HeaderValue, Request},
    response::Response,
};
use futures::future::BoxFuture;
use opentelemetry::{trace::TraceContextExt, TraceId};
use tower::{Layer, Service};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Debug, Clone)]
pub struct RequestId(pub String);

#[derive(Clone, Default)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for RequestIdMiddleware<S>
where
    S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
    S::Error: Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let request_id = Span::current().context().span().span_context().trace_id();

        let trace_id_str = request_id.to_string();
        if request_id != TraceId::INVALID {
            req.extensions_mut().insert(RequestId(trace_id_str.clone()));
        }

        let future = self.inner.call(req);

        Box::pin(async move {
            let mut res = future.await?;

            if request_id != TraceId::INVALID {
                res.headers_mut()
                    .insert("x-trace-id", HeaderValue::from_str(&trace_id_str).unwrap());
            }
            Ok(res)
        })
    }
}
