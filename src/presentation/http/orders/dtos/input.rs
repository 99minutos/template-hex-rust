use crate::application::orders::CreateOrder;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderInput {
    #[validate(length(equal = 24, message = "Invalid User ID format"))]
    pub user_id: String,

    #[validate(length(equal = 24, message = "Invalid Product ID format"))]
    pub product_id: String,

    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
}

impl From<CreateOrderInput> for CreateOrder {
    fn from(dto: CreateOrderInput) -> Self {
        Self {
            user_id: dto.user_id,
            product_id: dto.product_id,
            quantity: dto.quantity,
        }
    }
}
