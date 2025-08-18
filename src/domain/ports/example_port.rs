use std::fmt::Debug;

use async_trait::async_trait;

use crate::domain::{entities, DomainWrapper};

#[async_trait]
pub trait PortExampleRepo: Debug + Send + Sync {
    async fn all(&self) -> DomainWrapper<Vec<entities::Example>>;
    async fn insert(&self, example: entities::Example) -> DomainWrapper<entities::Example>;
}
