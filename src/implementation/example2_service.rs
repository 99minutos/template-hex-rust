use std::sync::Arc;

use crate::domain::{self, entities, ports, DomainWrapper};

#[derive(Debug, Clone)]
pub struct Example2Service {
    example2_repo: Arc<dyn ports::PortExample2Repo>,
}

impl Example2Service {
    /// Crea una nueva instancia del servicio.
    pub fn new(example2_repo: Arc<dyn ports::PortExample2Repo>) -> Self {
        Example2Service { example2_repo }
    }

    /// Obtiene todas las entidades Example2.
    #[tracing::instrument(skip_all)]
    pub async fn get_example2s(&self) -> DomainWrapper<Vec<entities::Example2>> {
        self.example2_repo.all().await
    }

    /// Retorna un error simulado.
    #[tracing::instrument(skip_all)]
    pub async fn get_example2s_with_error(&self) -> DomainWrapper<Vec<entities::Example2>> {
        Err(domain::DomainError::new(
            domain::ErrorKind::Conflict,
            format!("custom error for example2"),
        ))
    }

    /// Crea un Example2 aleatorio.
    #[tracing::instrument(skip_all)]
    pub async fn add_random_example2(&self) -> DomainWrapper<entities::Example2> {
        let mut example2 = entities::Example2::default();
        example2.name = format!("example2-{}", rand::random::<u32>());
        self.example2_repo.insert(example2).await
    }
}
