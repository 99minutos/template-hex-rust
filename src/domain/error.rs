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
