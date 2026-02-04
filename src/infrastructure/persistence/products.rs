use crate::domain::error::{Error, Result};
use crate::domain::products::{Product, ProductMetadata};
use mongodb::{
    Collection, Database, IndexModel,
    bson::{DateTime, doc, oid::ObjectId},
    options::IndexOptions,
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

    /// Crea índices para la colección de productos
    pub async fn create_indexes(&self) -> Result<()> {
        let indexes = vec![
            // SKU único (si se usa para identificar productos)
            IndexModel::builder()
                .keys(doc! { "metadata.sku": 1 })
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .sparse(true) // Permite null/undefined
                        .name("sku_unique_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice de texto para búsqueda por nombre y descripción
            IndexModel::builder()
                .keys(doc! { "name": "text", "metadata.description": "text" })
                .options(
                    IndexOptions::builder()
                        .name("name_description_text_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice en status para filtrar productos activos/draft
            IndexModel::builder()
                .keys(doc! { "status": 1 })
                .options(
                    IndexOptions::builder()
                        .name("status_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice compuesto para queries de productos por categoría y fecha
            IndexModel::builder()
                .keys(doc! { "metadata.category": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("category_created_compound_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice en precio para ordenar/filtrar por rango de precios
            IndexModel::builder()
                .keys(doc! { "price": 1 })
                .options(
                    IndexOptions::builder()
                        .name("price_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice en stock para encontrar productos sin inventario
            IndexModel::builder()
                .keys(doc! { "stock": 1 })
                .options(
                    IndexOptions::builder()
                        .name("stock_idx".to_string())
                        .build(),
                )
                .build(),
        ];

        self.collection.create_indexes(indexes).await?;
        tracing::info!("✓ Products indexes created");
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, product: &Product) -> Result<ObjectId> {
        let result = self.collection.insert_one(product).await?;
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Product>> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| Error::invalid_param("id", "Product", id))?;

        Ok(self.collection.find_one(doc! { "_id": oid }).await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_all(&self) -> Result<Vec<Product>> {
        use futures::stream::TryStreamExt;
        let cursor = self.collection.find(doc! {}).await?;
        Ok(cursor.try_collect().await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn update_metadata(
        &self,
        id: &str,
        metadata: &ProductMetadata,
    ) -> Result<bool> {
        let oid = ObjectId::parse_str(id)
            .map_err(|_| Error::invalid_param("id", "Product", id))?;

        let metadata_bson = bson::serialize_to_bson(metadata)
            .map_err(|e| Error::internal(format!("Serialization error: {}", e)))?;
        let now = DateTime::from_chrono(chrono::Utc::now());

        let result = self
            .collection
            .update_one(
                doc! { "_id": oid },
                doc! {
                    "$set": {
                        "metadata":  metadata_bson,
                        "updated_at": now,
                    }
                },
            )
            .await?;

        Ok(result.matched_count > 0)
    }
}
