use crate::domain::error::{Error, Result};
use crate::domain::users::{User, UserId};
use crate::infrastructure::persistence::Pagination;
use crate::infrastructure::persistence::users::model::UserDocument;
use futures::stream::TryStreamExt;
use mongodb::{
    Collection, Database, IndexModel,
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
};

#[derive(Clone)]
pub struct UsersRepository {
    collection: Collection<UserDocument>,
}

impl UsersRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }

    /// Create database indexes (idempotent — safe to call on every startup)
    pub async fn create_indexes(&self) -> Result<()> {
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .name("email_unique_idx".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_created_compound_idx".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "email": 1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_email_compound_idx".to_string())
                        .build(),
                )
                .build(),
        ];

        self.collection
            .create_indexes(indexes)
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        tracing::info!("✓ Users indexes created");
        Ok(())
    }

    // ===== CREATE =====

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, user: &User) -> Result<UserId> {
        let doc = UserDocument::from(user.clone());
        let result = self
            .collection
            .insert_one(doc)
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        result
            .inserted_id
            .as_object_id()
            .map(|oid| UserId::new(oid.to_hex()))
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    // ===== READ =====

    #[tracing::instrument(skip_all)]
    pub async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "User", &**id))?;

        let doc = self
            .collection
            .find_one(doc! { "_id": oid, "deleted_at": { "$exists": false } })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(doc.map(User::from))
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let doc = self
            .collection
            .find_one(doc! {
                "email": email,
                "deleted_at": { "$exists": false }
            })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(doc.map(User::from))
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_all(&self, pagination: Pagination) -> Result<Vec<User>> {
        let cursor = self
            .collection
            .find(doc! { "deleted_at": { "$exists": false } })
            .skip(pagination.skip())
            .limit(pagination.limit())
            .sort(doc! { "created_at": -1 })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        let docs: Vec<UserDocument> = cursor
            .try_collect()
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(docs.into_iter().map(User::from).collect())
    }

    // ===== UPDATE =====

    #[tracing::instrument(skip_all)]
    pub async fn update(&self, id: &UserId, user: &User) -> Result<bool> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "User", &**id))?;

        let doc = UserDocument::from(user.clone());
        let bson_doc =
            bson::serialize_to_document(&doc).map_err(|e| Error::internal(e.to_string()))?;

        let result = self
            .collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! { "$set": bson_doc },
            )
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(result.matched_count > 0)
    }

    // ===== SOFT DELETE =====

    #[tracing::instrument(skip_all)]
    pub async fn delete(&self, id: &UserId) -> Result<bool> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "User", &**id))?;

        let now = bson::DateTime::from_chrono(chrono::Utc::now());

        let result = self
            .collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! { "$set": { "deleted_at": now } },
            )
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(result.matched_count > 0)
    }

    // ===== COUNT =====

    #[tracing::instrument(skip_all)]
    pub async fn count(&self) -> Result<u64> {
        self.collection
            .count_documents(doc! { "deleted_at": { "$exists": false } })
            .await
            .map_err(|e| Error::database(e.to_string()))
    }
}
