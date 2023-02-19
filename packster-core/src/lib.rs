#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(clippy::all)]

const PACKAGE_EXENSION : &str = "packster";

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, error::Error>;

mod path;
pub use path::NormalizedPath;

mod domain;
pub use domain::{Identifier, Project};

mod port;
pub use port::*;

pub mod operation;