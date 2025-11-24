#![allow(dead_code)]
pub mod example2_dto;
pub mod example_dto;
pub mod helpers_dto;

pub trait OutputResponse {}
impl<T: OutputResponse> OutputResponse for Vec<T> {}
impl OutputResponse for () {}
