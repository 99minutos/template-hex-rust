use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{InputRequest, OutputResponse};
use crate::domain::entities::Example;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateExampleRequest {
    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters"
    ))]
    pub name: String,
}

impl InputRequest for CreateExampleRequest {}
