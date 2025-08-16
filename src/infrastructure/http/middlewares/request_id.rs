use std::task::{Context, Poll};

use axum::{
    http::{HeaderName, HeaderValue, Request},
    response::Response,
};
use futures::future::BoxFuture;
use rand::{distr::Alphanumeric, Rng};
use tower::{Layer, Service};

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
        let mut rng = rand::rng();

        let request_id = if let Some(val) = req.headers().get("x-request-id") {
            match val.to_str() {
                Ok(s) if !s.is_empty() => s.to_string(),
                _ => rng
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect::<String>(),
            }
        } else {
            rng.sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect::<String>()
        };

        req.extensions_mut().insert(RequestId(request_id.clone()));

        let mut svc = self.inner.clone();
        let fut = svc.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if let Ok(hv) = HeaderValue::from_str(&request_id) {
                res.headers_mut()
                    .insert(HeaderName::from_static("x-request-id"), hv);
            }
            Ok(res)
        })
    }
}
