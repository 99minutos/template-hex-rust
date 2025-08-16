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
        tools::init_logger(tracer.unwrap(), envs.project_id.clone());
    } else {
        tools::init_logger_without_trace()
    }

    let mongodb = MongoProvider::new(envs.mongo_uri.clone(), envs.mongo_db.clone()).await?;

    let event_repository = ExampleRepository::new(&mongodb.get_database()).await;

    let context = AppContext {
        example_srv: Arc::new(ExampleService::new(event_repository)),
    };

    let routes = HttpRouter::create_routes(context);
    let server = HttpProvider::new(envs.port, routes).await;
    server.run().await;

    Ok(())
}

#[derive(Debug, Clone)]
struct AppContext {
    pub example_srv: Arc<ExampleService>,
}
