use std::sync::Arc;

use async_trait::async_trait;
use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

use crate::domain::{self, entities, ports, DomainWrapper};

#[derive(Debug, Clone)]
pub struct Example2Repository {
    db: Collection<entities::Example2>,
}

impl Example2Repository {
    pub async fn new(client: &mongodb::Database) -> Arc<dyn ports::PortExample2Repo> {
        let collection = client.collection::<entities::Example2>("example2s");
        let a = Self { db: collection };
        a.create_index().await;
        Arc::new(a)
    }

    async fn create_index(&self) {
        self.db
            .create_indexes(vec![
                IndexModel::builder()
                    .keys(doc! {
                        "created_at": -1
                    })
                    .options(
                        IndexOptions::builder()
                            .name(String::from("created_at_idx"))
                            .build(),
                    )
                    .build(),
                IndexModel::builder()
                    .keys(doc! {
                        "updated_at": -1
                    })
                    .options(
                        IndexOptions::builder()
                            .name(String::from("updated_at_idx"))
                            .build(),
                    )
                    .build(),
            ])
            .await
            .unwrap();
    }
}

#[async_trait]
impl ports::PortExample2Repo for Example2Repository {
    #[tracing::instrument(skip_all)]
    async fn all(&self) -> DomainWrapper<Vec<entities::Example2>> {
        let filter = doc! {};

        match self.db.find(filter).await {
            Ok(cursor) => {
                let events: Vec<entities::Example2> = cursor.try_collect().await.map_err(|e| {
                    domain::DomainError::new(
                        domain::ErrorKind::Database(domain::DatabaseKind::Error),
                        format!("Failed to get example2s: {}", e),
                    )
                })?;
                Ok(events)
            }
            Err(e) => Err(domain::DomainError::new(
                domain::ErrorKind::Database(domain::DatabaseKind::Error),
                format!("Failed to fetch example2: {}", e),
            )),
        }
    }

    #[tracing::instrument(skip_all)]
    async fn insert(&self, mut example2: entities::Example2) -> DomainWrapper<entities::Example2> {
        let now = Utc::now();
        example2.id = ObjectId::new();
        example2.created_at = DateTime::from_chrono(now);
        example2.updated_at = DateTime::from_chrono(now);

        let result = self.db.insert_one(&example2).await;
        match result {
            Ok(_) => Ok(example2),
            Err(e) => Err(domain::DomainError::new(
                domain::ErrorKind::Database(domain::DatabaseKind::Error),
                format!("Failed to insert example2: {}", e),
            )),
        }
    }
}
