use super::OutputResponse;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entities::Example2;

#[derive(Debug, Serialize, Deserialize)]
pub struct Example2Dto {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OutputResponse for Example2Dto {}

impl From<Example2> for Example2Dto {
    fn from(example2: Example2) -> Self {
        Self {
            id: example2.id.to_string(),
            name: example2.name,
            created_at: example2.created_at.to_chrono(),
            updated_at: example2.updated_at.to_chrono(),
        }
    }
}

impl From<Example2Dto> for Example2 {
    fn from(example2_dto: Example2Dto) -> Self {
        Self {
            id: ObjectId::parse_str(&example2_dto.id).unwrap(),
            name: example2_dto.name,
            created_at: bson::DateTime::from_chrono(example2_dto.created_at),
            updated_at: bson::DateTime::from_chrono(example2_dto.updated_at),
        }
    }
}
