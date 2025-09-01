use crate::domain::core::Pagination;
use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};

impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(params) = Query::<Pagination>::from_request_parts(parts, state)
            .await
            .map_err(|_| "Invalid query parameters")?;

        let page = params.page.unwrap_or(0);
        let limit = params.limit.unwrap_or(10);

        Ok(Pagination {
            page: Some(page),
            limit: Some(limit),
        })
    }
}
