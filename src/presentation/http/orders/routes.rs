use crate::application::orders::OrdersService;
use crate::domain::orders::OrderId;
use crate::domain::products::ProductId;
use crate::domain::users::UserId;
use crate::infrastructure::persistence::Pagination;
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
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct OrderQuery {
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,
}

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
    let user_id = UserId::new(req.user_id);
    let product_id = ProductId::new(req.product_id);
    let order = service
        .create_order(&user_id, &product_id, req.quantity)
        .await?;
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
    let order_id = OrderId::new(id);
    let order = service.get_order(&order_id).await?;
    Ok(GenericApiResponse::success(order.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/orders",
    tag = "Orders",
    params(OrderQuery),
    responses(
        (status = 200, description = "List orders", body = GenericApiResponse<Vec<OrderOutput>>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn list_orders(
    State(service): State<Arc<OrdersService>>,
    Query(query): Query<OrderQuery>,
) -> Result<GenericApiResponse<Vec<OrderOutput>>, ApiError> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(20),
    };

    let orders = service.list_orders(pagination).await?;
    let dtos = orders.into_iter().map(Into::into).collect();
    Ok(GenericApiResponse::success(dtos))
}
