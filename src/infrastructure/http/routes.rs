use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

use crate::{infrastructure::http::handlers::handler_example, AppContext};

pub fn app(context: Arc<AppContext>) -> Router {
    let example_routes = Router::new()
        .route("/", get(handler_example::get_examples))
        .route("/random", post(handler_example::add_random_example))
        .route("/error", get(handler_example::get_examples_with_error));

    let api_routes = Router::new().nest("/example", example_routes);

    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .nest("/api/v1", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(context)
}
