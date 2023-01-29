use std::path::Path;

use crate::{essential::port::{Digester, ReadOnlyFileSystem}, Result};

pub struct DirectoryDigester;

impl Default for DirectoryDigester {
    fn default() -> Self {
        todo!()
    }
}

impl Digester for DirectoryDigester {
    fn generate_checksum<F: ReadOnlyFileSystem, P: AsRef<Path>>(&self, filesystem: &F, file_path: P) -> Result<String> {
        todo!()
    }
}