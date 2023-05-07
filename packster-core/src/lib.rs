#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(clippy::all)]

pub const PACKAGE_EXTENSION : &str = "packster";
pub const LOCKFILE_NAME : &str = "packster.lock";
pub const PROJECT_MANIFEST_NAME : &str = "packster.toml";

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, error::Error>;

pub mod path;
pub mod port;
pub mod operation;
pub mod domain;