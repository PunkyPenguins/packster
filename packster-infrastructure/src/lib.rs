#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_variables)]

use packster_core;

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, packster_core::error::Error>;

mod tarball_archiver;
pub use tarball_archiver::TarballArchiver;

mod digester;
pub use digester::Digester;

mod std_filesystem;
pub use std_filesystem::StdFileSystem;


#[cfg(feature = "test")]
mod in_memory_filesystem;
#[cfg(feature = "test")]
pub use in_memory_filesystem::*;