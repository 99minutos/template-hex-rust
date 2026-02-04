use crate::domain::error::{Error, Result};
use crate::domain::users::User;
use mongodb::{
    Collection, Database, IndexModel,
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
};

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

    /// Crea índices para la colección de usuarios
    pub async fn create_indexes(&self) -> Result<()> {
        let indexes = vec![
            // Email único (importante para login y validación)
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .name("email_unique_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice en created_at para ordenar usuarios por fecha
            IndexModel::builder()
                .keys(doc! { "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("created_at_desc_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice compuesto para queries comunes
            IndexModel::builder()
                .keys(doc! { "created_at": -1, "email": 1 })
                .options(
                    IndexOptions::builder()
                        .name("created_email_compound_idx".to_string())
                        .build(),
                )
                .build(),
        ];

        self.collection.create_indexes(indexes).await?;
        tracing::info!("✓ Users indexes created");
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, user: &User) -> Result<ObjectId> {
        let result = self.collection.insert_one(user).await?;
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| Error::invalid_param("id", "User", id))?;

        Ok(self.collection.find_one(doc! { "_id": oid }).await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        Ok(self.collection.find_one(doc! { "email": email }).await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_all(&self) -> Result<Vec<User>> {
        use futures::stream::TryStreamExt;
        let cursor = self.collection.find(doc! {}).await?;
        Ok(cursor.try_collect().await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn delete(&self, id: &str) -> Result<bool> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| Error::invalid_param("id", "User", id))?;

        let result = self.collection.delete_one(doc! { "_id": oid }).await?;
        Ok(result.deleted_count > 0)
    }
}
