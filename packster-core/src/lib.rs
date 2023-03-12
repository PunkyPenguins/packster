#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(clippy::all)]

pub const PACKAGE_EXTENSION : &str = "packster";
pub const LOCKFILE_NAME : &str = "packster.lock";

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, error::Error>;

mod path;
pub use path::{ Absolute, RelativePath, NormalizedPathBuf };

mod domain;
pub use domain::{Identifier, Project};

mod port;
pub use port::*;

pub mod operation;