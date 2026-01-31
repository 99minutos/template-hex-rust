use crate::domain::error::Error;
use crate::{
    domain::products::Product, infrastructure::persistence::products::ProductsRepository,
    presentation::http::products::dtos::CreateProductDto,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductsService {
    repo: Arc<ProductsRepository>,
}

impl ProductsService {
    pub fn new(repo: Arc<ProductsRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_product(&self, dto: CreateProductDto) -> Result<Product, Error> {
        let mut product = Product {
            id: None,
            name: dto.name,
            price: dto.price,
            stock: dto.stock,
        };
        let id = self.repo.create(&product).await?;
        product.id = Some(id);
        Ok(product)
    }

    pub async fn list_products(&self) -> Result<Vec<Product>, Error> {
        Ok(self.repo.find_all().await?)
    }

    // Internal helper for other services if needed,
    // though usually they go through repository or public service methods
    pub async fn get_product(&self, id: &str) -> Result<Product, Error> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Product {} not found", id)))
    }
}
