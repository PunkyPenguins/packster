use std::{io::{self, Read}, fmt};

use crate::{Result, essential::port::IDigester};
use sha2::{ Sha256, Digest };

pub enum Digester {
    Sha256
}

impl Default for Digester {
    fn default() -> Self {
        Digester::Sha256
    }
}

impl IDigester for Digester {
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

impl fmt::Display for Digester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sha256 => write!(f, "sha256")
        }
    }
}