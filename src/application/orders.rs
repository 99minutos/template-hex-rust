use crate::domain::error::Error;
use crate::{
    domain::orders::Order,
    infrastructure::persistence::{
        orders::OrdersRepository, products::ProductsRepository, users::UsersRepository,
    },
    presentation::http::orders::dtos::CreateOrderDto,
};
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

#[derive(Clone)]
pub struct OrdersService {
    orders_repo: Arc<OrdersRepository>,
    users_repo: Arc<UsersRepository>,
    products_repo: Arc<ProductsRepository>,
}

impl OrdersService {
    pub fn new(
        orders_repo: Arc<OrdersRepository>,
        users_repo: Arc<UsersRepository>,
        products_repo: Arc<ProductsRepository>,
    ) -> Self {
        Self {
            orders_repo,
            users_repo,
            products_repo,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn create_order(&self, dto: CreateOrderDto) -> Result<Order, Error> {
        // 1. Validate User
        // Note: We access repository directly.
        // In some architectures, you might inject UsersService here instead.
        if self.users_repo.find_by_id(&dto.user_id).await?.is_none() {
            return Err(Error::NotFound(format!("User {} not found", dto.user_id)));
        }

        // 2. Validate Product & Get Price
        let product = self
            .products_repo
            .find_by_id(&dto.product_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Product {} not found", dto.product_id)))?;

        // 3. Logic
        let total_price = product.price * (dto.quantity as f64);
        let user_id = ObjectId::parse_str(&dto.user_id)
            .map_err(|_| Error::InvalidId("Invalid User ID".to_string()))?;

        // 4. Persistence
        let mut order = Order {
            id: None,
            user_id,
            product_id: product.id.unwrap(),
            quantity: dto.quantity,
            total_price,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let id = self.orders_repo.create(&order).await?;
        order.id = Some(id);
        Ok(order)
    }
}
