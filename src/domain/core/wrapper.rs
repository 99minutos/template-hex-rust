use super::DomainError;

pub type DomainWrapper<T> = Result<T, DomainError>;

impl From<std::io::Error> for DomainError {
    fn from(err: std::io::Error) -> Self {
        DomainError::transient(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for DomainError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        DomainError::unknown(err.to_string())
    }
}

impl From<&str> for DomainError {
    fn from(s: &str) -> Self {
        DomainError::unknown(s.to_string())
    }
}

impl From<String> for DomainError {
    fn from(s: String) -> Self {
        DomainError::unknown(s)
    }
}

impl From<serde_json::Error> for DomainError {
    fn from(err: serde_json::Error) -> Self {
        DomainError::validation(err.to_string())
    }
}
