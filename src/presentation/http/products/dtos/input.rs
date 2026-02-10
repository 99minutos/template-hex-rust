use crate::application::products::{CreateProduct, UpdateProductMetadata};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProductInput {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(range(min = 0.0, message = "Price must be non-negative"))]
    pub price: f64,

    #[validate(range(min = 0, message = "Stock must be non-negative"))]
    pub stock: i32,

    #[validate(length(min = 1, message = "Category is required"))]
    pub category: String,

    #[validate(length(min = 1, message = "SKU is required"))]
    pub sku: String,

    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProductMetadataInput {
    pub description: Option<String>,

    #[validate(length(min = 1, message = "Category is required"))]
    pub category: String,

    pub tags: Vec<String>,

    #[validate(length(min = 1, message = "SKU is required"))]
    pub sku: String,
}

impl From<CreateProductInput> for CreateProduct {
    fn from(dto: CreateProductInput) -> Self {
        Self {
            name: dto.name,
            price: dto.price,
            stock: dto.stock,
            category: dto.category,
            sku: dto.sku,
            description: dto.description,
            tags: dto.tags,
        }
    }
}

impl From<UpdateProductMetadataInput> for UpdateProductMetadata {
    fn from(dto: UpdateProductMetadataInput) -> Self {
        Self {
            description: dto.description,
            category: dto.category,
            tags: dto.tags,
            sku: dto.sku,
        }
    }
}
