pub mod orders;
pub mod products;
pub mod users;

/// Pagination parameters for repository queries.
#[derive(Debug, Clone)]
pub struct Pagination {
    /// Page number (1-indexed)
    pub page: u32,
    /// Items per page (max 100)
    pub page_size: u32,
}

impl Pagination {
    pub fn skip(&self) -> u64 {
        ((self.page.saturating_sub(1)) * self.page_size) as u64
    }

    pub fn limit(&self) -> i64 {
        self.page_size.min(100) as i64
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}
