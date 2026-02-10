use crate::domain::error::{Error, Result};
use crate::domain::orders::Order;
use crate::infrastructure::persistence::Pagination;
use crate::infrastructure::persistence::orders::OrdersRepository;
use crate::infrastructure::persistence::products::ProductsRepository;
use crate::infrastructure::persistence::users::UsersRepository;
use std::sync::Arc;

// ===== Application Commands =====

#[derive(Debug, Clone)]
pub struct CreateOrder {
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
}

// ===== Service =====

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

    #[tracing::instrument(skip_all, fields(user_id = %cmd.user_id, product_id = %cmd.product_id))]
    pub async fn create_order(&self, cmd: CreateOrder) -> Result<Order> {
        // 1. Validate user exists
        if self.users_repo.find_by_id(&cmd.user_id).await?.is_none() {
            return Err(Error::not_found("User", &cmd.user_id));
        }

        // 2. Validate product exists and get price
        let product = self
            .products_repo
            .find_by_id(&cmd.product_id)
            .await?
            .ok_or_else(|| Error::not_found("Product", &cmd.product_id))?;

        // 3. Business rule: check stock availability
        if product.stock < cmd.quantity {
            return Err(Error::business_rule(format!(
                "Insufficient stock: requested {}, available {}",
                cmd.quantity, product.stock
            )));
        }

        // 4. Calculate total price
        let total_price = product.price * (cmd.quantity as f64);

        // 5. Decrement stock atomically
        let product_id = product
            .id
            .as_deref()
            .ok_or_else(|| Error::internal("Product missing ID"))?;

        let stock_updated = self
            .products_repo
            .update_stock(product_id, -cmd.quantity)
            .await?;

        if !stock_updated {
            return Err(Error::business_rule(
                "Failed to reserve stock — product may have been modified concurrently",
            ));
        }

        // 6. Persist order
        let now = chrono::Utc::now();
        let mut order = Order {
            id: None,
            user_id: cmd.user_id,
            product_id: cmd.product_id,
            quantity: cmd.quantity,
            total_price,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        let id = self.orders_repo.create(&order).await?;
        order.id = Some(id);

        tracing::info!(
            order_id = %order.id.as_deref().unwrap_or("unknown"),
            %total_price,
            "Order created"
        );
        Ok(order)
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn get_order(&self, id: &str) -> Result<Order> {
        self.orders_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("Order", id))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_orders(&self, pagination: Pagination) -> Result<Vec<Order>> {
        self.orders_repo.find_all(pagination).await
    }

    #[tracing::instrument(skip_all, fields(%user_id))]
    pub async fn list_orders_by_user(
        &self,
        user_id: &str,
        pagination: Pagination,
    ) -> Result<Vec<Order>> {
        // Validate user exists
        if self.users_repo.find_by_id(user_id).await?.is_none() {
            return Err(Error::not_found("User", user_id));
        }

        self.orders_repo.find_by_user_id(user_id, pagination).await
    }
}
