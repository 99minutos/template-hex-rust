use actix_cors::Cors;
use actix_web::{web, HttpResponse};
use tracing_actix_web::TracingLogger;

use crate::infrastructure::http::handlers::handler_example;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let example_scope = web::scope("/example")
        .service(handler_example::get_examples)
        .service(handler_example::add_random_example)
        .service(handler_example::get_examples_with_error);

    cfg.service(
        web::scope("")
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .route(
                "/healthz",
                web::get().to(|| async { HttpResponse::Ok().body("ok") }),
            )
            .service(web::scope("/api/v1").service(example_scope)),
    );
}
