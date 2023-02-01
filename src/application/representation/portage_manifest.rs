use serde::Deserialize;
use crate::essential::port::PortageManifest;

use super::RepresentationResult;

use toml;

#[derive(Deserialize)]
pub struct PortageManifestRepresentation {
    identifier: String,
    version: String
}

impl PortageManifest for PortageManifestRepresentation {
    fn as_identifier(&self) -> &str {
        &self.identifier
    }

    fn as_version(&self) -> &str {
        &self.version
    }
}

impl dyn PortageManifest {
    //This is not good because implicitely the domain rely on that function to be defined !!
    //this could be handled by design by making business expect impl PortageManifest directly
    //Or by a more strict approache with generics and Self Types so the domain knows from port that he can parse
    pub fn parse<S: AsRef<str>>(s: S) -> RepresentationResult<impl PortageManifest> {
        let dto : PortageManifestRepresentation = toml::from_str(s.as_ref())?;
        Ok(dto)
    }
}

