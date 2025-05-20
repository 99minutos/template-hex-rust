use bson::{oid::ObjectId, DateTime};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

use crate::domain::entities::Example;

#[derive(Debug)]
pub struct ExampleRepository {
    db: Collection<Example>,
}

impl ExampleRepository {
    pub async fn new(client: &mongodb::Database) -> Self {
        let collection = client.collection::<Example>("examples");
        let a = Self { db: collection };
        a.create_index().await;
        a
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

    #[tracing::instrument]
    pub async fn all(&self) -> Result<Vec<Example>, String> {
        let filter = doc! {};

        match self.db.find(filter).await {
            Ok(cursor) => {
                let events: Vec<Example> = cursor.try_collect().await.map_err(|e| e.to_string())?;
                Ok(events)
            }
            Err(e) => Err(format!("Failed to get events: {}", e)),
        }
    }

    #[tracing::instrument]
    pub async fn insert(&self, example: &mut Example) -> Result<(), mongodb::error::Error> {
        let now = Utc::now();
        example.id = ObjectId::new();
        example.created_at = DateTime::from_chrono(now);
        example.updated_at = DateTime::from_chrono(now);

        let result = self.db.insert_one(example.clone()).await;
        match result {
            Ok(model) => {
                example.id = model.inserted_id.as_object_id().unwrap();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
