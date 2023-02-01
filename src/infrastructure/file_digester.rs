use std::{path::Path, io::{self, Read}, fmt};

use crate::{Result, essential::port::Digester};
use sha2::{ Sha256, Digest };

pub enum FileDigester {
    Sha256
}

impl Default for FileDigester {
    fn default() -> Self {
        FileDigester::Sha256
    }
}

impl Digester for FileDigester {
    fn generate_checksum<R: Read>(&self, mut reader: R) -> Result<Vec<u8>> {
        match self {
            Self::Sha256 => {
                let mut hasher = Sha256::new();
                io::copy(&mut reader, &mut hasher)?;
                Ok(hasher.finalize().to_vec())
            }
        }
    }
}

impl fmt::Display for FileDigester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sha256 => write!(f, "sha256")
        }
    }
}