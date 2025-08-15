use std::sync::Arc;

use crate::domain::{entities, ports, DomainResult};

#[derive(Debug, Clone)]
pub struct ExampleService {
    example_repo: Arc<Box<dyn ports::PortExampleRepo>>,
}

impl ExampleService {
    pub fn new(example_repo: Arc<Box<dyn ports::PortExampleRepo>>) -> Self {
        ExampleService { example_repo }
    }

    #[tracing::instrument]
    pub async fn get_examples(&self) -> DomainResult<Vec<entities::Example>> {
        self.example_repo.all().await
    }

    #[tracing::instrument]
    pub async fn add_random_example(&self) -> DomainResult<entities::Example> {
        let mut example = entities::Example::default();
        example.name = format!("example-{}", rand::random::<u32>());
        self.example_repo.insert(example).await
    }
}
