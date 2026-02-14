use crate::domain::error::{Error, DomainResult};
use crate::domain::orders::{Order, OrderId};
use crate::domain::users::UserId;
use crate::infrastructure::persistence::Pagination;
use crate::infrastructure::persistence::orders::model::OrderDocument;
use futures::stream::TryStreamExt;
use mongodb::{
    Collection, Database, IndexModel,
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
};

#[derive(Clone)]
pub struct OrdersRepository {
    collection: Collection<OrderDocument>,
}

impl OrdersRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("orders"),
        }
    }

    /// Create database indexes (idempotent — safe to call on every startup)
    pub async fn create_indexes(&self) -> DomainResult<()> {
        let indexes = vec![
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "user_id": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_user_created_compound_idx".to_string())
                        .build(),
                )
                .build(),
            IndexModel::builder()
                .keys(doc! { "deleted_at": 1, "product_id": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("deleted_product_created_compound_idx".to_string())
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
                .keys(doc! { "deleted_at": 1, "total_price": -1 })
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

        tracing::info!("✓ Orders indexes created");
        Ok(())
    }

    // ===== CREATE =====

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, order: &Order) -> DomainResult<OrderId> {
        let doc = OrderDocument::try_from(order.clone())?;

        let result = self
            .collection
            .insert_one(doc)
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        result
            .inserted_id
            .as_object_id()
            .map(|oid| OrderId::new(oid.to_hex()))
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    // ===== READ =====

    #[tracing::instrument(skip_all)]
    pub async fn find_by_id(&self, id: &OrderId) -> DomainResult<Option<Order>> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "Order", &**id))?;

        let doc = self
            .collection
            .find_one(doc! { "_id": oid, "deleted_at": { "$exists": false } })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(doc.map(Order::from))
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_all(&self, pagination: Pagination) -> DomainResult<Vec<Order>> {
        let cursor = self
            .collection
            .find(doc! { "deleted_at": { "$exists": false } })
            .skip(pagination.skip())
            .limit(pagination.limit_i64())
            .sort(doc! { "created_at": -1 })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        let docs: Vec<OrderDocument> = cursor
            .try_collect()
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(docs.into_iter().map(Order::from).collect())
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_by_user_id(
        &self,
        user_id: &UserId,
        pagination: Pagination,
    ) -> DomainResult<Vec<Order>> {
        let oid = ObjectId::parse_str(&**user_id)
            .map_err(|_| Error::invalid_param("user_id", "User", &**user_id))?;

        let cursor = self
            .collection
            .find(doc! {
                "user_id": oid,
                "deleted_at": { "$exists": false }
            })
            .skip(pagination.skip())
            .limit(pagination.limit_i64())
            .sort(doc! { "created_at": -1 })
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        let docs: Vec<OrderDocument> = cursor
            .try_collect()
            .await
            .map_err(|e| Error::database(e.to_string()))?;

        Ok(docs.into_iter().map(Order::from).collect())
    }

    // ===== SOFT DELETE =====

    #[tracing::instrument(skip_all)]
    pub async fn delete(&self, id: &OrderId) -> DomainResult<bool> {
        let oid =
            ObjectId::parse_str(&**id).map_err(|_| Error::invalid_param("id", "Order", &**id))?;

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
    pub async fn count(&self) -> DomainResult<u64> {
        self.collection
            .count_documents(doc! { "deleted_at": { "$exists": false } })
            .await
            .map_err(|e| Error::database(e.to_string()))
    }
}
