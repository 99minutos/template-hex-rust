#![allow(dead_code)]
pub mod example_dto;
pub mod helpers_dto;

pub trait InputDto {}

pub trait OutputDto {}
impl<T: OutputDto> OutputDto for Vec<T> {}
impl OutputDto for () {}
