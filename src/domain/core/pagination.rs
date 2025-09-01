use serde::{Deserialize, Serialize};
use valuable::Valuable;

#[derive(Debug, Serialize, Deserialize, Valuable)]
pub struct Pagination {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl Pagination {
    pub fn skip(&self) -> u64 {
        if self.page.is_none() || self.limit.is_none() {
            return 0;
        }
        let page = self.page.unwrap_or(0);
        let limit = self.limit.unwrap_or(10);

        page * limit
    }
    pub fn limit(&self) -> i64 {
        if self.limit.is_none() {
            return 10;
        }
        self.limit.unwrap_or(10) as i64
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page: Some(0),
            limit: Some(10),
        }
    }
}
