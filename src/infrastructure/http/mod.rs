mod dto;
mod handlers;
mod parser;
mod routes;
use actix_web::{web, App, HttpServer};

use crate::AppContext;

mod response;

pub struct HttpProvider {}

impl HttpProvider {
    pub async fn start_server(port: u16, context: AppContext) -> Result<(), std::io::Error> {
        let addr = format!("0.0.0.0:{}", port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(context.clone()))
                .app_data(web::JsonConfig::default().error_handler(parser::json_handler))
                .app_data(web::QueryConfig::default().error_handler(parser::query_handler))
                .app_data(web::PathConfig::default().error_handler(parser::path_handler))
                .configure(routes::configure)
        })
        .bind(addr.clone())
        .expect("Failed to bind HTTP server")
        .run()
        .await
    }
}
