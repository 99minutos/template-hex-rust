use crate::presentation::state::AppState;
use axum::Router;

pub mod users;
pub mod products;
pub mod orders;
pub mod validation;
pub mod response;

pub fn app_router() -> Router<AppState> {
    Router::new()
        .nest("/users", users::routes::router())
        .nest("/products", products::routes::router())
        .nest("/orders", orders::routes::router())
}