use crate::domain::orders::Order;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct OrderOutput {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub total_price: f64,
    pub created_at: String,
}

impl From<Order> for OrderOutput {
    fn from(order: Order) -> Self {
        Self {
            id: order.id.unwrap_or_default(),
            user_id: order.user_id,
            product_id: order.product_id,
            quantity: order.quantity,
            total_price: order.total_price,
            created_at: order.created_at.to_rfc3339(),
        }
    }
}
