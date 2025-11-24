use dotenv::dotenv;
use implementation::{Example2Service, ExampleService};
use infrastructure::{
    http::HttpProvider,
    persistence::{Example2Repository, ExampleRepository},
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
    dotenv().ok();

    let envs = crate::envs::get();
    let tracer = tools::init_tracer(envs.service_name.clone()).await;
    if tracer.is_ok() && envs.project_id.is_some() {
        tools::init_logger(tracer.unwrap(), envs.project_id.clone().unwrap());
    } else {
        tools::init_logger_without_trace()
    }

    // initialize MongoDB Provider
    let mongodb = MongoProvider::new(envs.mongo_uri.clone(), envs.mongo_db.clone())
        .await
        .expect("Failed to init Mongo");

    // initialize repositories async
    let database = mongodb.get_database();

    let (example_rep, example2_rep) = tokio::join!(
        ExampleRepository::new(&database),
        Example2Repository::new(&database)
    );

    // initialize context

    let context = AppContext {
        example_srv: ExampleService::new(example_rep),
        example2_srv: Example2Service::new(example2_rep),
    };

    HttpProvider::start_server(envs.port, context).await
}
