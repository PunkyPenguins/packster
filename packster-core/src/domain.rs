// use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
// #[serde(try_from = "String")]
pub struct Identifier(String);

#[derive(Deserialize)]
pub struct Version(String);

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

//TODO rework & extend according to type design