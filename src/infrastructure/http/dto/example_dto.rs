use super::OutputResponse;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entities::Example;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleDto {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OutputResponse for ExampleDto {}


impl From<Example> for ExampleDto {
    fn from(example: Example) -> Self {
        Self {
            id: example.id.to_string(),
            name: example.name,
            created_at: example.created_at.to_chrono(),
            updated_at: example.updated_at.to_chrono(),
        }
    }
}

impl From<ExampleDto> for Example {
    fn from(example_dto: ExampleDto) -> Self {
        Self {
            id: ObjectId::parse_str(&example_dto.id).unwrap(),
            name: example_dto.name,
            created_at: bson::DateTime::from_chrono(example_dto.created_at),
            updated_at: bson::DateTime::from_chrono(example_dto.updated_at),
        }
    }
}
