use crate::domain::entities::example::Example;
use bson::DateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ExampleDto {
    pub id: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<Example> for ExampleDto {
    fn from(example: Example) -> Self {
        ExampleDto {
            id: example.id.to_hex(), // Convert ObjectId to hex string
            name: example.name,
            created_at: example.created_at,
            updated_at: example.updated_at,
        }
    }
}