#![allow(dead_code)]

use core::fmt;

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseKind {
    NotFound,
    Duplicate,
    Error,
}

impl fmt::Display for DatabaseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseKind::NotFound => write!(f, "register not found"),
            DatabaseKind::Duplicate => write!(f, "register duplicate"),
            DatabaseKind::Error => write!(f, "Database error"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
    Conflict,
    Validation,
    Database(DatabaseKind),
    Unauthorized,
    Unknown,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::NotFound => write!(f, "not found"),
            ErrorKind::Conflict => write!(f, "conflict"),
            ErrorKind::Validation => write!(f, "validation error"),
            ErrorKind::Database(kind) => write!(f, "database error: {}", kind),
            ErrorKind::Unauthorized => write!(f, "unauthorized"),
            ErrorKind::Unknown => write!(f, "unknown error"),
        }
    }
}

#[derive(Debug)]
pub struct DomainError {
    kind: ErrorKind,
    message: String,
    data: Option<serde_json::Value>,
}

impl DomainError {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_data<D: Serialize>(mut self, data: D) -> Self {
        self.data = serde_json::to_value(data).ok();
        self
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }

    pub fn is_not_found(&self) -> bool {
        self.kind == ErrorKind::NotFound
    }

    pub fn is_database_duplicate(&self) -> bool {
        self.kind == ErrorKind::Database(DatabaseKind::Duplicate)
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)?;
        if let Some(data) = &self.data {
            write!(f, " (Context: {})", data)?;
        }
        Ok(())
    }
}

impl std::error::Error for DomainError {}
