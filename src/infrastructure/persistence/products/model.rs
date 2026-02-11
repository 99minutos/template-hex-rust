use crate::domain::products::{Product, ProductId, ProductMetadata, ProductStatus};
use crate::infrastructure::serde::chrono_bson::ChronoAsBson;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{IfIsHumanReadable, serde_as};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<IfIsHumanReadable<serde_with::DisplayFromStr>>")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub status: ProductStatus,
    pub metadata: ProductMetadata,
    #[serde_as(as = "ChronoAsBson")]
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "ChronoAsBson")]
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<ChronoAsBson>")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<Product> for ProductDocument {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.and_then(|id| ObjectId::parse_str(&*id).ok()),
            name: product.name,
            price: product.price,
            stock: product.stock,
            status: product.status,
            metadata: product.metadata,
            created_at: product.created_at,
            updated_at: product.updated_at,
            deleted_at: product.deleted_at,
        }
    }
}

impl From<ProductDocument> for Product {
    fn from(doc: ProductDocument) -> Self {
        Self {
            id: doc.id.map(|oid| ProductId::new(oid.to_hex())),
            name: doc.name,
            price: doc.price,
            stock: doc.stock,
            status: doc.status,
            metadata: doc.metadata,
            created_at: doc.created_at,
            updated_at: doc.updated_at,
            deleted_at: doc.deleted_at,
        }
    }
}
