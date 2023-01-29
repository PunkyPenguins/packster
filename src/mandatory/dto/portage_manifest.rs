use serde::Deserialize;
use crate::essential::{
    port::{ PackageManifestContract, PackageManifest },
    port::DtoParser
};

use super::DtoResult;

use toml;

#[derive(Deserialize)]
pub struct PortageManifestDto {
    identifier: String,
    version: String
}

impl PackageManifestContract for PortageManifestDto {
    fn as_identifier(&self) -> &str {
        &self.identifier
    }

    fn as_version(&self) -> &str {
        &self.version
    }
}

impl DtoParser<PackageManifest> {
    pub fn parse<S: AsRef<str>>(s: S) -> DtoResult<impl PackageManifestContract> {
        let dto : PortageManifestDto = toml::from_str(s.as_ref())?;
        Ok(dto)
    }
}


