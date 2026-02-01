use crate::presentation::{
    http::{
        error::ApiError,
        orders::dtos::{CreateOrderDto, OrderResponseDto},
        response::GenericApiResponse,
        validation::ValidatedJson,
    },
    state::AppState,
};
use axum::{Router, extract::State, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_order))
}

#[utoipa::path(
    post,
    path = "/api/v1/orders",
    tag = "Orders",
    request_body = CreateOrderDto,
    responses(
        (status = 200, description = "Order created", body = GenericApiResponse<OrderResponseDto>),
        (status = 404, description = "User or Product not found"),
        (status = 400, description = "Invalid IDs")
    )
)]
#[tracing::instrument(skip_all)]
pub async fn create_order(
    State(service): State<std::sync::Arc<crate::application::orders::OrdersService>>,
    ValidatedJson(req): ValidatedJson<CreateOrderDto>,
) -> Result<GenericApiResponse<OrderResponseDto>, ApiError> {
    // Controller is super clean now
    let order = service.create_order(req).await?;

    Ok(GenericApiResponse::success(order.into()))
}
