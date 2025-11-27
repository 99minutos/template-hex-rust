use std::sync::Arc;

use async_trait::async_trait;
use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

use crate::domain::{self, entities, ports, DomainWrapper, Paginated, Pagination};

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
        let _ = self
            .db
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
            .await;
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
    async fn find_paginated(
        &self,
        pagination: &Pagination,
    ) -> DomainWrapper<Paginated<entities::Example>> {
        let filter = doc! {};

        let total = self.db.count_documents(filter.clone()).await.map_err(|e| {
            domain::DomainError::new(
                domain::ErrorKind::Database(domain::DatabaseKind::Error),
                format!("Failed to count examples: {}", e),
            )
        })?;

        let options = mongodb::options::FindOptions::builder()
            .skip(pagination.skip())
            .limit(pagination.limit as i64)
            .sort(doc! { "created_at": -1 })
            .build();

        let cursor = self
            .db
            .find(filter)
            .with_options(options)
            .await
            .map_err(|e| {
                domain::DomainError::new(
                    domain::ErrorKind::Database(domain::DatabaseKind::Error),
                    format!("Failed to fetch examples: {}", e),
                )
            })?;

        let data: Vec<entities::Example> = cursor.try_collect().await.map_err(|e| {
            domain::DomainError::new(
                domain::ErrorKind::Database(domain::DatabaseKind::Error),
                format!("Failed to collect examples: {}", e),
            )
        })?;

        Ok(Paginated::new(data, total, pagination))
    }

    #[tracing::instrument(skip_all)]
    async fn insert(&self, mut example: entities::Example) -> DomainWrapper<entities::Example> {
        let now = Utc::now();
        example.id = ObjectId::new();
        example.created_at = DateTime::from_chrono(now);
        example.updated_at = DateTime::from_chrono(now);

        let result = self.db.insert_one(&example).await;
        match result {
            Ok(_) => Ok(example),
            Err(e) => Err(domain::DomainError::new(
                domain::ErrorKind::Database(domain::DatabaseKind::Error),
                format!("Failed to insert example: {}", e),
            )),
        }
    }
}
