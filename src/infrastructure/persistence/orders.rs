use crate::domain::error::{Error, Result};
use crate::domain::orders::Order;
use mongodb::{
    Collection, Database, IndexModel,
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
};

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

    /// Crea índices para la colección de órdenes
    pub async fn create_indexes(&self) -> Result<()> {
        let indexes = vec![
            // Índice en user_id (foreign key) - queries frecuentes de "órdenes del usuario"
            IndexModel::builder()
                .keys(doc! { "user_id": 1 })
                .options(
                    IndexOptions::builder()
                        .name("user_id_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice en product_id (foreign key) - queries de "órdenes del producto"
            IndexModel::builder()
                .keys(doc! { "product_id": 1 })
                .options(
                    IndexOptions::builder()
                        .name("product_id_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice en created_at para ordenar órdenes por fecha
            IndexModel::builder()
                .keys(doc! { "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("created_at_desc_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice compuesto: órdenes de un usuario por fecha (query MUY común)
            IndexModel::builder()
                .keys(doc! { "user_id": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("user_created_compound_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice compuesto: órdenes de un producto por fecha
            IndexModel::builder()
                .keys(doc! { "product_id": 1, "created_at": -1 })
                .options(
                    IndexOptions::builder()
                        .name("product_created_compound_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice en total_price para análisis y reportes
            IndexModel::builder()
                .keys(doc! { "total_price": -1 })
                .options(
                    IndexOptions::builder()
                        .name("total_price_desc_idx".to_string())
                        .build(),
                )
                .build(),
            // Índice compuesto para queries de órdenes en rango de fechas con totales
            IndexModel::builder()
                .keys(doc! { "created_at": -1, "total_price": -1 })
                .options(
                    IndexOptions::builder()
                        .name("created_price_compound_idx".to_string())
                        .build(),
                )
                .build(),
        ];

        self.collection.create_indexes(indexes).await?;
        tracing::info!("✓ Orders indexes created");
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn create(&self, order: &Order) -> Result<ObjectId> {
        let result = self.collection.insert_one(order).await?;
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| Error::internal("Failed to get inserted ID"))
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Order>> {
        let oid = ObjectId::parse_str(id).map_err(|_| Error::invalid_param("id", "Order", id))?;

        Ok(self.collection.find_one(doc! { "_id": oid }).await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn find_all(&self) -> Result<Vec<Order>> {
        use futures::stream::TryStreamExt;
        let cursor = self.collection.find(doc! {}).await?;
        Ok(cursor.try_collect().await?)
    }
}
