use std::sync::Arc;

use crate::domain::ports::PortHealthRepo;

#[derive(Debug, Clone)]
pub struct HealthService {
    health_repo: Arc<dyn PortHealthRepo>,
}

impl HealthService {
    /// Crea una nueva instancia del servicio de salud.
    pub fn new(health_repo: Arc<dyn PortHealthRepo>) -> Self {
        Self { health_repo }
    }

    /// Ejecuta el chequeo de salud de los componentes de infraestructura.
    #[tracing::instrument(skip_all)]
    pub async fn check(&self) -> bool {
        self.health_repo.check().await
    }
}
