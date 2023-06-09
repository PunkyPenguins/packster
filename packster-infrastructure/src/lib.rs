#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![allow(dead_code)]

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, packster_core::Error>;

mod tarball_archiver;
pub use tarball_archiver::TarballArchiver;

mod sha2_digester;
pub use sha2_digester::Sha2Digester;

mod std_filesystem;
pub use std_filesystem::StdFileSystem;

mod uniqid_identifier_generator;
pub use uniqid_identifier_generator::UniqidIdentifierGenerator;

mod toml;
pub use crate::toml::Toml;

mod json;
pub use crate::json::Json;


#[cfg(feature = "test")]
mod in_memory_filesystem;
#[cfg(feature = "test")]
pub use in_memory_filesystem::*;