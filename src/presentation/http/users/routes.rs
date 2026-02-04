use crate::application::users::UsersService;
use crate::presentation::{
    http::{
        error::ApiError,
        response::GenericApiResponse,
        users::dtos::{CreateUserInput, UserOutput},
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
    let user = service.create_user(req).await?;
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
    let user = service.get_user(&id).await?;
    Ok(GenericApiResponse::success(user.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    responses(
        (status = 200, description = "List users", body = GenericApiResponse<Vec<UserOutput>>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn list_users(
    State(service): State<Arc<UsersService>>,
) -> Result<GenericApiResponse<Vec<UserOutput>>, ApiError> {
    let users = service.list_users().await?;
    let dtos = users.into_iter().map(Into::into).collect();
    Ok(GenericApiResponse::success(dtos))
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
    service.delete_user(&id).await?;
    Ok(GenericApiResponse::success(()))
}
