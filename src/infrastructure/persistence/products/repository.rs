use crate::domain::error::{Error, Result};
use crate::domain::products::{Product, ProductMetadata};
use crate::infrastructure::persistence::Pagination;
use crate::infrastructure::persistence::products::model::ProductDocument;
use futures::stream::TryStreamExt;
use mongodb::{
    Collection, Database, IndexModel,
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
};

#[derive(Clone)]
pub struct ProductsRepository {
    collection: Collection<ProductDocument>,
}

impl ProductsRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("products"),
        }
    }

    /// Create database indexes (idempotent — safe to call on every startup)
    pub async fn create_indexes(&self) -> Result<()> {
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "metadata.sku": 1 })
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .sparse(true)
                        .name("sku_unique_idx".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "name": "text", "metadata.description": "text" })
                .options(
                    IndexOptions::builder()
                        .name("name_description_text_idx".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "status": 1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_status_compound_idx".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "metadata.category": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_category_created_compound_idx".to_string())
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
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "stock": 1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_stock_compound_idx".to_string())
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

    // ===== CREATE =====

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, product: &Product) -> Result<String> {
        let doc = ProductDocument::from(product.clone());
        let result = self
            .collection
            .insert_one(doc)
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        result
            .inserted_id
            .as_object_id()
            .map(|oid| oid.to_hex())
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    // ===== READ =====

    #[tracing::instrument(skip_all)]
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Product>> {
        let oid = ObjectId::parse_str(id).map_err(|_| Error::invalid_param("id", "Product", id))?;

        let doc = self
            .collection
            .find_one(doc! { "_id": oid, "deleted_at": { "$exists": false } })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(doc.map(Product::from))
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_all(&self, pagination: Pagination) -> Result<Vec<Product>> {
        let cursor = self
            .collection
            .find(doc! { "deleted_at": { "$exists": false } })
            .skip(pagination.skip())
            .limit(pagination.limit())
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
    pub async fn update_metadata(&self, id: &str, metadata: &ProductMetadata) -> Result<bool> {
        let oid = ObjectId::parse_str(id).map_err(|_| Error::invalid_param("id", "Product", id))?;

        let metadata_bson = bson::serialize_to_bson(metadata)
            .map_err(|e| Error::internal(format!("Serialization error: {}", e)))?;
        let now = bson::DateTime::from_chrono(chrono::Utc::now());

        let result = self
            .collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! {
                    "$set": {
                        "metadata": metadata_bson,
                        "updated_at": now,
                    }
                },
            )
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(result.matched_count > 0)
    }

    #[tracing::instrument(skip_all)]
    pub async fn update_stock(&self, id: &str, quantity_delta: i32) -> Result<bool> {
        let oid = ObjectId::parse_str(id).map_err(|_| Error::invalid_param("id", "Product", id))?;

        let now = bson::DateTime::from_chrono(chrono::Utc::now());

        let result = self
            .collection
            .update_one(
                doc! { "_id": oid, "deleted_at": { "$exists": false } },
                doc! {
                    "$inc": { "stock": quantity_delta },
                    "$set": { "updated_at": now },
                },
            )
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(result.matched_count > 0)
    }

    // ===== SOFT DELETE =====

    #[tracing::instrument(skip_all)]
    pub async fn delete(&self, id: &str) -> Result<bool> {
        let oid = ObjectId::parse_str(id).map_err(|_| Error::invalid_param("id", "Product", id))?;

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
