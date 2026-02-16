use crate::domain::error::{DomainResult, Error};
use crate::domain::pagination::Pagination;
use crate::domain::ports::product::ProductRepositoryPort;
use crate::domain::product::{Product, ProductId, ProductMetadata};
use crate::infrastructure::persistence::product::model::ProductDocument;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    Collection, Database, IndexModel,
    bson::{self, doc, oid::ObjectId},
    options::IndexOptions,
};

#[derive(Clone)]
pub struct ProductRepository {
    collection: Collection<ProductDocument>,
}

impl ProductRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("products"),
        }
    }

    /// Create database indexes (idempotent — safe to call on every startup)
    pub async fn create_indexes(&self) -> DomainResult<()> {
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_created_compound_idx".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "price": 1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_price_compound_idx".to_string())
                        .build(),
                )
                .build(),
        ];

        self.collection
            .create_indexes(indexes)
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        tracing::info!("✓ Products indexes created");
        Ok(())
    }
}

#[async_trait]
impl ProductRepositoryPort for ProductRepository {
    // ===== CREATE =====

    #[tracing::instrument(skip_all)]
    async fn create(&self, product: &Product) -> DomainResult<ProductId> {
        let doc = ProductDocument::from(product.clone());
        let result = self
            .collection
            .insert_one(doc)
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        result
            .inserted_id
            .as_object_id()
            .map(|oid| ProductId::new(oid.to_hex()))
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    // ===== READ =====

    #[tracing::instrument(skip_all)]
    async fn find_by_id(&self, id: &ProductId) -> DomainResult<Option<Product>> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "Product", &**id))?;

        let doc = self
            .collection
            .find_one(doc! { "_id": oid, "deleted_at": { "$exists": false } })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(doc.map(Product::from))
    }

    #[tracing::instrument(skip_all)]
    async fn find_all(&self, pagination: Pagination) -> DomainResult<Vec<Product>> {
        let cursor = self
            .collection
            .find(doc! { "deleted_at": { "$exists": false } })
            .skip(pagination.get_skip())
            .limit(pagination.get_limit())
            .sort(doc! { "created_at": -1 })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        let docs: Vec<ProductDocument> = cursor
            .try_collect()
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(docs.into_iter().map(Product::from).collect())
    }

    // ===== UPDATE =====

    #[tracing::instrument(skip_all)]
    async fn update_metadata(
        &self,
        id: &ProductId,
        metadata: &ProductMetadata,
    ) -> DomainResult<bool> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "Product", &**id))?;

        let bson_metadata = bson::serialize_to_bson(metadata)
            .map_err(|e| Error::internal(format!("Serialization error: {}", e)))?;

        let now = bson::DateTime::from_chrono(chrono::Utc::now());

        let result = self
            .collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! {
                    "$set": {
                        "metadata": bson_metadata,
                        "updated_at": now
                    }
                },
            )
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(result.matched_count > 0)
    }

    #[tracing::instrument(skip_all)]
    async fn update_stock(&self, id: &ProductId, delta: i32) -> DomainResult<bool> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "Product", &**id))?;

        let now = bson::DateTime::from_chrono(chrono::Utc::now());

        // Use $inc for atomic update
        let result = self
            .collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! {
                    "$inc": { "stock": delta },
                    "$set": { "updated_at": now },
                },
            )
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(result.matched_count > 0)
    }

    // ===== SOFT DELETE =====

    #[tracing::instrument(skip_all)]
    async fn delete(&self, id: &ProductId) -> DomainResult<bool> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "Product", &**id))?;

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
    async fn count(&self) -> DomainResult<u64> {
        self.collection
            .count_documents(doc! { "deleted_at": { "$exists": false } })
            .await
            .map_err(|e| Error::database(e.to_string()))
    }
}
