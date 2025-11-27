use serde::Deserialize;
use validator::Validate;

use crate::domain::Pagination;
use crate::infrastructure::http::dto::InputRequest;

/// DTO de entrada para crear un nuevo ejemplo.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateExampleRequest {
    /// Nombre del ejemplo. Debe tener entre 3 y 100 caracteres.
    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters"
    ))]
    pub name: String,
}

impl InputRequest for CreateExampleRequest {}

/// Query parameters para listar ejemplos paginados.
#[derive(Debug, Deserialize, Validate)]
pub struct ListExamplesQuery {
    /// Número de página (inicia en 0).
    #[validate(range(min = 0))]
    #[serde(default)]
    pub page: u64,

    /// Cantidad de elementos por página (default 20, max 100).
    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_limit() -> u64 {
    20
}

impl From<ListExamplesQuery> for Pagination {
    fn from(q: ListExamplesQuery) -> Self {
        Pagination::new(q.page, q.limit)
    }
}
