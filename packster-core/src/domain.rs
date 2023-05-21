use std::{ path::Path, fmt, str::FromStr };
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use regex::Regex;
use hex;

use crate::{ Result, Error, PACKAGE_EXTENSION };

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Identifier(String);

impl FromStr for Identifier {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Identifier(s.to_string())) //TODO proper identifier validation
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Version(s.to_string())) //TODO enforce semver through Version type ( from_str )
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub fn as_identifier(&self) -> &Identifier { &self.identifier }
    pub fn as_checksum(&self) -> &Checksum { &self.checksum }
    pub fn as_version(&self) -> &Version { &self.version }
    pub fn as_packster_version(&self) -> &Version { &self.packster_version }

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        lazy_static! {
            static ref PACKAGE_FILENAME_REGEXP: Regex = Regex::new("(?P<identifier>[^_]+)_(?P<version>[^_]+)_(?P<checksum>[^.]+).(?P<packster_version>[^.]+)").unwrap();
        }
        let path = path.as_ref();
        let filename = path.file_stem()
            .ok_or_else(|| Error::NoFileNameInPath(path.to_path_buf()))?
            .to_str()
            .ok_or_else(|| Error::InvalidUtf8Path(path.to_path_buf()))
        ?;

        let captures = PACKAGE_FILENAME_REGEXP.captures(filename)
            .ok_or_else(|| Error::WrongFileNameFormat("No match".into(), path.to_path_buf()))
        ?;

        let identifier = captures.name("identifier")
            .ok_or_else(|| Error::WrongFileNameFormat("Identifier missing".into(), path.to_path_buf()))
            .map(|m| m.as_str())
            .and_then(Identifier::from_str)
        ?;

        let version = captures.name("version")
            .ok_or_else(|| Error::WrongFileNameFormat("Version missing".into(), path.to_path_buf()))
            .map(|m| m.as_str())
            .and_then(Version::from_str)
        ?;

        let checksum = captures.name("checksum")
            .ok_or_else(|| Error::WrongFileNameFormat("Checksum missing".into(), path.to_path_buf()))
            .map(|m| m.as_str())
            .and_then(Checksum::from_str)
        ?;

        let packster_version = captures.name("packster_version")
            .ok_or_else(|| Error::WrongFileNameFormat("Packster version missing".into(), path.to_path_buf()))
            .map(|m| m.as_str())
            .map(|s| {
                println!("S {s}");
                s
            })
            .and_then(|s| hex::decode(s).map_err(Error::from))
            .and_then(|b| String::from_utf8(b).map_err(Error::from))
            .and_then(|s| Version::from_str(&s))
        ?;

        Ok(
            Package {
                identifier,
                version,
                checksum,
                packster_version
            }
        )
    }
}

#[cfg(test)]
impl Default for Package {
    fn default() -> Self {
        Package {
            identifier: Identifier(String::from("my-package")),
            version: Version(String::from("0.0.1")),
            checksum: Checksum::from_str("d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4").unwrap(),
            packster_version: Version(String::from("0.1.4"))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Deployment {
    #[serde(flatten)]
    package: Package
    //TODO installed packster version
}

impl Deployment {
    pub fn new( package: Package ) -> Self { Deployment { package } }

    pub fn as_checksum(&self) -> &Checksum { self.package.as_checksum() }
}

impl AsRef<Package> for Deployment {
    fn as_ref(&self) -> &Package { &self.package }
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

    pub fn remove_deployment(&mut self, checksum: &Checksum) {
        self.deployments
            .retain(|deployment| deployment.as_checksum() != checksum)
    }

    pub fn get_deployment(&self, checksum: &Checksum) -> Option<&Deployment> {
        self.deployments.iter()
            .find(|deployment| deployment.as_checksum() == checksum)
    }

    pub fn is_checksum_deployed(&self, checksum: &Checksum) -> bool {
        self.deployments.iter()
            .any(|deployment| deployment.as_checksum() == checksum)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Deployment> {
        self.deployments.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    pub use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_extract_checksum_from_path() -> Result<()> {
        let path = Path::new("C:\\Downloads\\static-package-a_0.0.1_d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4.302e312e30.packster");
        let checksum = Package::from_path(path)?.as_checksum().to_string();

        assert_eq!(checksum, "d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4");
        Ok(())
    }

    #[test]
    fn test_filename_reciprocity() -> Result<()> {
        let original_package = Package::default();
        let file_name = original_package.to_file_name();
        let parsed_package = Package::from_path(file_name)?;

        assert_eq!(original_package.as_identifier(), parsed_package.as_identifier());
        assert_eq!(original_package.as_checksum(), parsed_package.as_checksum());
        assert_eq!(original_package.as_version(), parsed_package.as_version());
        assert_eq!(original_package.as_packster_version(), parsed_package.as_packster_version());

        Ok(())
    }
}