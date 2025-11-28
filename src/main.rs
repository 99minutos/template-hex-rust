use implementation::{Example2Service, ExampleService, HealthService};
use infrastructure::{
    http::HttpProvider,
    persistence::{Example2Repository, ExampleRepository, HealthRepository},
    providers::MongoProvider,
};

mod ctx;
mod domain;
mod envs;
mod implementation;
mod infrastructure;
mod tools;

pub use ctx::AppContext;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let envs = crate::envs::get();

    if let Err(e) = tools::init_tracing().await {
        tracing::error!("Failed to initialize tracing: {}", e);
    }

    let mongodb = MongoProvider::new(envs.mongo_uri.clone(), envs.mongo_db.clone())
        .await
        .expect("Failed to init Mongo");

    let database = mongodb.get_database();

    let (example_rep, example2_rep) = tokio::join!(
        ExampleRepository::new(&database),
        Example2Repository::new(&database)
    );

    let health_rep = HealthRepository::new(&database);

    let context = AppContext {
        example_srv: ExampleService::new(example_rep),
        example2_srv: Example2Service::new(example2_rep),
        health_srv: HealthService::new(health_rep),
    };

    HttpProvider::start_server(envs.port, context).await
}
