use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Entity Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid Identifier: {0}")]
    InvalidId(String),

    #[error("Database Error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Internal Error: {0}")]
    InternalError(String),
}
