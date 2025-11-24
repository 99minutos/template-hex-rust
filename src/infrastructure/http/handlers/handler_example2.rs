use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::{
    infrastructure::http::{
        dto::example2_dto::Example2Dto, error::AppError, response::GenericApiResponse,
    },
    AppContext,
};

#[tracing::instrument(skip_all)]
pub async fn get_example2s(
    State(ctx): State<Arc<AppContext>>,
) -> Result<impl IntoResponse, AppError> {
    let list = ctx.example2_srv.get_example2s().await?;

    let result = list
        .into_iter()
        .map(Example2Dto::from)
        .collect::<Vec<Example2Dto>>();

    Ok(GenericApiResponse::from(Ok(result)))
}

#[tracing::instrument(skip_all)]
pub async fn add_random_example2(
    State(ctx): State<Arc<AppContext>>,
) -> Result<impl IntoResponse, AppError> {
    let result = ctx.example2_srv.add_random_example2().await?;

    Ok(GenericApiResponse::from(Ok(Example2Dto::from(result))))
}

#[tracing::instrument(skip_all)]
pub async fn get_example2s_with_error(
    State(ctx): State<Arc<AppContext>>,
) -> Result<impl IntoResponse, AppError> {
    let list = ctx.example2_srv.get_example2s_with_error().await?;

    let result = list
        .into_iter()
        .map(Example2Dto::from)
        .collect::<Vec<Example2Dto>>();

    Ok(GenericApiResponse::from(Ok(result)))
}
