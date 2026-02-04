use crate::domain::error::{Error, Result};
use crate::domain::products::{Product, ProductMetadata, ProductStatus};
use crate::{
    infrastructure::persistence::products::ProductsRepository,
    presentation::http::products::dtos::CreateProductInput,
};
use chrono::Utc;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductsService {
    repo: Arc<ProductsRepository>,
}

impl ProductsService {
    pub fn new(repo: Arc<ProductsRepository>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip_all)]
    pub async fn create_product(&self, dto: CreateProductInput) -> Result<Product> {
        let now = Utc::now();
        let mut product = Product {
            id: None,
            name: dto.name,
            price: dto.price,
            stock: dto.stock,
            status: ProductStatus::Draft,
            metadata: ProductMetadata {
                description: dto.description,
                category: dto.category,
                tags: dto.tags.unwrap_or_default(),
                sku: dto.sku,
            },
            created_at: now,
            updated_at: now,
        };
        let id = self.repo.create(&product).await?;
        product.id = Some(id);
        Ok(product)
    }

    #[tracing::instrument(skip_all)]
    pub async fn get_product(&self, id: &str) -> Result<Product> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("Product", id))
    }

    #[tracing::instrument(skip_all)]
    pub async fn list_products(&self) -> Result<Vec<Product>> {
        Ok(self.repo.find_all().await?)
    }

    #[tracing::instrument(skip_all)]
    pub async fn update_metadata(&self, id: &str, metadata: ProductMetadata) -> Result<Product> {
        let updated = self.repo.update_metadata(id, &metadata).await?;
        if !updated {
            return Err(Error::not_found("Product", id));
        }
        self.get_product(id).await
    }

    #[tracing::instrument(skip_all)]
    pub async fn delete_product(&self, id: &str) -> Result<()> {
        let deleted = self.repo.delete(id).await?;
        if !deleted {
            return Err(Error::not_found("Product", id));
        }
        Ok(())
    }
}
