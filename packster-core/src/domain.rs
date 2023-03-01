use std::fmt;

// use semver::Version;
use serde::Deserialize;

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

const PACKAGE_EXTENSION : &str = "packster";

pub struct Package {
    identifier: Identifier,
    version: Version,
    digester_representation: String, //TODO enum instead ?
    digest: Vec<u8>,
    archiver_representation: String //TODO enum instead ?
}

impl Package {
    pub fn new(project: Project, digester_representation: String, archiver_representation: String, digest: Vec<u8>) -> Self {
        Package {
            identifier: project.identifier,
            version: project.version,
            digester_representation,
            digest,
            archiver_representation
        }
    }

    pub fn file_name(&self) -> String {
        format!(
            "{}_{}_{}_{}.{}.{}",
            self.identifier,
            self.version,
            self.digester_representation,
            hex::encode(&self.digest),
            self.archiver_representation,
            PACKAGE_EXTENSION
        )
    }
}