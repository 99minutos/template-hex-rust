pub mod example;
pub mod example2;
pub mod helpers_dto;

pub use helpers_dto::*;

/// Trait marcador para DTOs de entrada (Requests).
pub trait InputRequest {}

/// Trait marcador para DTOs de salida (Responses).
pub trait OutputResponse {}
