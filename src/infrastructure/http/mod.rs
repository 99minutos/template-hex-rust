mod dto;
mod error;
mod handlers;
mod middlewares;
mod routes;
use axum::{serve::Serve, Router};

pub use error::HttpError;
pub use routes::HttpRouter;

use tokio::net::TcpListener;

pub struct HttpProvider {
    addr: String,
    server: Serve<TcpListener, Router, Router>,
}

impl HttpProvider {
    pub async fn new(port: u16, routes: Router) -> Self {
        let addr = format!("0.0.0.0:{}", port);
        let listerener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        let server = axum::serve(listerener, routes);

        Self { addr, server }
    }

    pub async fn run(self) {
        tracing::info!("Listening on {}", self.addr);
        self.server.await.unwrap();
    }
}
