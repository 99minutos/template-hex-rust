use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE: u64 = 0;
const DEFAULT_LIMIT: u64 = 20;
const MAX_LIMIT: u64 = 100;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Pagination {
    #[validate(range(min = 0, message = "page must be >= 0"))]
    #[serde(default)]
    pub page: u64,

    #[validate(range(min = 1, max = 100, message = "limit must be between 1 and 100"))]
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_limit() -> u64 {
    DEFAULT_LIMIT
}

impl Pagination {
    pub fn new(page: u64, limit: u64) -> Self {
        Self {
            page,
            limit: limit.min(MAX_LIMIT),
        }
    }

    pub fn skip(&self) -> u64 {
        self.page * self.limit
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            limit: DEFAULT_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl<T> Paginated<T> {
    pub fn new(data: Vec<T>, total: u64, pagination: &Pagination) -> Self {
        let total_pages = if pagination.limit > 0 {
            (total as f64 / pagination.limit as f64).ceil() as u64
        } else {
            0
        };

        Self {
            data,
            total,
            page: pagination.page,
            limit: pagination.limit,
            total_pages,
        }
    }

    pub fn map<U, F>(self, f: F) -> Paginated<U>
    where
        F: FnMut(T) -> U,
    {
        Paginated {
            data: self.data.into_iter().map(f).collect(),
            total: self.total,
            page: self.page,
            limit: self.limit,
            total_pages: self.total_pages,
        }
    }
}
