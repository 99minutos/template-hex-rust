use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};

use crate::{
    domain::Pagination,
    infrastructure::http::{
        dto::{
            example::{CreateExampleRequest, ExampleDto, ListExamplesQuery},
            ValidatedJson,
        },
        error::AppError,
        response::GenericApiResponse,
    },
    AppContext,
};

#[tracing::instrument(skip_all)]
pub async fn get_examples(
    State(ctx): State<Arc<AppContext>>,
) -> Result<impl IntoResponse, AppError> {
    let list = ctx.example_srv.get_examples().await?;

    let result = list
        .into_iter()
        .map(ExampleDto::from)
        .collect::<Vec<ExampleDto>>();

    Ok(GenericApiResponse::from(Ok(result)))
}

#[tracing::instrument(skip_all)]
pub async fn create_example(
    State(ctx): State<Arc<AppContext>>,
    ValidatedJson(payload): ValidatedJson<CreateExampleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = ctx.example_srv.create_example(payload.name).await?;

    Ok(GenericApiResponse::from(Ok(ExampleDto::from(result))))
}

#[tracing::instrument(skip_all)]
pub async fn add_random_example(
    State(ctx): State<Arc<AppContext>>,
) -> Result<impl IntoResponse, AppError> {
    let result = ctx.example_srv.add_random_example().await?;

    Ok(GenericApiResponse::from(Ok(ExampleDto::from(result))))
}

#[tracing::instrument(skip_all)]
pub async fn get_examples_with_error(
    State(ctx): State<Arc<AppContext>>,
) -> Result<impl IntoResponse, AppError> {
    let list = ctx.example_srv.get_examples_with_error().await?;

    let result = list
        .into_iter()
        .map(ExampleDto::from)
        .collect::<Vec<ExampleDto>>();

    Ok(GenericApiResponse::from(Ok(result)))
}

#[tracing::instrument(skip_all)]
pub async fn get_examples_paginated(
    State(ctx): State<Arc<AppContext>>,
    Query(query): Query<ListExamplesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let pagination = Pagination::from(query);
    let result = ctx.example_srv.get_examples_paginated(&pagination).await?;

    Ok(GenericApiResponse::from(Ok(result.map(ExampleDto::from))))
}
