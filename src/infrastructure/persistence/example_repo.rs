use std::sync::Arc;

use async_trait::async_trait;
use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

use crate::domain::{entities, ports};

#[derive(Debug)]
pub struct ExampleRepository {
    db: Collection<entities::Example>,
}

impl ExampleRepository {
    pub async fn new(client: &mongodb::Database) -> Arc<Box<dyn ports::PortExampleRepo>> {
        let collection = client.collection::<entities::Example>("examples");
        let a = Self { db: collection };
        a.create_index().await;
        Arc::new(Box::new(a))
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
    #[tracing::instrument]
    async fn all(&self) -> Result<Vec<entities::Example>, String> {
        let filter = doc! {};

        match self.db.find(filter).await {
            Ok(cursor) => {
                let events: Vec<entities::Example> =
                    cursor.try_collect().await.map_err(|e| e.to_string())?;
                Ok(events)
            }
            Err(e) => Err(format!("Failed to get events: {}", e)),
        }
    }

    #[tracing::instrument]
    async fn insert(
        &self,
        example: entities::Example,
    ) -> Result<entities::Example, mongodb::error::Error> {
        let mut example = example.clone(); // Clone the example to avoid ownership issues
        let now = Utc::now();
        example.id = ObjectId::new();
        example.created_at = DateTime::from_chrono(now);
        example.updated_at = DateTime::from_chrono(now);

        let result = self.db.insert_one(example.clone()).await;
        match result {
            Ok(model) => {
                example.id = model.inserted_id.as_object_id().unwrap();
                Ok(example)
            }
            Err(e) => Err(e),
        }
    }
}
