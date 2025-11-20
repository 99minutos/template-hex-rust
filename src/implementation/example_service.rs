use std::sync::Arc;

use crate::domain::{self, entities, ports, DomainWrapper};

#[derive(Debug, Clone)]
pub struct ExampleService {
    example_repo: Arc<dyn ports::PortExampleRepo>,
}

impl ExampleService {
    pub fn new(example_repo: Arc<dyn ports::PortExampleRepo>) -> Self {
        ExampleService { example_repo }
    }

    #[tracing::instrument(skip_all)]
    pub async fn get_examples(&self) -> DomainWrapper<Vec<entities::Example>> {
        self.example_repo.all().await
    }

    #[tracing::instrument(skip_all)]
    pub async fn get_examples_with_error(&self) -> DomainWrapper<Vec<entities::Example>> {
        Err(domain::DomainError::new(
            domain::ErrorKind::Conflict,
            format!("custom error for example"),
        ))
    }

    #[tracing::instrument(skip_all)]
    pub async fn add_random_example(&self) -> DomainWrapper<entities::Example> {
        let mut example = entities::Example::default();
        example.name = format!("example-{}", rand::random::<u32>());
        self.example_repo.insert(example).await
    }
}
