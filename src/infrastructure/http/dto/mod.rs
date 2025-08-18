#![allow(dead_code)]
pub mod example_dto;

pub trait OutputDto {}

pub trait InputDto {}

impl<T: OutputDto> OutputDto for Vec<T> {}

impl OutputDto for () {}
