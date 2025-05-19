use std::sync::Arc;

use dotenv::dotenv;
use implementation::ExampleService;
use infrastructure::{
    http::{HttpProvider, HttpRouter},
    persistence::ExampleRepository,
    providers::MongoProvider,
};

mod domain;
mod envs;
mod implementation;
mod infrastructure;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let envs = crate::envs::get();
    let tracer = tools::init_tracer(envs.service_name.clone()).await;
    if tracer.is_ok() {
        tools::init_logger(tracer.unwrap());
    }

    // third party dependencies
    let mongodb = MongoProvider::new(envs.mongo_uri.clone(), envs.mongo_db.clone()).await?;

    // repositories
    let event_repository = Arc::new(ExampleRepository::new(&mongodb.get_database()).await);

    // services
    let context = AppContext {
        pubsub_service: Arc::new(ExampleService::new(event_repository)),
    };

    // start server
    let routes = HttpRouter::create_routes(context);
    let server = HttpProvider::new(envs.port, routes).await;
    server.run().await;

    Ok(())
}

#[derive(Clone)]
struct AppContext {
    pub pubsub_service: Arc<ExampleService>,
}
