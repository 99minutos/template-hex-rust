pub mod entities;
pub mod ports;
pub mod serializer;

mod error;
mod wrapper;
pub use error::DomainError;
pub use wrapper::DomainWrapper;
