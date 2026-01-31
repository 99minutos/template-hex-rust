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
            orders::dtos::{CreateOrderDto, OrderResponseDto},
        },
        state::AppState,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_order))
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
pub async fn create_order(
    State(service): State<std::sync::Arc<crate::application::orders::OrdersService>>,
    ValidatedJson(req): ValidatedJson<CreateOrderDto>,
) -> Result<GenericApiResponse<OrderResponseDto>, Error> {
    
    // Controller is super clean now
    let order = service.create_order(req).await?;

    Ok(GenericApiResponse::success(order.into()))
}
