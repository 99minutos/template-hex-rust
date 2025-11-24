use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example2 {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Example2 {
    fn default() -> Self {
        Self {
            id: ObjectId::new(),
            name: "example2".to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
