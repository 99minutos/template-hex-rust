use crate::domain::products::Product;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};

#[derive(Clone)]
pub struct ProductsRepository {
    collection: Collection<Product>,
}

impl ProductsRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("products"),
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, product: &Product) -> mongodb::error::Result<ObjectId> {
        let result = self.collection.insert_one(product).await?;
        Ok(result.inserted_id.as_object_id().unwrap_or_default())
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_by_id(&self, id: &str) -> mongodb::error::Result<Option<Product>> {
        let oid = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };
        self.collection.find_one(doc! { "_id": oid }).await
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_all(&self) -> mongodb::error::Result<Vec<Product>> {
        use futures::stream::TryStreamExt;
        let cursor = self.collection.find(doc! {}).await?;
        cursor.try_collect().await
    }
}
