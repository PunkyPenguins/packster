#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod pack;
mod project_manifest;

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, packster_core::error::Error>;