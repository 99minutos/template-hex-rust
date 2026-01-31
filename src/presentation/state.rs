use std::sync::Arc;
use axum::extract::FromRef;
use crate::application::{
    users::UsersService,
    products::ProductsService,
    orders::OrdersService,
};

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
