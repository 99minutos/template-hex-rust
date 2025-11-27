use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, decompression::RequestDecompressionLayer,
    trace::TraceLayer,
};

use crate::{
    infrastructure::http::handlers::{handler_example, handler_example2},
    AppContext,
};

async fn health_check(State(context): State<Arc<AppContext>>) -> impl axum::response::IntoResponse {
    if context.health_srv.check().await {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "error")
    }
}

pub fn app(context: Arc<AppContext>) -> Router {
    let example_routes = Router::new()
        .route("/", get(handler_example::get_examples))
        .route("/", post(handler_example::create_example))
        .route("/paginated", get(handler_example::get_examples_paginated))
        .route("/random", post(handler_example::add_random_example))
        .route("/error", get(handler_example::get_examples_with_error));

    let example2_routes = Router::new()
        .route("/", get(handler_example2::get_example2s))
        .route("/random", post(handler_example2::add_random_example2))
        .route("/error", get(handler_example2::get_example2s_with_error));

    let api_routes = Router::new()
        .nest("/example", example_routes)
        .nest("/example2", example2_routes)
        .route(
            "/test/{value}",
            get(
                |axum::extract::Path(value): axum::extract::Path<String>| async move {
                    format!("You sent: {}", value)
                },
            ),
        );

    Router::new()
        .route("/healthz", get(health_check))
        .nest("/api/v1", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(RequestDecompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(context)
}
