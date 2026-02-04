use serde::{Deserialize, Serialize};
use serde_with::{IfIsHumanReadable, serde_as};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<IfIsHumanReadable<serde_with::DisplayFromStr>>")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    pub price: f64,
    pub stock: i32,
}
