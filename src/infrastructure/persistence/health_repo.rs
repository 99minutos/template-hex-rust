use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::doc;

use crate::domain::ports::PortHealthRepo;

#[derive(Debug, Clone)]
pub struct HealthRepository {
    db: mongodb::Database,
}

impl HealthRepository {
    pub fn new(db: &mongodb::Database) -> Arc<dyn PortHealthRepo> {
        Arc::new(Self { db: db.clone() })
    }
}

#[async_trait]
impl PortHealthRepo for HealthRepository {
    async fn check(&self) -> bool {
        match self.db.run_command(doc! { "ping": 1 }).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("Health check failed: {}", e);
                false
            }
        }
    }
}
