use axum::{routing, Router};
use tower_http::cors::CorsLayer;

use crate::AppContext;

use super::handlers::handler_example;

pub struct HttpRouter;

impl HttpRouter {
    pub fn create_routes(app_context: AppContext) -> Router {
        // example routes
        let examples = Router::new().route("/", routing::get(handler_example::get_examples));

        // all routes handled by axum
        let routes = Router::new()
            .nest("/api/v1/example", examples)
            .layer(CorsLayer::permissive())
            .with_state(app_context);

        routes
    }
}
