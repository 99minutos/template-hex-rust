use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{IfIsHumanReadable, serde_as};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    #[default]
    Draft,
    Active,
    Archived,
    OutOfStock,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema)]
pub struct ProductMetadata {
    pub description: Option<String>,
    pub category: String,
    pub tags: Vec<String>,
    pub sku: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<IfIsHumanReadable<serde_with::DisplayFromStr>>")]
    #[schema(value_type = Option<String>)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub status: ProductStatus,
    pub metadata: ProductMetadata,
    #[serde_as(as = "crate::infrastructure::serde::chrono_bson::ChronoAsBson")]
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "crate::infrastructure::serde::chrono_bson::ChronoAsBson")]
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}
