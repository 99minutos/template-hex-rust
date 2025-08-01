use std::fmt::Debug;

use async_trait::async_trait;

use crate::domain::entity;

#[async_trait]
pub trait PortExampleRepo: Debug + Send + Sync {
    async fn all(&self) -> Result<Vec<entity::Example>, String>;
    async fn insert(
        &self,
        example: entity::Example,
    ) -> Result<entity::Example, mongodb::error::Error>;
}
