use crate::domain::products::{Product, ProductMetadata, ProductStatus};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProductDto {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
    #[validate(range(min = 0))]
    pub stock: i32,
    pub category: String,
    pub sku: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProductMetadataDto {
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub category: String,
    pub tags: Vec<String>,
    #[validate(length(min = 1))]
    pub sku: String,
}

impl From<UpdateProductMetadataDto> for ProductMetadata {
    fn from(dto: UpdateProductMetadataDto) -> Self {
        Self {
            description: dto.description,
            category: dto.category,
            tags: dto.tags,
            sku: dto.sku,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ProductResponseDto {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub status: ProductStatus,
    pub metadata: ProductMetadata,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Product> for ProductResponseDto {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.map(|id| id.to_hex()).unwrap_or_default(),
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
