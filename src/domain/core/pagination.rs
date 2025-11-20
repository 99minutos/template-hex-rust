#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use valuable::Valuable;

#[derive(Debug, Serialize, Deserialize, Valuable)]
pub struct Pagination {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl Pagination {
    pub fn skip(&self) -> u64 {
        if self.page.is_none() || self.limit.is_none() {
            return 0;
        }
        let page = self.page.unwrap_or(0);
        let limit = self.limit.unwrap_or(10);

        (page * limit) as u64
    }

    pub fn limit(&self) -> i64 {
        if self.limit.is_none() {
            return 10;
        }
        self.limit.unwrap_or(10)
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
