use crate::implementation::ExampleService;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub example_srv: ExampleService,
}
