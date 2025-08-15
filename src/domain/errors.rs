#![allow(dead_code)]
//! Domain-level error type and result alias for clean error handling.
//!
//! Goals:
//! - Keep the domain layer decoupled from infrastructure/framework errors.
//! - Provide a small, expressive set of error variants that represent domain/application intent.
//! - Offer a convenient `DomainResult<T>` alias to be used across domain and application services.
//!
//! Usage example in a domain port or service:
//!
//! ```ignore
//! use crate::domain::errors::{DomainError, DomainResult};
//!
//! pub async fn do_something() -> DomainResult<()> {
//!     // ...
//!     Err(DomainError::Validation("invalid input".into()))
//! }
//! ```
//!
//! Mapping to transport-specific errors (e.g., HTTP) should be done at the boundary
//! (e.g., handlers/adapters), not inside the domain.

use core::fmt;

/// A domain-level error representing intent (not technology) oriented failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// The requested resource was not found.
    NotFound(String),

    /// A conflicting state exists (e.g. unique constraint, version conflict).
    Conflict(String),

    /// The input or state is invalid from a domain perspective.
    Validation(String),

    /// A temporary failure that may succeed if retried (network hiccup, timeout).
    Transient(String),

    /// Any other unexpected or unclassified error.
    Unknown(String),
}

impl DomainError {
    /// Construct a NotFound error with a message.
    pub fn not_found<M: Into<String>>(msg: M) -> Self {
        Self::NotFound(msg.into())
    }

    /// Construct a Conflict error with a message.
    pub fn conflict<M: Into<String>>(msg: M) -> Self {
        Self::Conflict(msg.into())
    }

    /// Construct a Validation error with a message.
    pub fn validation<M: Into<String>>(msg: M) -> Self {
        Self::Validation(msg.into())
    }

    /// Construct a Transient error with a message.
    pub fn transient<M: Into<String>>(msg: M) -> Self {
        Self::Transient(msg.into())
    }

    /// Construct an Unknown error with a message.
    pub fn unknown<M: Into<String>>(msg: M) -> Self {
        Self::Unknown(msg.into())
    }

    /// Borrow the human-readable message for this error.
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

/// A convenient result alias for domain/application logic.
pub type DomainResult<T> = Result<T, DomainError>;

/// Generic conversions from common error types to `DomainError`.
/// Use these at the domain/application boundary as needed to keep the domain pure.
impl From<std::io::Error> for DomainError {
    fn from(err: std::io::Error) -> Self {
        // IO errors are commonly transient (timeouts, temporary unavailability).
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
