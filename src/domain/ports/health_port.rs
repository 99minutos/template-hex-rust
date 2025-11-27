use std::fmt::Debug;

use async_trait::async_trait;

#[async_trait]
pub trait PortHealthRepo: Debug + Send + Sync {
    async fn check(&self) -> bool;
}
