use crate::domain::error::{Error, Result};
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
    pub async fn create_order(&self, dto: CreateOrderDto) -> Result<Order> {
        // 1. Validate User
        if self.users_repo.find_by_id(&dto.user_id).await?.is_none() {
            return Err(Error::not_found("User", &dto.user_id));
        }

        // 2. Validate Product & Get Price
        let product = self
            .products_repo
            .find_by_id(&dto.product_id)
            .await?
            .ok_or_else(|| Error::not_found("Product", &dto.product_id))?;

        // 3. Business Logic
        if dto.quantity <= 0 {
            return Err(Error::invalid_range("quantity", 1, i32::MAX));
        }

        let total_price = product.price * (dto.quantity as f64);
        let user_id = ObjectId::parse_str(&dto.user_id)
            .map_err(|_| Error::invalid_param("user_id", "User", &dto.user_id))?;

        let product_id = product
            .id
            .ok_or_else(|| Error::internal("Product missing ID"))?;

        // 4. Persistence
        let mut order = Order {
            id: None,
            user_id,
            product_id,
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
