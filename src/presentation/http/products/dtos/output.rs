use crate::domain::products::{Product, ProductMetadata, ProductStatus};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ProductOutput {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub status: ProductStatus,
    pub metadata: ProductMetadata,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Product> for ProductOutput {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.unwrap_or_default(),
            name: product.name,
            price: product.price,
            stock: product.stock,
            status: product.status,
            metadata: product.metadata,
            created_at: product.created_at.to_rfc3339(),
            updated_at: product.updated_at.to_rfc3339(),
        }
    }
}
