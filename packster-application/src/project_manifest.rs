use serde::Deserialize;
use packster_core::{IProjectManifest, Result};

use toml;

use crate::Error;

#[derive(Deserialize)]
pub struct ProjectManifestRepresentation {
    identifier: String,
    version: String
}

impl IProjectManifest for ProjectManifestRepresentation {
    fn as_identifier(&self) -> &str {
        &self.identifier
    }

    fn as_version(&self) -> &str {
        &self.version
    }
}

// impl dyn IProjectManifest {
//     //This is not good because implicitely the domain rely on that function to be defined !!
//     //this could be handled by design by making business expect impl ProjectManifest directly
//     //Or by a more strict approach with generics and Self Types so the domain knows from port that he can parse

// }


pub fn parse<S: AsRef<str>>(s: S) -> Result<impl IProjectManifest> {
    let dto : ProjectManifestRepresentation = toml::from_str(s.as_ref()).map_err(Error::from)?;
    Ok(dto)
}