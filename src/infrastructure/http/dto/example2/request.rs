use serde::Deserialize;
use validator::Validate;

use crate::infrastructure::http::dto::InputRequest;

/// DTO de entrada para crear un nuevo Example2.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateExample2Request {
    /// Nombre de la entidad.
    #[validate(length(min = 3, message = "El nombre debe tener al menos 3 caracteres"))]
    pub name: String,
}

impl InputRequest for CreateExample2Request {}
