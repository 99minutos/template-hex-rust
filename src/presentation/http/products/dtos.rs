use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::domain::products::Product;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProductDto {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
    #[validate(range(min = 0))]
    pub stock: i32,
}

#[derive(Serialize, ToSchema)]
pub struct ProductResponseDto {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}

impl From<Product> for ProductResponseDto {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.unwrap().to_hex(),
            name: product.name,
            price: product.price,
            stock: product.stock,
        }
    }
}
