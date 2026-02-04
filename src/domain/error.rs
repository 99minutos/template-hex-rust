use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("{entity} not found: {id}")]
    NotFound { entity: &'static str, id: String },

    #[error("{entity} already exists: {details}")]
    AlreadyExists {
        entity: &'static str,
        details: String,
    },

    #[error("Invalid {field}: {reason}")]
    Invalid { field: &'static str, reason: String },

    #[error("{field} is required")]
    Required { field: &'static str },

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Business rule violated: {0}")]
    BusinessRule(String),

    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },

    #[error("Database error: {0}")]
    Database(#[from] mongodb::error::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

// Helpers genéricos para crear errores comunes
impl DomainError {
    // ===== Búsqueda y existencia =====

    pub fn not_found(entity: &'static str, id: impl Into<String>) -> Self {
        Self::NotFound {
            entity,
            id: id.into(),
        }
    }

    pub fn already_exists(entity: &'static str, details: impl Into<String>) -> Self {
        Self::AlreadyExists {
            entity,
            details: details.into(),
        }
    }

    // ===== Validaciones =====

    pub fn invalid(field: &'static str, reason: impl Into<String>) -> Self {
        Self::Invalid {
            field,
            reason: reason.into(),
        }
    }

    pub fn required(field: &'static str) -> Self {
        Self::Required { field }
    }

    pub fn invalid_param(
        param: &'static str,
        entity: &'static str,
        value: impl Into<String>,
    ) -> Self {
        Self::Invalid {
            field: param,
            reason: format!("Invalid {} ID: {}", entity, value.into()),
        }
    }

    pub fn invalid_email(email: impl Into<String>) -> Self {
        Self::Invalid {
            field: "email",
            reason: format!("Invalid email format: {}", email.into()),
        }
    }

    pub fn invalid_length(field: &'static str, min: usize, max: usize) -> Self {
        Self::Invalid {
            field,
            reason: format!("Length must be between {} and {}", min, max),
        }
    }

    pub fn invalid_range<T: std::fmt::Display>(field: &'static str, min: T, max: T) -> Self {
        Self::Invalid {
            field,
            reason: format!("Value must be between {} and {}", min, max),
        }
    }

    // ===== Autorización =====

    pub fn unauthorized(reason: impl Into<String>) -> Self {
        Self::Unauthorized(reason.into())
    }

    pub fn forbidden(reason: impl Into<String>) -> Self {
        Self::Forbidden(reason.into())
    }

    pub fn missing_token() -> Self {
        Self::Unauthorized("Authentication token required".into())
    }

    pub fn invalid_token() -> Self {
        Self::Unauthorized("Invalid or expired token".into())
    }

    pub fn insufficient_permissions(action: impl Into<String>) -> Self {
        Self::Forbidden(format!("Insufficient permissions to {}", action.into()))
    }

    // ===== Reglas de negocio =====

    pub fn business_rule(message: impl Into<String>) -> Self {
        Self::BusinessRule(message.into())
    }

    pub fn duplicate(entity: &'static str, field: &'static str, value: impl Into<String>) -> Self {
        Self::AlreadyExists {
            entity,
            details: format!("{} '{}' already in use", field, value.into()),
        }
    }

    pub fn operation_not_allowed(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::BusinessRule(format!("{}: {}", operation.into(), reason.into()))
    }

    // ===== Errores internos =====

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    // ===== Servicios externos =====

    pub fn external(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalService {
            service: service.into(),
            message: message.into(),
        }
    }

    pub fn external_timeout(service: impl Into<String>) -> Self {
        Self::ExternalService {
            service: service.into(),
            message: "Request timeout".into(),
        }
    }

    // ===== Conversiones y parsing =====

    pub fn parse_error(field: &'static str, value: impl Into<String>) -> Self {
        Self::Invalid {
            field,
            reason: format!("Cannot parse value: {}", value.into()),
        }
    }

    pub fn conversion_error(from: &str, to: &str) -> Self {
        Self::Internal(format!("Failed to convert from {} to {}", from, to))
    }
}

// Re-export como "Error" para conveniencia
pub type Error = DomainError;

// Alias para Result con nuestro error
pub type Result<T> = std::result::Result<T, DomainError>;
