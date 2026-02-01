use crate::presentation::state::AppState;
use axum::Router;

pub mod orders;
pub mod products;
pub mod response;
pub mod users;
pub mod validation;

pub fn app_router() -> Router<AppState> {
    Router::new()
        .nest("/users", users::routes::router())
        .nest("/products", products::routes::router())
        .nest("/orders", orders::routes::router())
}
