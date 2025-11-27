use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::entities::Example;
use crate::infrastructure::http::dto::OutputResponse;

/// DTO de respuesta que representa un ejemplo.
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
