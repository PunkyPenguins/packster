#![cfg_attr(all(not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(clippy::all)]

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, error::Error>;

pub mod application;
pub mod domain;
pub mod packaging;