use std::fmt::Debug;

use async_trait::async_trait;

use crate::domain::entities;

#[async_trait]
pub trait PortExampleRepo: Debug + Send + Sync {
    async fn all(&self) -> Result<Vec<entities::Example>, String>;
    async fn insert(
        &self,
        example: entities::Example,
    ) -> Result<entities::Example, mongodb::error::Error>;
}
