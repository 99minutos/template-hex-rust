mod config;

// Layered Architecture Modules
mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::infrastructure::providers::mongo::MongoProvider;
use crate::presentation::server::ServerLauncher;
use crate::presentation::state::AppState;
use std::sync::Arc;

use crate::application::{orders::OrdersService, products::ProductsService, users::UsersService};
use crate::infrastructure::persistence::{
    orders::OrdersRepository, products::ProductsRepository, users::UsersRepository,
};

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let env = config::get();

    if let Err(e) = crate::infrastructure::providers::telemetry::init_tracing().await {
        eprintln!("Failed to initialize tracing: {}", e);
    }

    tracing::info!("Starting {} (env: {})", env.service_name, env.app_env);

    // Initialize infrastructure
    let mongo = MongoProvider::new(&env.service_name, &env.mongo_url, &env.mongo_db).await;
    let db = mongo.get_database();

    // 1. Initialize Repositories
    let users_repo = Arc::new(UsersRepository::new(&db));
    let products_repo = Arc::new(ProductsRepository::new(&db));
    let orders_repo = Arc::new(OrdersRepository::new(&db));

    // 2. Initialize Services
    let users_service = Arc::new(UsersService::new(users_repo.clone()));
    let products_service = Arc::new(ProductsService::new(products_repo.clone()));
    let orders_service = Arc::new(OrdersService::new(orders_repo, users_repo, products_repo));

    // 3. Wire State
    let state = AppState {
        users_service,
        products_service,
        orders_service,
    };

    ServerLauncher::new(state).with_http(env.port).run().await;
}
