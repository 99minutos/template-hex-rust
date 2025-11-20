pub mod dto;
pub mod error;
pub mod handlers;
// mod parser;
pub mod response;
pub mod routes;

use crate::AppContext;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

pub struct HttpProvider;

impl HttpProvider {
    pub async fn start_server(port: u16, context: AppContext) -> Result<(), std::io::Error> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr.clone()).await?;

        let app = routes::app(Arc::new(context));

        tracing::info!("Server started at {}", addr);

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
    }
}

async fn shutdown_signal() {
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

    tracing::info!("Signal received, starting graceful shutdown");
}
