use std::{ path::Path, fmt, str::FromStr };
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use regex::Regex;
use hex;

use crate::{ Result, Error, PACKAGE_EXTENSION };

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier(String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Checksum(
    #[serde(with = "hex")]
    Vec<u8>
);

impl FromStr for Checksum {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(hex::decode(s).map(Checksum)?)
    }
}

impl ToString for Checksum {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl AsRef<[u8]> for Checksum {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Checksum {
    fn from(value: Vec<u8>) -> Self {
        Checksum(value)
    }
}
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

#[derive(Debug)]
pub struct Package {
    identifier: Identifier,
    version: Version,
    checksum: Checksum,
    packster_version: Version
}

impl Package {
    pub fn new(project: Project, checksum: Checksum, packster_version: Version) -> Self {
        Package {
            identifier: project.identifier,
            version: project.version,
            checksum,
            packster_version
        }
    }

    pub fn as_identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn as_checksum(&self) -> &Checksum {
        &self.checksum
    }

    pub fn to_file_name(&self) -> String {
        format!(
            "{}_{}_{}.{}.{}",
            self.identifier,
            self.version,
            hex::encode(&self.checksum),
            hex::encode(self.packster_version.as_bytes()),
            PACKAGE_EXTENSION
        )
    }

    //TODO test reciprocity with to_file_name
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self { //TODO handle error properly
        lazy_static! {
            static ref PACKAGE_FILENAME_REGEXP: Regex = Regex::new("(?P<identifier>[^_]+)_(?P<version>[^_]+)_(?P<checksum>[^.]+).(?P<packster_version>[^.]+)").unwrap();
        }
        let filename = path.as_ref().file_stem().unwrap().to_str().unwrap();
        let captures = PACKAGE_FILENAME_REGEXP.captures(filename).unwrap();

        let identifier = captures.name("identifier").unwrap().as_str();
        let version = captures.name("version").unwrap().as_str();
        let checksum = captures.name("checksum").unwrap().as_str();
        let packster_version = captures.name("packster_version").unwrap().as_str();

        Package {
            identifier: Identifier(identifier.to_owned()),
            version: Version(version.to_owned()),
            checksum: Checksum::from_str(checksum).unwrap(),
            packster_version: Version::new(packster_version) //TODO bug : has to be decoded and parsed to string => enforce semver through Version type
        }
    }
}

#[cfg(test)]
impl Default for Package {
    fn default() -> Self {
        Package {
            identifier: Identifier(String::from("my-package")),
            version: Version(String::from("0.0.1")),
            checksum: Checksum::from_str("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap(),
            packster_version: Version(String::from("0.1.4"))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Deployment {
    checksum: Checksum
}

impl Deployment {
    pub fn new( checksum: Checksum ) -> Self {
        Deployment { checksum }
    }

    pub fn as_checksum(&self) -> &Checksum {
        &self.checksum
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct DeployLocation {
    deployments: Vec<Deployment>
}

impl DeployLocation {
    pub fn as_slice(&self) -> &[Deployment] {
        &self.deployments
    }

    pub fn add_deployment(&mut self, deployment: Deployment) {
        self.deployments.push(deployment);
    }

    pub fn is_checksum_deployed(&self, checksum: &Checksum) -> bool {
        self.deployments.iter()
            .any(|deployment| deployment.as_checksum() == checksum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_checksum_from_path() {
        let path = Path::new("C:\\Downloads\\static-package-a_0.0.1_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.f61f10025ad.packster");
        let checksum = Package::from_path(path).as_checksum().to_string();

        assert_eq!(checksum, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }
}