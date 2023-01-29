use serde::Deserialize;
use crate::business::port::PortageManifest;

use super::DtoResult;

use toml;

#[derive(Deserialize)]
pub struct PortageManifestDto {
    identifier: String,
    version: String
}

impl PortageManifest for PortageManifestDto {
    fn as_identifier(&self) -> &str {
        &self.identifier
    }

    fn as_version(&self) -> &str {
        &self.version
    }
}

impl dyn PortageManifest {
    pub fn parse<S: AsRef<str>>(s: S) -> DtoResult<impl PortageManifest> {
        let dto : PortageManifestDto = toml::from_str(s.as_ref())?;
        Ok(dto)
    }
}