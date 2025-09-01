use serde::Serialize;

use super::OutputDto;

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub total: u64,
    pub data: Vec<T>,
}
impl<T> OutputDto for PaginatedResponse<T> {}
