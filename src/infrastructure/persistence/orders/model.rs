use crate::domain::orders::Order;
use crate::infrastructure::serde::chrono_bson::ChronoAsBson;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{IfIsHumanReadable, serde_as};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<IfIsHumanReadable<serde_with::DisplayFromStr>>")]
    pub id: Option<ObjectId>,
    #[serde_as(as = "IfIsHumanReadable<serde_with::DisplayFromStr>")]
    pub user_id: ObjectId,
    #[serde_as(as = "IfIsHumanReadable<serde_with::DisplayFromStr>")]
    pub product_id: ObjectId,
    pub quantity: i32,
    pub total_price: f64,
    #[serde_as(as = "ChronoAsBson")]
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "ChronoAsBson")]
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<ChronoAsBson>")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<Order> for OrderDocument {
    type Error = crate::domain::error::DomainError;

    fn try_from(order: Order) -> Result<Self, Self::Error> {
        let id = order.id.and_then(|id| ObjectId::parse_str(&id).ok());

        let user_id = ObjectId::parse_str(&order.user_id).map_err(|_| {
            crate::domain::error::Error::invalid_param("user_id", "User", &order.user_id)
        })?;

        let product_id = ObjectId::parse_str(&order.product_id).map_err(|_| {
            crate::domain::error::Error::invalid_param("product_id", "Product", &order.product_id)
        })?;

        Ok(Self {
            id,
            user_id,
            product_id,
            quantity: order.quantity,
            total_price: order.total_price,
            created_at: order.created_at,
            updated_at: order.updated_at,
            deleted_at: order.deleted_at,
        })
    }
}

impl From<OrderDocument> for Order {
    fn from(doc: OrderDocument) -> Self {
        Self {
            id: doc.id.map(|oid| oid.to_hex()),
            user_id: doc.user_id.to_hex(),
            product_id: doc.product_id.to_hex(),
            quantity: doc.quantity,
            total_price: doc.total_price,
            created_at: doc.created_at,
            updated_at: doc.updated_at,
            deleted_at: doc.deleted_at,
        }
    }
}
