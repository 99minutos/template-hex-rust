use std::sync::Arc;

use dotenv::dotenv;
use implementation::ExampleService;
use infrastructure::{
    http::HttpProvider,
    persistence::ExampleRepository,
    providers::MongoProvider,
};

mod domain;
mod envs;
mod implementation;
mod infrastructure;
mod tools;

mod app_context {
    use crate::implementation::ExampleService;
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    pub struct AppContext {
        pub example_srv: Arc<ExampleService>,
    }
}

pub use app_context::AppContext;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let envs = crate::envs::get();
    let tracer = tools::init_tracer(envs.service_name.clone()).await;
    if tracer.is_ok() && envs.project_id.is_some() {
        tools::init_logger(tracer.unwrap(), envs.project_id.clone().unwrap());
    } else {
        tools::init_logger_without_trace()
    }

    let mongodb = MongoProvider::new(envs.mongo_uri.clone(), envs.mongo_db.clone())
        .await
        .expect("Failed to init Mongo");

    let event_repository: Arc<dyn crate::domain::ports::PortExampleRepo> =
        Arc::new(ExampleRepository::new(&mongodb.get_database()).await);

    let context = AppContext {
        example_srv: Arc::new(ExampleService::new(event_repository)),
    };

    HttpProvider::start_server(envs.port, context).await
}
