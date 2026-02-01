use crate::application::{orders::OrdersService, products::ProductsService, users::UsersService};
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub users_service: Arc<UsersService>,
    pub products_service: Arc<ProductsService>,
    pub orders_service: Arc<OrdersService>,
}

impl FromRef<AppState> for Arc<UsersService> {
    fn from_ref(state: &AppState) -> Self {
        state.users_service.clone()
    }
}

impl FromRef<AppState> for Arc<ProductsService> {
    fn from_ref(state: &AppState) -> Self {
        state.products_service.clone()
    }
}

impl FromRef<AppState> for Arc<OrdersService> {
    fn from_ref(state: &AppState) -> Self {
        state.orders_service.clone()
    }
}
