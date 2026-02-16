use crate::application::product::ProductService;
use crate::domain::pagination::Pagination;
use crate::domain::product::{ProductId, ProductMetadata};
use crate::presentation::{
    http::{
        error::ApiError,
        product::dtos::{CreateProductInput, ProductOutput, UpdateProductMetadataInput},
        response::GenericApiResponse,
        validation::ValidatedJson,
    },
    state::AppState,
};
use axum::{
    Router,
    extract::{Path, Query, State},
    routing::{get, patch, post},
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct ProductQuery {
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_product).get(list_products))
        .route("/{id}", get(get_product).delete(delete_product))
        .route("/{id}/metadata", patch(update_metadata))
}

#[utoipa::path(
    post,
    path = "/api/v1/products",
    tag = "Products",
    request_body = CreateProductInput,
    responses(
        (status = 200, description = "Product created", body = GenericApiResponse<ProductOutput>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn create_product(
    State(service): State<Arc<ProductService>>,
    ValidatedJson(req): ValidatedJson<CreateProductInput>,
) -> Result<GenericApiResponse<ProductOutput>, ApiError> {
    let metadata = ProductMetadata {
        description: req.description,
        category: req.category,
        tags: req.tags.unwrap_or_default(),
        sku: req.sku,
    };

    let product = service
        .create_product(&req.name, req.price, req.stock, metadata)
        .await?;
    Ok(GenericApiResponse::success(product.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/products/{id}",
    tag = "Products",
    responses(
        (status = 200, description = "Get product", body = GenericApiResponse<ProductOutput>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn get_product(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<ProductOutput>, ApiError> {
    let product_id = ProductId::new(id);
    let product = service.get_product(&product_id).await?;
    Ok(GenericApiResponse::success(product.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/products",
    tag = "Products",
    params(ProductQuery),
    responses(
        (status = 200, description = "List products", body = GenericApiResponse<Vec<ProductOutput>>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn list_products(
    State(service): State<Arc<ProductService>>,
    Query(query): Query<ProductQuery>,
) -> Result<GenericApiResponse<Vec<ProductOutput>>, ApiError> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(20),
    };

    let products = service.list_products(pagination).await?;
    let dtos = products.into_iter().map(Into::into).collect();
    Ok(GenericApiResponse::success(dtos))
}

#[utoipa::path(
    patch,
    path = "/api/v1/products/{id}/metadata",
    tag = "Products",
    request_body = UpdateProductMetadataInput,
    responses(
        (status = 200, description = "Metadata updated", body = GenericApiResponse<ProductOutput>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn update_metadata(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<String>,
    ValidatedJson(req): ValidatedJson<UpdateProductMetadataInput>,
) -> Result<GenericApiResponse<ProductOutput>, ApiError> {
    let product_id = ProductId::new(id);
    let metadata = ProductMetadata {
        description: req.description,
        category: req.category,
        tags: req.tags,
        sku: req.sku,
    };

    let product = service.update_metadata(&product_id, metadata).await?;
    Ok(GenericApiResponse::success(product.into()))
}

#[utoipa::path(
    delete,
    path = "/api/v1/products/{id}",
    tag = "Products",
    responses(
        (status = 200, description = "Delete product")
    )
)]
#[tracing::instrument(skip_all)]
pub async fn delete_product(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<()>, ApiError> {
    let product_id = ProductId::new(id);
    service.delete_product(&product_id).await?;
    Ok(GenericApiResponse::success(()))
}
