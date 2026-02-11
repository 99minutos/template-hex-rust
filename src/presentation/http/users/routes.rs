use crate::application::users::UsersService;
use crate::domain::users::UserId;
use crate::infrastructure::persistence::Pagination;
use crate::presentation::{
    http::{
        error::ApiError,
        response::{GenericApiResponse, GenericPagination},
        users::dtos::{CreateUserInput, UserOutput},
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
pub struct UserQuery {
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/{id}", get(get_user).delete(delete_user))
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    request_body = CreateUserInput,
    responses(
        (status = 200, description = "User created", body = GenericApiResponse<UserOutput>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn create_user(
    State(service): State<Arc<UsersService>>,
    ValidatedJson(req): ValidatedJson<CreateUserInput>,
) -> Result<GenericApiResponse<UserOutput>, ApiError> {
    let user = service.create_user(&req.name, &req.email).await?;
    Ok(GenericApiResponse::success(user.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    responses(
        (status = 200, description = "Get user", body = GenericApiResponse<UserOutput>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn get_user(
    State(service): State<Arc<UsersService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<UserOutput>, ApiError> {
    let user_id = UserId::new(id);
    let user = service.get_user(&user_id).await?;
    Ok(GenericApiResponse::success(user.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    params(UserQuery),
    responses(
        (status = 200, description = "List users (paginated)", body = GenericApiResponse<GenericPagination<UserOutput>>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn list_users(
    State(service): State<Arc<UsersService>>,
    Query(query): Query<UserQuery>,
) -> Result<GenericApiResponse<GenericPagination<UserOutput>>, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let pagination = Pagination { page, limit };

    let users = service.list_users(pagination).await?;
    let total = service.count_users().await?;
    let data: Vec<UserOutput> = users.into_iter().map(Into::into).collect();

    Ok(GenericApiResponse::paginated(data, total, page, limit))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "Users",
    responses(
        (status = 200, description = "Delete user")
    )
)]
#[tracing::instrument(skip_all)]
pub async fn delete_user(
    State(service): State<Arc<UsersService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<()>, ApiError> {
    let user_id = UserId::new(id);
    service.delete_user(&user_id).await?;
    Ok(GenericApiResponse::success(()))
}
