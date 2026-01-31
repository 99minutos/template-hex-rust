use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub user_id: ObjectId,
    pub product_id: ObjectId,
    pub quantity: i32,
    pub total_price: f64,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}
