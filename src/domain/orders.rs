use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub user_id: ObjectId,
    pub product_id: ObjectId,
    pub quantity: i32,
    pub total_price: f64,
    #[serde_as(as = "crate::infrastructure::serde::chrono_bson::ChronoAsBson")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "crate::infrastructure::serde::chrono_bson::ChronoAsBson")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
