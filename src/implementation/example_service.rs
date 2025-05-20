use std::sync::Arc;

use axum::http::StatusCode;

use crate::{
    domain::entities::Example,
    infrastructure::{http::HttpError, persistence::ExampleRepository},
};

#[derive(Debug, Clone)]
pub struct ExampleService {
    example_repo: Arc<ExampleRepository>,
}

impl ExampleService {
    pub fn new(example_repo: Arc<ExampleRepository>) -> Self {
        ExampleService { example_repo }
    }

    #[tracing::instrument]
    pub async fn get_examples(&self) -> Result<Vec<Example>, HttpError> {
        let examples = self
            .example_repo
            .all()
            .await
            .map_err(|e| HttpError::Custom(StatusCode::BAD_GATEWAY, e, None))?;

        Ok(examples)
    }

    #[tracing::instrument]
    pub async fn add_random_example(&self) -> Result<Example, HttpError> {
        let mut example = Example::default();
        example.name = format!("example-{}", rand::random::<u32>());
        self.example_repo
            .insert(&mut example)
            .await
            .map_err(|e| HttpError::Custom(StatusCode::BAD_GATEWAY, e.to_string(), None))?;
        Ok(example)
    }
}
