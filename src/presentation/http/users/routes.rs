use crate::{
    domain::error::Error,
    presentation::{
        http::{
            response::GenericApiResponse,
            users::dtos::{CreateUserDto, UserResponseDto},
            validation::ValidatedJson,
        },
        state::AppState,
    },
};
use axum::{
    Router,
    extract::{Path, State},
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/{id}", get(get_user).delete(delete_user))
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "User created", body = GenericApiResponse<UserResponseDto>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn create_user(
    State(service): State<std::sync::Arc<crate::application::users::UsersService>>,
    ValidatedJson(req): ValidatedJson<CreateUserDto>,
) -> Result<GenericApiResponse<UserResponseDto>, Error> {
    // Controller is now pure: Parse HTTP -> Call Service -> Return HTTP
    let user = service.create_user(req).await?;
    Ok(GenericApiResponse::success(user.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    responses(
        (status = 200, description = "Get user", body = GenericApiResponse<UserResponseDto>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn get_user(
    State(service): State<std::sync::Arc<crate::application::users::UsersService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<UserResponseDto>, Error> {
    let user = service.get_user(&id).await?;
    Ok(GenericApiResponse::success(user.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    responses(
        (status = 200, description = "List users", body = GenericApiResponse<Vec<UserResponseDto>>)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn list_users(
    State(service): State<std::sync::Arc<crate::application::users::UsersService>>,
) -> Result<GenericApiResponse<Vec<UserResponseDto>>, Error> {
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
    State(service): State<std::sync::Arc<crate::application::users::UsersService>>,
    Path(id): Path<String>,
) -> Result<GenericApiResponse<()>, Error> {
    service.delete_user(&id).await?;
    Ok(GenericApiResponse::success(()))
}
