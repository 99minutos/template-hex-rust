use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::entities::Example2;
use crate::infrastructure::http::dto::OutputResponse;

/// DTO de respuesta que representa un ejemplo 2.
#[derive(Debug, Serialize)]
pub struct Example2Dto {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OutputResponse for Example2Dto {}

impl From<Example2> for Example2Dto {
    fn from(example: Example2) -> Self {
        Self {
            id: example.id.to_string(),
            name: example.name,
            created_at: example.created_at.to_chrono(),
            updated_at: example.updated_at.to_chrono(),
        }
    }
}
