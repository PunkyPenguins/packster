use std::fmt;

// use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{PACKAGE_EXTENSION, AbsolutePath};

#[derive(Deserialize)]
// #[serde(try_from = "String")]
pub struct Identifier(String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Deserialize)]
pub struct Version(String);

impl Version {
    pub fn new<S: AsRef<str>>(version_str: S) -> Self {
        Version(version_str.as_ref().to_owned())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

//deserialize_with ?

//https://serde.rs/field-attrs.html#deserialize_with
#[derive(Deserialize)]
pub struct Project {
    identifier: Identifier,
    version: Version
}

impl Project {
    pub fn as_identifier(&self) -> &str {
        &self.identifier.0
    }

    pub fn as_version(&self) -> &str {
        &self.version.0
    }
}

pub struct Package {
    identifier: Identifier,
    version: Version,
    digest: Vec<u8>,
    packster_version: Version
}

impl Package {
    pub fn new(project: Project, digest: Vec<u8>, packster_version: Version) -> Self {
        Package {
            identifier: project.identifier,
            version: project.version,
            digest,
            packster_version
        }
    }

    pub fn file_name(&self) -> String {
        format!(
            "{}_{}_{}.{}.{}",
            self.identifier,
            self.version,
            hex::encode(&self.digest),
            hex::encode(self.packster_version.as_bytes()),
            PACKAGE_EXTENSION
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Deployment {}

#[derive(Serialize, Deserialize, Default)]
pub struct DeployLocation {
    deployments: Vec<Deployment>
}