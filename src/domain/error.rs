#![allow(dead_code)]

use core::fmt;

#[derive(Debug, Clone)]
pub enum DomainError {
    NotFound {
        message: String,
        data: Option<serde_json::Value>,
    },
    Conflict {
        message: String,
        data: Option<serde_json::Value>,
    },
    Validation {
        message: String,
        data: Option<serde_json::Value>,
    },
    Transient {
        message: String,
        data: Option<serde_json::Value>,
    },
    Unknown {
        message: String,
        data: Option<serde_json::Value>,
    },
}

impl DomainError {
    pub fn not_found<M: Into<String>>(msg: M) -> Self {
        Self::NotFound {
            message: msg.into(),
            data: None,
        }
    }

    pub fn not_found_with_data<M: Into<String>, D: serde::Serialize>(msg: M, data: D) -> Self {
        Self::NotFound {
            message: msg.into(),
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
        }
    }

    pub fn conflict<M: Into<String>>(msg: M) -> Self {
        Self::Conflict {
            message: msg.into(),
            data: None,
        }
    }

    pub fn conflict_with_data<M: Into<String>, D: serde::Serialize>(msg: M, data: D) -> Self {
        Self::Conflict {
            message: msg.into(),
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
        }
    }

    pub fn validation<M: Into<String>>(msg: M) -> Self {
        Self::Validation {
            message: msg.into(),
            data: None,
        }
    }

    pub fn validation_with_data<M: Into<String>, D: serde::Serialize>(msg: M, data: D) -> Self {
        Self::Validation {
            message: msg.into(),
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
        }
    }

    pub fn transient<M: Into<String>>(msg: M) -> Self {
        Self::Transient {
            message: msg.into(),
            data: None,
        }
    }

    pub fn transient_with_data<M: Into<String>, D: serde::Serialize>(msg: M, data: D) -> Self {
        Self::Transient {
            message: msg.into(),
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
        }
    }

    pub fn unknown<M: Into<String>>(msg: M) -> Self {
        Self::Unknown {
            message: msg.into(),
            data: None,
        }
    }

    pub fn unknown_with_data<M: Into<String>, D: serde::Serialize>(msg: M, data: D) -> Self {
        Self::Unknown {
            message: msg.into(),
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::NotFound { message, .. }
            | Self::Conflict { message, .. }
            | Self::Validation { message, .. }
            | Self::Transient { message, .. }
            | Self::Unknown { message, .. } => message.as_str(),
        }
    }

    pub fn data(&self) -> Option<&serde_json::Value> {
        match self {
            Self::NotFound { data, .. }
            | Self::Conflict { data, .. }
            | Self::Validation { data, .. }
            | Self::Transient { data, .. }
            | Self::Unknown { data, .. } => data.as_ref(),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound { message, .. } => write!(f, "Not found: {message}"),
            DomainError::Conflict { message, .. } => write!(f, "Conflict: {message}"),
            DomainError::Validation { message, .. } => write!(f, "Validation: {message}"),
            DomainError::Transient { message, .. } => write!(f, "Transient: {message}"),
            DomainError::Unknown { message, .. } => write!(f, "Unknown: {message}"),
        }
    }
}

impl std::error::Error for DomainError {}
