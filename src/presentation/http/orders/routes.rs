use crate::application::orders::OrdersService;
use crate::presentation::{
    http::{
        error::ApiError,
        orders::dtos::{CreateOrderInput, OrderOutput},
        response::GenericApiResponse,
        validation::ValidatedJson,
    },
    state::AppState,
};
use axum::{
    Router,
    extract::{Path, State},
    routing::{get, post},
};
use std::sync::Arc;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_order).get(list_orders))
        .route("/{id}", get(get_order))
}

#[utoipa::path(
    post,
    path = "/api/v1/orders",
    tag = "Orders",
    request_body = CreateOrderInput,
    responses(
        (status = 200, description = "Order created", body = GenericApiResponse<OrderOutput>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn create_order(
    State(service): State<Arc<OrdersService>>,
    ValidatedJson(req): ValidatedJson<CreateOrderInput>,
) -> Result<GenericApiResponse<OrderOutput>, ApiError> {
    let order = service.create_order(req).await?;
    Ok(GenericApiResponse::success(order.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/orders/{id}",
    tag = "Orders",
    responses(
        (status = 200, description = "Get order", body = GenericApiResponse<OrderOutput>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn get_order(
    State(service): State<Arc<OrdersService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<OrderOutput>, ApiError> {
    let order = service.get_order(&id).await?;
    Ok(GenericApiResponse::success(order.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/orders",
    tag = "Orders",
    responses(
        (status = 200, description = "List orders", body = GenericApiResponse<Vec<OrderOutput>>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn list_orders(
    State(service): State<Arc<OrdersService>>,
) -> Result<GenericApiResponse<Vec<OrderOutput>>, ApiError> {
    let orders = service.list_orders().await?;
    let dtos = orders.into_iter().map(Into::into).collect();
    Ok(GenericApiResponse::success(dtos))
}
