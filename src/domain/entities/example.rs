use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
