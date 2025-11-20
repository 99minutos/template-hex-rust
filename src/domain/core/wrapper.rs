use crate::domain::core::ErrorKind;

use super::DomainError;

pub type DomainWrapper<T> = Result<T, DomainError>;

impl From<std::io::Error> for DomainError {
    fn from(err: std::io::Error) -> Self {
        DomainError::new(ErrorKind::Unknown, err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for DomainError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        DomainError::new(ErrorKind::Unknown, err.to_string())
    }
}

impl From<&str> for DomainError {
    fn from(s: &str) -> Self {
        DomainError::new(ErrorKind::Unknown, s.to_string())
    }
}

impl From<String> for DomainError {
    fn from(s: String) -> Self {
        DomainError::new(ErrorKind::Unknown, s)
    }
}
