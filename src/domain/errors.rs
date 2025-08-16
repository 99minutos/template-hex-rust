#![allow(dead_code)]

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    NotFound(String),
    Conflict(String),
    Validation(String),
    Transient(String),
    Unknown(String),
}

impl DomainError {
    pub fn not_found<M: Into<String>>(msg: M) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn conflict<M: Into<String>>(msg: M) -> Self {
        Self::Conflict(msg.into())
    }

    pub fn validation<M: Into<String>>(msg: M) -> Self {
        Self::Validation(msg.into())
    }

    pub fn transient<M: Into<String>>(msg: M) -> Self {
        Self::Transient(msg.into())
    }

    pub fn unknown<M: Into<String>>(msg: M) -> Self {
        Self::Unknown(msg.into())
    }

    pub fn message(&self) -> &str {
        match self {
            Self::NotFound(m)
            | Self::Conflict(m)
            | Self::Validation(m)
            | Self::Transient(m)
            | Self::Unknown(m) => m.as_str(),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound(msg) => write!(f, "Not found: {msg}"),
            DomainError::Conflict(msg) => write!(f, "Conflict: {msg}"),
            DomainError::Validation(msg) => write!(f, "Validation: {msg}"),
            DomainError::Transient(msg) => write!(f, "Transient: {msg}"),
            DomainError::Unknown(msg) => write!(f, "Unknown: {msg}"),
        }
    }
}

impl std::error::Error for DomainError {}

pub type DomainResult<T> = Result<T, DomainError>;

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
