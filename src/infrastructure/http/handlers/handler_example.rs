use axum::{extract::State, response, Json};

use crate::{
    infrastructure::http::{dto::ExampleDto, HttpError},
    AppContext,
};

#[tracing::instrument]
pub async fn get_examples(
    State(ctx): State<AppContext>,
) -> Result<impl response::IntoResponse, HttpError> {
    let examples = ctx
        .example_srv
        .get_examples()
        .await
        .map_err(HttpError::from)?;
    Ok(Json(
        examples
            .into_iter()
            .map(ExampleDto::from)
            .collect::<Vec<_>>(),
    ))
}

#[tracing::instrument]
pub async fn add_random_example(
    State(ctx): State<AppContext>,
) -> Result<impl response::IntoResponse, HttpError> {
    let example = ctx
        .example_srv
        .add_random_example()
        .await
        .map_err(HttpError::from)?;
    Ok(Json(ExampleDto::from(example)))
}
