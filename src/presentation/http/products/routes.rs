use crate::presentation::{
    http::{
        error::ApiError,
        products::dtos::{CreateProductDto, ProductResponseDto, UpdateProductMetadataDto},
        response::{GenericApiResponse, MessageResponseDto},
        validation::ValidatedJson,
    },
    state::AppState,
};
use axum::{
    Router,
    extract::{Path, State},
    routing::{patch, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_product).get(list_products))
        .route("/{id}/metadata", patch(update_product_metadata))
}

#[utoipa::path(
    post,
    path = "/api/v1/products",
    tag = "Products",
    request_body = CreateProductDto,
    responses(
        (status = 200, description = "Product created", body = GenericApiResponse<ProductResponseDto>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn create_product(
    State(service): State<std::sync::Arc<crate::application::products::ProductsService>>,
    ValidatedJson(req): ValidatedJson<CreateProductDto>,
) -> Result<GenericApiResponse<ProductResponseDto>, ApiError> {
    let product = service.create_product(req).await?;
    Ok(GenericApiResponse::success(product.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/products",
    tag = "Products",
    responses(
        (status = 200, description = "List products", body = GenericApiResponse<Vec<ProductResponseDto>>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn list_products(
    State(service): State<std::sync::Arc<crate::application::products::ProductsService>>,
) -> Result<GenericApiResponse<Vec<ProductResponseDto>>, ApiError> {
    let products = service.list_products().await?;
    let dtos = products.into_iter().map(Into::into).collect();
    Ok(GenericApiResponse::success(dtos))
}

#[utoipa::path(
    patch,
    path = "/api/v1/products/{id}/metadata",
    tag = "Products",
    params(
        ("id" = String, Path, description = "Product ID")
    ),
    request_body = UpdateProductMetadataDto,
    responses(
        (status = 200, description = "Metadata updated", body = GenericApiResponse<MessageResponseDto>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn update_product_metadata(
    State(service): State<std::sync::Arc<crate::application::products::ProductsService>>,
    Path(id): Path<String>,
    ValidatedJson(req): ValidatedJson<UpdateProductMetadataDto>,
) -> Result<GenericApiResponse<MessageResponseDto>, ApiError> {
    service.update_metadata(&id, req.into()).await?;
    Ok(GenericApiResponse::success(MessageResponseDto {
        message: "Product metadata updated successfully".to_string(),
    }))
}
