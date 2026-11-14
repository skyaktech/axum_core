#![doc = include_str!("../README.md")]

pub mod errors;
pub mod response;

pub use errors::ApiError;
pub use response::{created, error, no_content, success, ApiResponse, ApiSuccess};
