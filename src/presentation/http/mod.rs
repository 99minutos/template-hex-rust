use crate::presentation::state::AppState;
use axum::Router;

pub mod error;
pub mod order;
pub mod product;
pub mod response;
pub mod user;
pub mod validation;

pub fn app_router() -> Router<AppState> {
    Router::new()
        .nest("/users", user::routes::router())
        .nest("/products", product::routes::router())
        .nest("/orders", order::routes::router())
}
