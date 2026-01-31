use axum::{
    extract::State,
    routing::post,
    Router,
};
use crate::{
    domain::error::Error,
    presentation::{
        http::{
            response::GenericApiResponse,
            validation::ValidatedJson,
            products::dtos::{CreateProductDto, ProductResponseDto},
        },
        state::AppState,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_product).get(list_products))
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
pub async fn list_products(
    State(service): State<std::sync::Arc<crate::application::products::ProductsService>>,
) -> Result<GenericApiResponse<Vec<ProductResponseDto>>, Error> {
    let products = service.list_products().await?;
    let dtos = products.into_iter().map(Into::into).collect();
    Ok(GenericApiResponse::success(dtos))
}
