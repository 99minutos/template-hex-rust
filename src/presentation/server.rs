use crate::presentation::openapi::ApiDoc;
use axum::{Router, extract::DefaultBodyLimit};
use std::net::SocketAddr;
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    decompression::RequestDecompressionLayer,
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config;
use crate::presentation::http;
use crate::presentation::state::AppState;

pub struct ServerLauncher {
    state: AppState,
    http_port: Option<u16>,
}

impl ServerLauncher {
    pub fn new(state: AppState) -> Self {
        Self {
            state,
            http_port: None,
        }
    }

    pub fn with_http(mut self, port: u16) -> Self {
        self.http_port = Some(port);
        self
    }

    pub async fn run(self) {
        let env = config::get();

        if let Some(port) = self.http_port {
            let state = self.state.clone();

            let cors = if env.cors_origins == "*" {
                CorsLayer::permissive()
                    .allow_methods(Any)
                    .allow_headers(Any)
            } else {
                let origins: Vec<_> = env
                    .cors_origins
                    .split(',')
                    .map(|s| s.parse().expect("Invalid CORS origin"))
                    .collect();

                CorsLayer::new()
                    .allow_methods(Any)
                    .allow_headers(Any)
                    .allow_origin(origins)
            };

            let mut router_builder = Router::new().nest("/api/v1", http::app_router());

            if env.app_env != "PRD" {
                router_builder = router_builder.merge(
                    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
                );
            }

            let rest_router = router_builder
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(RequestDecompressionLayer::new())
                .layer(DefaultBodyLimit::max(32 * 1024 * 1024))
                .layer(cors)
                .with_state(state);

            let rest_addr = SocketAddr::from(([0, 0, 0, 0], port));
            tracing::info!("REST Server listening on {}", rest_addr);
            if env.app_env != "PRD" {
                tracing::info!(
                    "Swagger UI available at http://localhost:{}/swagger-ui",
                    port
                );
            }

            let listener = tokio::net::TcpListener::bind(rest_addr).await.unwrap();
            axum::serve(listener, rest_router)
                .with_graceful_shutdown(shutdown_signal("REST"))
                .await
                .unwrap();
        }
    }
}

async fn shutdown_signal(name: &str) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!(
        "Signal received, starting graceful shutdown for {}...",
        name
    );
}
