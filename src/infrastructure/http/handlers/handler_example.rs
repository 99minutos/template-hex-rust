use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::{
    infrastructure::http::{dto::example_dto::ExampleDto, response::GenericApiResponse},
    AppContext,
};

#[tracing::instrument(skip_all)]
pub async fn get_examples(State(ctx): State<Arc<AppContext>>) -> impl IntoResponse {
    let result = ctx.example_srv.get_examples().await.map(|list| {
        list.into_iter()
            .map(ExampleDto::from)
            .collect::<Vec<ExampleDto>>()
    });

    GenericApiResponse::from(result)
}

#[tracing::instrument(skip_all)]
pub async fn add_random_example(State(ctx): State<Arc<AppContext>>) -> impl IntoResponse {
    let result = ctx
        .example_srv
        .add_random_example()
        .await
        .map(ExampleDto::from);

    GenericApiResponse::from(result)
}

#[tracing::instrument(skip_all)]
pub async fn get_examples_with_error(State(ctx): State<Arc<AppContext>>) -> impl IntoResponse {
    let result = ctx.example_srv.get_examples_with_error().await.map(|list| {
        list.into_iter()
            .map(ExampleDto::from)
            .collect::<Vec<ExampleDto>>()
    });

    GenericApiResponse::from(result)
}
