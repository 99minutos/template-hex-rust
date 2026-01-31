use mongodb::{
    bson::oid::ObjectId,
    Collection, Database,
};
use crate::domain::orders::Order;

#[derive(Clone)]
pub struct OrdersRepository {
    collection: Collection<Order>,
}

impl OrdersRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("orders"),
        }
    }

    pub async fn create(&self, order: &Order) -> mongodb::error::Result<ObjectId> {
        let result = self.collection.insert_one(order).await?;
        Ok(result.inserted_id.as_object_id().unwrap_or_default())
    }
}
