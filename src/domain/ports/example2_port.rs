use std::fmt::Debug;

use async_trait::async_trait;

use crate::domain::{entities, DomainWrapper};

#[async_trait]
pub trait PortExample2Repo: Debug + Send + Sync {
    async fn all(&self) -> DomainWrapper<Vec<entities::Example2>>;
    async fn insert(&self, example2: entities::Example2) -> DomainWrapper<entities::Example2>;
}
