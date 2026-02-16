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

use crate::application::{order::OrderService, product::ProductService, user::UserService};
use crate::domain::ports::{
    order::OrderRepositoryPort, product::ProductRepositoryPort, user::UserRepositoryPort,
};
use crate::infrastructure::persistence::{
    order::repository::OrderRepository, product::repository::ProductRepository,
    user::repository::UserRepository,
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
    let user_repo = Arc::new(UserRepository::new(&db));
    let product_repo = Arc::new(ProductRepository::new(&db));
    let order_repo = Arc::new(OrderRepository::new(&db));

    // 2. Create database indexes (idempotent - safe to run on every startup)
    tracing::info!("Creating database indexes...");
    if let Err(e) = user_repo.create_indexes().await {
        tracing::error!("Failed to create user indexes: {}", e);
    }
    if let Err(e) = product_repo.create_indexes().await {
        tracing::error!("Failed to create product indexes: {}", e);
    }
    if let Err(e) = order_repo.create_indexes().await {
        tracing::error!("Failed to create order indexes: {}", e);
    }

    // 3. Initialize Services
    let user_service = Arc::new(UserService::new(
        user_repo.clone() as Arc<dyn UserRepositoryPort>
    ));
    let product_service = Arc::new(ProductService::new(
        product_repo.clone() as Arc<dyn ProductRepositoryPort>
    ));
    let order_service = Arc::new(OrderService::new(
        order_repo as Arc<dyn OrderRepositoryPort>,
        user_repo as Arc<dyn UserRepositoryPort>,
        product_repo as Arc<dyn ProductRepositoryPort>,
    ));

    // 4. Wire State
    let state = AppState {
        user_service,
        product_service,
        order_service,
    };

    ServerLauncher::new(state).with_http(env.port).run().await;
}
