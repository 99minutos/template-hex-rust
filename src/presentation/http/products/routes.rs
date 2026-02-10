use crate::application::products::ProductsService;
use crate::infrastructure::persistence::Pagination;
use crate::presentation::{
    http::{
        error::ApiError,
        products::dtos::{CreateProductInput, ProductOutput, UpdateProductMetadataInput},
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
    pub page_size: Option<u32>,
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
    State(service): State<Arc<ProductsService>>,
    ValidatedJson(req): ValidatedJson<CreateProductInput>,
) -> Result<GenericApiResponse<ProductOutput>, ApiError> {
    let product = service.create_product(req.into()).await?;
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
    State(service): State<Arc<ProductsService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<ProductOutput>, ApiError> {
    let product = service.get_product(&id).await?;
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
    State(service): State<Arc<ProductsService>>,
    Query(query): Query<ProductQuery>,
) -> Result<GenericApiResponse<Vec<ProductOutput>>, ApiError> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(20),
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
    State(service): State<Arc<ProductsService>>,
    Path(id): Path<String>,
    ValidatedJson(req): ValidatedJson<UpdateProductMetadataInput>,
) -> Result<GenericApiResponse<ProductOutput>, ApiError> {
    let product = service.update_metadata(&id, req.into()).await?;
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
    State(service): State<Arc<ProductsService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<()>, ApiError> {
    service.delete_product(&id).await?;
    Ok(GenericApiResponse::success(()))
}
