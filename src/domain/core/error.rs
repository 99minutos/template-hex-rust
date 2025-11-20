#![allow(dead_code)]

use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum DatabaseKind {
    #[error("register not found")]
    NotFound,
    #[error("register duplicate")]
    Duplicate,
    #[error("Database error")]
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ErrorKind {
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("validation error")]
    Validation,
    #[error("database error: {0}")]
    Database(#[from] DatabaseKind),
    #[error("unauthorized")]
    Unauthorized,
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug, Error)]
#[error("{kind}: {message}")]
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
