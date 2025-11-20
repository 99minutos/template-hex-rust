use std::sync::Arc;

use async_trait::async_trait;
use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

use crate::domain::{self, entities, ports, DomainWrapper};

#[derive(Debug, Clone)]
pub struct ExampleRepository {
    db: Collection<entities::Example>,
}

impl ExampleRepository {
    pub async fn new(client: &mongodb::Database) -> Arc<dyn ports::PortExampleRepo> {
        let collection = client.collection::<entities::Example>("examples");
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
impl ports::PortExampleRepo for ExampleRepository {
    #[tracing::instrument(skip_all)]
    async fn all(&self) -> DomainWrapper<Vec<entities::Example>> {
        let filter = doc! {};

        match self.db.find(filter).await {
            Ok(cursor) => {
                let events: Vec<entities::Example> = cursor.try_collect().await.map_err(|e| {
                    domain::DomainError::new(
                        domain::ErrorKind::Database(domain::DatabaseKind::Error),
                        format!("Failed to get examples: {}", e),
                    )
                })?;
                Ok(events)
            }
            Err(e) => Err(domain::DomainError::new(
                domain::ErrorKind::Database(domain::DatabaseKind::Error),
                format!("Failed to fetch example: {}", e),
            )),
        }
    }

    #[tracing::instrument(skip_all)]
    async fn insert(&self, example: entities::Example) -> DomainWrapper<entities::Example> {
        let mut example = example.clone();
        let now = Utc::now();
        example.id = ObjectId::new();
        example.created_at = DateTime::from_chrono(now);
        example.updated_at = DateTime::from_chrono(now);

        let result = self.db.insert_one(example.clone()).await;
        match result {
            Ok(_) => Ok(example),
            Err(e) => Err(domain::DomainError::new(
                domain::ErrorKind::Database(domain::DatabaseKind::Error),
                format!("Failed to get examples: {}", e),
            )),
        }
    }
}
