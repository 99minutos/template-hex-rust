use crate::domain::error::{DomainResult, Error};
use crate::domain::order::{Order, OrderId};
use crate::domain::pagination::Pagination;
use crate::domain::ports::order::OrderRepositoryPort;
use crate::domain::ports::product::ProductRepositoryPort;
use crate::domain::ports::user::UserRepositoryPort;
use crate::domain::product::ProductId;
use crate::domain::user::UserId;
use std::sync::Arc;

#[derive(Clone)]
pub struct OrderService {
    order_repo: Arc<dyn OrderRepositoryPort>,
    user_repo: Arc<dyn UserRepositoryPort>,
    product_repo: Arc<dyn ProductRepositoryPort>,
}

impl OrderService {
    pub fn new(
        order_repo: Arc<dyn OrderRepositoryPort>,
        user_repo: Arc<dyn UserRepositoryPort>,
        product_repo: Arc<dyn ProductRepositoryPort>,
    ) -> Self {
        Self {
            order_repo,
            user_repo,
            product_repo,
        }
    }

    #[tracing::instrument(skip_all, fields(%user_id, %product_id, %quantity))]
    pub async fn create_order(
        &self,
        user_id: &UserId,
        product_id: &ProductId,
        quantity: i32,
    ) -> DomainResult<Order> {
        // 1. Validate user exists
        let user_opt: Option<crate::domain::user::User> =
            self.user_repo.find_by_id(user_id).await?;
        if user_opt.is_none() {
            return Err(Error::not_found("User", user_id.to_string()));
        }

        // 2. Validate product exists and get price
        let product = self
            .product_repo
            .find_by_id(product_id)
            .await?
            .ok_or_else(|| Error::not_found("Product", product_id.to_string()))?;

        // 3. Business rule: check stock availability
        if product.stock < quantity {
            return Err(Error::business_rule(format!(
                "Insufficient stock: requested {}, available {}",
                quantity, product.stock
            )));
        }

        // 4. Calculate total price
        let total_price = product.price * (quantity as f64);

        // 5. Decrement stock atomically
        let pid = product
            .id
            .as_ref()
            .ok_or_else(|| Error::internal("Product missing ID"))?;

        let stock_updated = self.product_repo.update_stock(pid, -quantity).await?;

        if !stock_updated {
            return Err(Error::business_rule(
                "Failed to reserve stock — product may have been modified concurrently",
            ));
        }

        // 6. Persist order
        let now = chrono::Utc::now();
        let mut order = Order {
            id: None,
            user_id: user_id.clone(),
            product_id: product_id.clone(),
            quantity,
            total_price,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        let id = self.order_repo.create(&order).await?;
        order.id = Some(id);

        tracing::info!(
            order_id = %order.id.as_deref().unwrap_or("unknown"),
            %total_price,
            "Order created"
        );
        Ok(order)
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn get_order(&self, id: &OrderId) -> DomainResult<Order> {
        self.order_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("Order", id.to_string()))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_orders(&self, pagination: Pagination) -> DomainResult<Vec<Order>> {
        self.order_repo.find_all(pagination).await
    }

    #[tracing::instrument(skip_all, fields(%user_id))]
    pub async fn list_orders_by_user(
        &self,
        user_id: &UserId,
        pagination: Pagination,
    ) -> DomainResult<Vec<Order>> {
        // Validate user exists
        let user_opt: Option<crate::domain::user::User> =
            self.user_repo.find_by_id(user_id).await?;
        if user_opt.is_none() {
            return Err(Error::not_found("User", user_id.to_string()));
        }

        self.order_repo.find_by_user_id(user_id, pagination).await
    }
}
