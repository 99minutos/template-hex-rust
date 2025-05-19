use futures::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

use crate::domain::entities::Example;

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

    pub async fn get_examples(&self) -> Result<Vec<Example>, String> {
        let filter = doc! {};

        match self.db.find(filter).await {
            Ok(cursor) => {
                let events: Vec<Example> = cursor.try_collect().await.map_err(|e| e.to_string())?;
                Ok(events)
            }
            Err(e) => Err(format!("Failed to get events: {}", e)),
        }
    }
}
