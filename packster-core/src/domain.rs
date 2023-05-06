use std::{ path::Path, fmt };


use serde::{Deserialize, Serialize};

use crate::PACKAGE_EXTENSION;

#[derive(Serialize, Deserialize)]
pub struct Identifier(String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize)]
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

    pub fn to_checksum(&self) -> String {
        hex::encode(&self.digest)
    }

    pub fn to_file_name(&self) -> String {
        format!(
            "{}_{}_{}.{}.{}",
            self.identifier,
            self.version,
            hex::encode(&self.digest),
            hex::encode(self.packster_version.as_bytes()),
            PACKAGE_EXTENSION
        )
    }

    pub fn from_path(path: &Path) -> Self {
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let mut parts = filename.split('_');

        let identifier = parts.next().unwrap();
        let version = parts.next().unwrap();
        let checksum = parts.next().unwrap();
        let packster_version = parts.next().unwrap();

        Package {
            identifier: Identifier(identifier.to_owned()),
            version: Version(version.to_owned()),
            digest: hex::decode(checksum).unwrap(),
            packster_version: Version(packster_version.to_owned())
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Deployment {
    checksum: String
}

impl Deployment {
    pub fn new( checksum: String) -> Self {
        Deployment { checksum }
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_checksum_from_path() {
        let path = Path::new("C:\\Downloads\\static-package-a_0.0.1_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad_f61f10025ad.packster");
        let checksum = Package::from_path(path).to_checksum();

        assert_eq!(checksum, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }
}