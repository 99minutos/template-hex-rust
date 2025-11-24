use crate::implementation::{Example2Service, ExampleService};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub example_srv: ExampleService,
    pub example2_srv: Example2Service,
}
