use std::fmt::Debug;

use async_trait::async_trait;

use crate::domain::{entities, DomainResult};

#[async_trait]
pub trait PortExampleRepo: Debug + Send + Sync {
    async fn all(&self) -> DomainResult<Vec<entities::Example>>;
    async fn insert(&self, example: entities::Example) -> DomainResult<entities::Example>;
}
