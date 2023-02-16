#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(clippy::all)]

const PACKAGE_EXENSION : &str = "packster";

pub mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, error::Error>;

pub mod path;
pub mod entity;

pub mod port;
pub use port::*;

pub mod pack;
