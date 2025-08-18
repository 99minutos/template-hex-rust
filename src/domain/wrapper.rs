use crate::domain::DomainError;

pub type DomainWrapper<T> = Result<T, DomainError>;

impl From<std::io::Error> for DomainError {
    fn from(err: std::io::Error) -> Self {
        DomainError::Transient(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for DomainError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        DomainError::Unknown(err.to_string())
    }
}

impl From<&str> for DomainError {
    fn from(s: &str) -> Self {
        DomainError::Unknown(s.to_string())
    }
}

impl From<String> for DomainError {
    fn from(s: String) -> Self {
        DomainError::Unknown(s)
    }
}
