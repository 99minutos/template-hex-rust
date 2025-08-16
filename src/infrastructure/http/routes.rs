use axum::{http::Request, routing, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info_span;

use crate::AppContext;

use super::handlers::handler_example;
use super::middlewares::{RequestId, RequestIdLayer};

pub struct HttpRouter;

impl HttpRouter {
    pub fn create_routes(app_context: AppContext) -> Router {
        let examples = Router::new()
            .route("/", routing::get(handler_example::get_examples))
            .route(
                "/random",
                routing::post(handler_example::add_random_example),
            );

        let trace = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let method = request.method().clone();
            let uri = request.uri().to_string();
            let request_id = request
                .extensions()
                .get::<RequestId>()
                .map(|r| r.0.clone())
                .unwrap_or_else(|| "unknown".to_string());
            info_span!("http_request", %method, %uri, %request_id)
        });

        let routes = Router::new()
            .route("/healthz", routing::get(|| async { "ok" }))
            .nest("/api/v1/example", examples)
            .layer(trace)
            .layer(RequestIdLayer::default())
            .layer(CorsLayer::permissive())
            .with_state(app_context);

        routes
    }
}
