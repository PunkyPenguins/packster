use std::io::{self, Read};
use sha2::{Digest, Sha256};
use packster_core::{application::port::Digester, domain::entity::Checksum};
use crate::{Error, Result};

#[derive(Default)]
pub enum Sha2Digester {
    #[default]
    Sha256,
}

impl Digester for Sha2Digester {
    fn generate_checksum<R: Read>(&self, mut reader: R) -> Result<Checksum> {
        match self {
            Self::Sha256 => {
                let mut hasher = Sha256::new();
                io::copy(&mut reader, &mut hasher).map_err(Error::from)?;
                Ok(Checksum::from(hasher.finalize().to_vec()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_generate_sha256() {
        let digester = Sha2Digester::Sha256;
        let checksum = digester
            .generate_checksum(
                "This is a long sentence that stands for binary content to be checked".as_bytes(),
            )
            .unwrap();
        assert_eq!(
            checksum,
            Checksum::from_str("564fef4556880e65e5ca00ae35bac4f07fa5f714ea31cc1119f6cdacbc14bcd8")
                .unwrap()
        );
    }
}
