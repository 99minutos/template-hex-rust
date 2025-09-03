use serde::Serialize;

use super::OutputResponse;

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub total: u64,
    pub data: Vec<T>,
}
impl<T> OutputResponse for PaginatedResponse<T> {}
