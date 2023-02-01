mod compressor;
mod file_digester;

pub use compressor::Compressor;
pub use file_digester::FileDigester;

#[cfg(test)]
mod in_memory_filesystem;

#[cfg(test)]
pub use in_memory_filesystem::*;
