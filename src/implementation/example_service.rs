use std::sync::Arc;

use axum::http::StatusCode;

use crate::{
    domain::{entities, ports},
    infrastructure::http::HttpError,
};

#[derive(Debug, Clone)]
pub struct ExampleService {
    example_repo: Arc<Box<dyn ports::PortExampleRepo>>,
}

impl ExampleService {
    pub fn new(example_repo: Box<dyn ports::PortExampleRepo>) -> Self {
        let example_repo = Arc::new(example_repo);
        ExampleService { example_repo }
    }

    #[tracing::instrument]
    pub async fn get_examples(&self) -> Result<Vec<entities::Example>, HttpError> {
        let examples = self
            .example_repo
            .all()
            .await
            .map_err(|e| HttpError::Custom(StatusCode::BAD_GATEWAY, e, None))?;

        Ok(examples)
    }

    #[tracing::instrument]
    pub async fn add_random_example(&self) -> Result<entities::Example, HttpError> {
        let mut example = entities::Example::default();
        example.name = format!("example-{}", rand::random::<u32>());

        let example = self
            .example_repo
            .insert(example)
            .await
            .map_err(|e| HttpError::Custom(StatusCode::BAD_GATEWAY, e.to_string(), None))?;
        Ok(example)
    }
}
