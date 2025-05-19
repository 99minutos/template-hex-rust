use std::sync::Arc;

use crate::infrastructure::{http::HttpError, persistence::ExampleRepository};

pub struct ExampleService {
    example_repo: Arc<ExampleRepository>,
}

impl ExampleService {
    pub fn new(example_repo: Arc<ExampleRepository>) -> Self {
        ExampleService { example_repo }
    }

    pub async fn get_examples(&self) -> Result<(), HttpError> {
        Ok(())
    }
}
