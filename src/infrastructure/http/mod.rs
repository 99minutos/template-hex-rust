pub mod dto;
pub mod handlers;
// mod parser;
pub mod response;
pub mod routes;

use crate::AppContext;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct HttpProvider;

impl HttpProvider {
    pub async fn start_server(port: u16, context: AppContext) -> Result<(), std::io::Error> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr).await?;

        let app = routes::app(Arc::new(context));

        axum::serve(listener, app).await
    }
}
