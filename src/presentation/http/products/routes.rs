use crate::{
    domain::error::Error,
    presentation::{
        http::{
            products::dtos::{CreateProductDto, ProductResponseDto},
            response::GenericApiResponse,
            validation::ValidatedJson,
        },
        state::AppState,
    },
};
use axum::{Router, extract::State, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_product).get(list_products))
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
) -> Result<GenericApiResponse<ProductResponseDto>, Error> {
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
) -> Result<GenericApiResponse<Vec<ProductResponseDto>>, Error> {
    let products = service.list_products().await?;
    let dtos = products.into_iter().map(Into::into).collect();
    Ok(GenericApiResponse::success(dtos))
}
