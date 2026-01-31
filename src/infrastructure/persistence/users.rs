use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};
use crate::domain::users::User;

#[derive(Clone)]
pub struct UsersRepository {
    collection: Collection<User>,
}

impl UsersRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }

    pub async fn create(&self, user: &User) -> mongodb::error::Result<ObjectId> {
        let result = self.collection.insert_one(user).await?;
        Ok(result.inserted_id.as_object_id().unwrap_or_default())
    }

    pub async fn find_by_id(&self, id: &str) -> mongodb::error::Result<Option<User>> {
        let oid = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };
        self.collection.find_one(doc! { "_id": oid }).await
    }

    pub async fn find_by_email(&self, email: &str) -> mongodb::error::Result<Option<User>> {
        self.collection.find_one(doc! { "email": email }).await
    }

    pub async fn find_all(&self) -> mongodb::error::Result<Vec<User>> {
        use futures::stream::TryStreamExt;
        let cursor = self.collection.find(doc! {}).await?;
        cursor.try_collect().await
    }

    pub async fn delete(&self, id: &str) -> mongodb::error::Result<bool> {
        let oid = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(false),
        };
        let result = self.collection.delete_one(doc! { "_id": oid }).await?;
        Ok(result.deleted_count > 0)
    }
}
