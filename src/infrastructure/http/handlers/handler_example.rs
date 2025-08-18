use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    infrastructure::http::{dto::example_dto::ExampleDto, response::GenericApiResponse},
    AppContext,
};

#[tracing::instrument(skip_all)]
pub async fn get_examples(State(ctx): State<AppContext>) -> impl IntoResponse {
    let examples = ctx.example_srv.get_examples().await;
    let response = examples.map(|examples| {
        examples
            .into_iter()
            .map(ExampleDto::from)
            .collect::<Vec<_>>()
    });
    Json(GenericApiResponse::from(response))
}

#[tracing::instrument(skip_all)]
pub async fn add_random_example(State(ctx): State<AppContext>) -> impl IntoResponse {
    let example = ctx.example_srv.add_random_example().await;
    let response = example.map(ExampleDto::from);
    Json(GenericApiResponse::from(response))
}

#[tracing::instrument(skip_all)]
pub async fn get_examples_with_error(State(ctx): State<AppContext>) -> impl IntoResponse {
    let examples = ctx.example_srv.get_examples_with_error().await;
    let response = examples.map(|examples| {
        examples
            .into_iter()
            .map(ExampleDto::from)
            .collect::<Vec<_>>()
    });
    Json(GenericApiResponse::from(response))
}
