use std::{io::{self, Read}, fmt};
use sha2::{Sha256, Digest};
use packster_core::{Digester};
use crate::{ Result, Error };

//TODO add some logging and integration tests
pub enum Sha2Digester {
    Sha256
}

impl Default for Sha2Digester {
    fn default() -> Self {
        Sha2Digester::Sha256
    }
}

impl Digester for Sha2Digester {
    fn generate_checksum<R: Read>(&self, mut reader: R) -> Result<Vec<u8>> {
        match self {
            Self::Sha256 => {
                let mut hasher = Sha256::new();
                io::copy(&mut reader, &mut hasher).map_err(Error::from)?;
                Ok(hasher.finalize().to_vec())
            }
        }
    }
}

impl fmt::Display for Sha2Digester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sha256 => write!(f, "sha256")
        }
    }
}