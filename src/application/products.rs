use crate::domain::error::{Error, Result};
use crate::domain::products::{Product, ProductMetadata, ProductStatus};
use crate::infrastructure::persistence::Pagination;
use crate::infrastructure::persistence::products::ProductsRepository;
use std::sync::Arc;

// ===== Application Commands =====

#[derive(Debug, Clone)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub category: String,
    pub sku: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct UpdateProductMetadata {
    pub description: Option<String>,
    pub category: String,
    pub tags: Vec<String>,
    pub sku: String,
}

// ===== Service =====

#[derive(Clone)]
pub struct ProductsService {
    repo: Arc<ProductsRepository>,
}

impl ProductsService {
    pub fn new(repo: Arc<ProductsRepository>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip_all, fields(name = %cmd.name, sku = %cmd.sku))]
    pub async fn create_product(&self, cmd: CreateProduct) -> Result<Product> {
        let now = chrono::Utc::now();
        let mut product = Product {
            id: None,
            name: cmd.name,
            price: cmd.price,
            stock: cmd.stock,
            status: ProductStatus::Draft,
            metadata: ProductMetadata {
                description: cmd.description,
                category: cmd.category,
                tags: cmd.tags.unwrap_or_default(),
                sku: cmd.sku,
            },
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        let id = self.repo.create(&product).await?;
        product.id = Some(id);

        tracing::info!(product_id = %product.id.as_deref().unwrap_or("unknown"), "Product created");
        Ok(product)
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn get_product(&self, id: &str) -> Result<Product> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("Product", id))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_products(&self, pagination: Pagination) -> Result<Vec<Product>> {
        self.repo.find_all(pagination).await
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn update_metadata(&self, id: &str, cmd: UpdateProductMetadata) -> Result<Product> {
        let metadata = ProductMetadata {
            description: cmd.description,
            category: cmd.category,
            tags: cmd.tags,
            sku: cmd.sku,
        };

        let updated = self.repo.update_metadata(id, &metadata).await?;
        if !updated {
            return Err(Error::not_found("Product", id));
        }

        tracing::info!("Product metadata updated");
        self.get_product(id).await
    }

    #[tracing::instrument(skip_all, fields(%id))]
    pub async fn delete_product(&self, id: &str) -> Result<()> {
        let deleted = self.repo.delete(id).await?;
        if !deleted {
            return Err(Error::not_found("Product", id));
        }
        tracing::info!("Product soft-deleted");
        Ok(())
    }

    /// Atomically decrement stock. Returns error if product not found.
    #[tracing::instrument(skip_all, fields(%id, %quantity))]
    pub async fn decrement_stock(&self, id: &str, quantity: i32) -> Result<()> {
        let product = self.get_product(id).await?;

        if product.stock < quantity {
            return Err(Error::business_rule(format!(
                "Insufficient stock for product {}: requested {}, available {}",
                id, quantity, product.stock
            )));
        }

        let updated = self.repo.update_stock(id, -quantity).await?;
        if !updated {
            return Err(Error::not_found("Product", id));
        }

        tracing::info!(remaining = product.stock - quantity, "Stock decremented");
        Ok(())
    }
}
