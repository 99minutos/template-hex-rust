use crate::domain::orders::Order;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderDto {
    #[validate(length(min = 24, max = 24, message = "Invalid User ID"))]
    pub user_id: String,
    #[validate(length(min = 24, max = 24, message = "Invalid Product ID"))]
    pub product_id: String,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

#[derive(Serialize, ToSchema)]
pub struct OrderResponseDto {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub total_price: f64,
    pub created_at: String,
}

impl From<Order> for OrderResponseDto {
    fn from(order: Order) -> Self {
        Self {
            id: order.id.unwrap().to_hex(),
            user_id: order.user_id.to_hex(),
            product_id: order.product_id.to_hex(),
            quantity: order.quantity,
            total_price: order.total_price,
            created_at: order.created_at.to_chrono().to_rfc3339(),
        }
    }
}
