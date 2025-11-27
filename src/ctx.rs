use crate::implementation::{Example2Service, ExampleService, HealthService};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub example_srv: ExampleService,
    pub example2_srv: Example2Service,
    pub health_srv: HealthService,
}
