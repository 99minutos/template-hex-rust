use crate::application::{order::OrderService, product::ProductService, user::UserService};
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub product_service: Arc<ProductService>,
    pub order_service: Arc<OrderService>,
}

impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(state: &AppState) -> Self {
        state.user_service.clone()
    }
}

impl FromRef<AppState> for Arc<ProductService> {
    fn from_ref(state: &AppState) -> Self {
        state.product_service.clone()
    }
}

impl FromRef<AppState> for Arc<OrderService> {
    fn from_ref(state: &AppState) -> Self {
        state.order_service.clone()
    }
}
