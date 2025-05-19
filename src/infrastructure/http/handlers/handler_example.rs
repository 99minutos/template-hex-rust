use axum::{response, Json};

use crate::infrastructure::http::{dto, HttpError};

#[tracing::instrument]
pub async fn get_examples() -> Result<impl response::IntoResponse, HttpError> {
    let example = get_example().await?;
    tracing::info!("acá: {:?}", example);

    Ok(Json(example))
}

#[tracing::instrument]
pub async fn get_example() -> Result<dto::ExampleDto, HttpError> {
    let example = dto::ExampleDto {
        name: "example".to_string(),
    };

    tracing::info!("Aqui");

    Ok(example)
}
