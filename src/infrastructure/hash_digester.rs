use std::{path::Path, io};

use crate::{Result, business::port::{ReadOnlyFileSystem, Digester}};
use sha2::{ Sha256, Digest };

pub enum HashDigester {
    Sha256
}

impl HashDigester {
    fn hash_file<F: ReadOnlyFileSystem, P: AsRef<Path>>(&self, filesystem: &F, file_path: P) -> Result<Vec<u8>> {
        let mut reader = filesystem.open_read(file_path)?;
        match self {
            Self::Sha256 => {
                let mut hasher = Sha256::new();
                io::copy(&mut reader, &mut hasher)?;
                Ok(hasher.finalize().to_vec())
            }
        }
    }
}

impl Default for HashDigester {
    fn default() -> Self {
        HashDigester::Sha256
    }
}

impl Digester for HashDigester {
    fn generate_checksum<F: ReadOnlyFileSystem, P: AsRef<Path>>(&self, filesystem: &F, file_path: P) -> Result<Vec<u8>> {
        if ! filesystem.is_directory(file_path.as_ref()) {
            //filesystem.walk_dir()
            todo!()
        } else {
            self.hash_file(filesystem, file_path)
        }
    }
}

impl ToString for HashDigester {
    fn to_string(&self) -> String {
        match self {
            Self::Sha256 => String::from("sha256")
        }
    }
}